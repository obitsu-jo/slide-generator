use anyhow::Result;
use printpdf::*;
use std::collections::HashMap;
use std::fs;

// --- 型定義 (変更なし) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FontStyle { Regular, Bold }

#[derive(Debug, Clone, Copy)]
enum NamedColor { Black, White, Red, Green, Blue }

#[derive(Debug, Clone, Copy)]
enum SlideColor {
    Named(NamedColor),
    Custom(f32, f32, f32), // f32からf32に変更し、精度を統一
}

impl SlideColor {
    fn into_pdf_color(self) -> Color {
        match self {
            SlideColor::Named(named) => match named {
                NamedColor::Black => Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
                NamedColor::White => Color::Rgb(Rgb::new(1.0, 1.0, 1.0, None)),
                NamedColor::Red   => Color::Rgb(Rgb::new(0.8, 0.0, 0.0, None)),
                NamedColor::Green => Color::Rgb(Rgb::new(0.0, 0.8, 0.0, None)),
                NamedColor::Blue  => Color::Rgb(Rgb::new(0.0, 0.0, 0.8, None)),
            },
            SlideColor::Custom(r, g, b) => Color::Rgb(Rgb::new(r, g, b, None)),
        }
    }
}

struct DrawConfig {
    page_height_pt: Pt,
    base_font_size: Pt,
    default_font_style: FontStyle,
    default_color: SlideColor,
}

fn load_font(doc: &mut PdfDocument, path: &str, warns: &mut Vec<PdfWarnMsg>) -> FontId {
    let bytes: Vec<u8> = fs::read(path).expect("font read failed");
    let parsed: ParsedFont = ParsedFont::from_bytes(&bytes, 0, warns).expect("font parse failed");
    doc.add_font(&parsed)
}

// --- === 新しいアーキテクチャの導入 === ---

// 1. 中間表現: スタイル付きのテキスト断片
#[derive(Clone)]
struct TextSpan {
    text: String,
    style: FontStyle,
    size_ratio: f32,
    color: SlideColor,
}

// 2. 中間表現: テキスト断片か、改行のような制御命令かを表す
#[derive(Clone)]
enum Content {
    Span(TextSpan),
    Newline,
}

#[derive(Debug, Clone, Copy)]
pub enum VAlign {
    Top,
    Middle,
    Bottom, // ベースライン揃え
}

/// 【低レベル関数】単一のTextSpanを、指定された絶対グリッド座標に描画する
fn add_single_span(
    ops: &mut Vec<Op>,
    fonts: &HashMap<FontStyle, FontId>,
    config: &DrawConfig,
    span: &TextSpan,
    col: f32,
    row: f32,
) {
    let final_pdf_color = span.color.into_pdf_color();
    let font_id = fonts.get(&span.style).expect("Specified font style not loaded.");
    let final_font_size = config.base_font_size * span.size_ratio;
    
    let base_unit_pt = config.base_font_size.0;
    let x_pt = Pt(col * base_unit_pt);
    let y_pt_from_top = Pt(row * base_unit_pt);

    let y_from_bottom_pt = config.page_height_pt - y_pt_from_top;
    let baseline_y = y_from_bottom_pt - final_font_size;
    
    let new_ops = vec![
        Op::StartTextSection,
        Op::SetFillColor { col: (final_pdf_color) },
        Op::SetTextCursor { pos: Point { x: x_pt.into(), y: baseline_y.into() } },
        Op::SetFontSize { size: final_font_size, font: font_id.clone() },
        Op::WriteText { items: vec![TextItem::Text(span.text.clone())], font: font_id.clone() },
        Op::EndTextSection,
    ];
    ops.extend(new_ops);
}

/// 【高レベル関数】Contentのリストを受け取り、ブロックとしてレイアウトして描画する
fn draw_text_block(
    ops: &mut Vec<Op>,
    fonts: &HashMap<FontStyle, FontId>,
    config: &DrawConfig,
    contents: &[Content],
    start_col: f32,
    start_row: f32,
    line_spacing_ratio: f32,
    align: VAlign,
) {
    let mut current_row = start_row;
    let mut current_content_index = 0;

    // contentsがなくなるまで、一行ずつループ処理
    while current_content_index < contents.len() {
        // --- 1. 測定パス ---
        // 現在の行に含まれるSpanを収集し、最大のフォントサイズ比率を見つける
        let mut spans_in_line: Vec<&TextSpan> = Vec::new();
        let mut line_end_index = current_content_index;
        let mut max_font_size_ratio = 1.0;

        for i in current_content_index..contents.len() {
            match &contents[i] {
                Content::Span(span) => {
                    spans_in_line.push(span);
                    if span.size_ratio > max_font_size_ratio {
                        max_font_size_ratio = span.size_ratio;
                    }
                    line_end_index = i + 1;
                },
                Content::Newline => {
                    line_end_index = i + 1;
                    break; // 改行が見つかったらこの行はここまで
                },
            }
        }
        
        // --- 2. 描画パス ---
        // 収集したSpanを、配置モードに基づいて描画していく
        let mut current_col = start_col;
        for span in spans_in_line {
            // 配置モードに応じて、Y座標のオフセットを計算
            let y_offset = match align {
                // Top揃え: オフセットなし。spanの上端は行の上端に揃う。
                VAlign::Top => 0.0,
                // Middle揃え: 行の高さの中心と、spanの高さの中心を合わせる
                VAlign::Middle => (max_font_size_ratio - span.size_ratio) / 2.0,
                // Bottom(ベースライン)揃え: spanの上端を下にずらし、ベースラインを合わせる
                VAlign::Bottom => max_font_size_ratio - span.size_ratio,
            };
            
            // 調整後の行座標(row)で低レベル描画関数を呼び出す
            add_single_span(ops, fonts, config, span, current_col, current_row + y_offset);
            
            // 仮想カーソルを右に進める
            current_col += span.text.chars().count() as f32 * span.size_ratio;
        }

        // --- 仮想カーソルの更新 ---
        // 次の行の開始位置に移動
        current_row += max_font_size_ratio * line_spacing_ratio;
        // 処理済みのコンテンツをスキップ
        current_content_index = line_end_index;
    }
}


fn main() -> Result<()> {
    // --- グリッドシステムと基本単位の設定  ---
    let base_font_size_pt = Pt(24.0);
    let grid_width = 32.0;
    let grid_height = 18.0;
    let page_width_pt = Pt(grid_width * base_font_size_pt.0);
    let page_height_pt = Pt(grid_height * base_font_size_pt.0);

    let config = DrawConfig {
        page_height_pt,
        base_font_size: base_font_size_pt,
        default_font_style: FontStyle::Regular,
        default_color: SlideColor::Named(NamedColor::Black),
    };

    // --- ドキュメントとフォントの準備 ---
    let mut doc: PdfDocument = PdfDocument::new("Grid-based Slide");
    let mut font_warnings: Vec<PdfWarnMsg> = Vec::new();

    let mut fonts: HashMap<FontStyle, FontId> = HashMap::new();
    fonts.insert(FontStyle::Regular, load_font(&mut doc, "fonts/RictyDiminished-Regular.ttf", &mut font_warnings));
    fonts.insert(FontStyle::Bold, load_font(&mut doc, "fonts/RictyDiminished-Bold.ttf", &mut font_warnings));

    // --- 描画処理 ---
    let mut all_pages_ops: Vec<Vec<Op>> = Vec::new();

    // --- 1ページ目の作成と描画 ---
    all_pages_ops.push(Vec::new()); // 新しいページ (インデックス 0) を追加
    let current_page_index = 0;

    let page1_title = vec![
        Content::Span(TextSpan { text: "スライド 1".to_string(), style: FontStyle::Bold, size_ratio: 2.0, color: SlideColor::Named(NamedColor::Black) }),
    ];
    draw_text_block(&mut all_pages_ops[current_page_index], &fonts, &config, &page1_title, 2.0, 2.0, 1.2, VAlign::Bottom);
    
    let page1_body = vec![
        Content::Span(TextSpan { text: "これは最初のページです。".to_string(), style: FontStyle::Regular, size_ratio: 1.0, color: SlideColor::Named(NamedColor::Black) }),
        Content::Newline,
        Content::Span(TextSpan { text: "複数ページのPDFを作成できます。".to_string(), style: FontStyle::Regular, size_ratio: 1.0, color: SlideColor::Named(NamedColor::Black) }),
    ];
    draw_text_block(&mut all_pages_ops[current_page_index], &fonts, &config, &page1_body, 2.0, 5.0, 1.5, VAlign::Top);

    // --- 2ページ目の作成と描画 ---
    all_pages_ops.push(Vec::new());
    let current_page_index = 1;

    let page2_title = vec![
        Content::Span(TextSpan { text: "スライド 2".to_string(), style: FontStyle::Bold, size_ratio: 2.0, color: SlideColor::Named(NamedColor::Black) }),
    ];
    draw_text_block(&mut all_pages_ops[current_page_index], &fonts, &config, &page2_title, 2.0, 2.0, 1.2, VAlign::Bottom);

    let page2_body = vec![
        Content::Span(TextSpan { text: "これは2ページ目です。".to_string(), style: FontStyle::Regular, size_ratio: 1.0, color: SlideColor::Named(NamedColor::Black) }),
        Content::Newline,
        Content::Span(TextSpan { text: "複数ページのPDFを作成できます。".to_string(), style: FontStyle::Regular, size_ratio: 1.0, color: SlideColor::Named(NamedColor::Black) }),
    ];
    draw_text_block(&mut all_pages_ops[current_page_index], &fonts, &config, &page2_body, 2.0, 5.0, 1.5, VAlign::Top);


    // --- PDFの生成と保存 (変更なし) ---
    let page_width_mm: Mm = page_width_pt.into();
    let page_height_mm: Mm = page_height_pt.into();

    // 描画命令のリストをループ処理し、PdfPageのリストを作成
    let pdf_pages: Vec<PdfPage> = all_pages_ops.into_iter().map(|ops| {
        PdfPage::new(page_width_mm, page_height_mm, ops)
    }).collect();

    // 作成したページのリストをドキュメントに追加して保存
    let save_opts: PdfSaveOptions = PdfSaveOptions { subset_fonts: true, ..Default::default() };
    let mut save_warnings: Vec<PdfWarnMsg> = Vec::new();
    let pdf_bytes: Vec<u8> = doc.with_pages(pdf_pages).save(&save_opts, &mut save_warnings);

    fs::write("outputs/output_multipage.pdf", &pdf_bytes)?;
    if !font_warnings.is_empty() {
        eprintln!("Warnings: font={:?}, save={:?}", font_warnings, save_warnings);
    }
    println!("Wrote output_multipage.pdf");
    Ok(())
}