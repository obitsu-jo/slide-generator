use anyhow::Result;
use printpdf::*;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FontStyle {
    Regular,
    Bold,
    RegularIt,
    BoldIt,
}

// DrawConfigの構造を変更。ページの物理的な高さ(Pt)を保持する
struct DrawConfig {
    page_height_pt: Pt,
    base_font_size: Pt,
    default_font_style: FontStyle,
    default_color: Color,
}

fn load_font(doc: &mut PdfDocument, path: &str, warns: &mut Vec<PdfWarnMsg>) -> FontId {
    let bytes: Vec<u8> = fs::read(path).expect("font read failed");
    let parsed: ParsedFont = ParsedFont::from_bytes(&bytes, 0, warns).expect("font parse failed");
    doc.add_font(&parsed)
}

/// グリッド座標系に基づいてページにテキストを追加する関数
///
/// # Arguments
/// * `x`, `y` - グリッドの座標 (列, 行)。ページの左上が (0, 0)。
/// * `size_ratio` - 基準フォントサイズからの倍率。1.0でグリッド1マス分の大きさ。
fn add_text(
    ops: &mut Vec<Op>,
    fonts: &HashMap<FontStyle, FontId>,
    config: &DrawConfig,
    text: &str,
    x: f32, // 単位: 文字数 (列)
    y: f32, // 単位: 文字数 (行)
    style: Option<FontStyle>,
    size_ratio: Option<f32>,
    color: Option<Color>,
) {
    let final_style = style.unwrap_or(config.default_font_style);
    let final_size_ratio = size_ratio.unwrap_or(1.0);
    let final_color = color.unwrap_or_else(|| config.default_color.clone());

    let font_id = fonts.get(&final_style).expect("Specified font style not loaded.");
    // 最終的なフォントサイズを計算
    let final_font_size = config.base_font_size * final_size_ratio;
    
    // --- 【重要】グリッド座標から物理座標(Pt)への変換 ---
    // 1文字の幅/高さ = base_font_size とする
    let base_unit_pt = config.base_font_size.0;

    // グリッド座標 (x, y) を物理的なPt座標 (x_pt, y_pt) に変換
    let x_pt: Pt = Pt(x * base_unit_pt);
    let y_pt_from_top: Pt = Pt(y * base_unit_pt);

    // PDFの左下原点座標系に変換し、さらにベースラインを調整
    let y_from_bottom_pt = config.page_height_pt - y_pt_from_top;
    let baseline_y: Pt = y_from_bottom_pt - final_font_size;
    
    let new_ops = vec![
        Op::StartTextSection,
        Op::SetFillColor { col: (final_color) },
        Op::SetTextCursor { pos: Point { x: x_pt.into(), y: baseline_y.into() } },
        Op::SetFontSize { size: final_font_size, font: font_id.clone() },
        Op::WriteText {
            items: vec![TextItem::Text(text.to_string())],
            font: font_id.clone(),
        },
        Op::EndTextSection,
    ];
    ops.extend(new_ops);
}

fn main() -> Result<()> {
    // --- グリッドシステムと基本単位の設定 ---
    // 1. すべての基準となるフォントサイズを定義 (これが1グリッドの大きさになる)
    let base_font_size_pt = Pt(24.0);

    // 2. ページの大きさを「文字数」で定義
    let grid_width = 32.0;  // 横に32文字分
    let grid_height = 18.0; // 縦に18文字分

    // 3. 上記設定から、ページの物理的な大きさ(Pt)を計算
    let page_width_pt = Pt(grid_width * base_font_size_pt.0);
    let page_height_pt = Pt(grid_height * base_font_size_pt.0);

    // --- 描画設定を初期化 ---
    let config = DrawConfig {
        page_height_pt, // 計算済みの物理的な高さを渡す
        base_font_size: base_font_size_pt,
        default_font_style: FontStyle::Regular,
        default_color: Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
    };
    
    // --- ドキュメントとフォントの準備 ---
    let mut doc: PdfDocument = PdfDocument::new("Grid-based Slide");
    let mut font_warnings: Vec<PdfWarnMsg> = Vec::new();

    let mut fonts: HashMap<FontStyle, FontId> = HashMap::new();
    fonts.insert(FontStyle::Regular, load_font(&mut doc, "fonts/RictyDiminished-Regular.ttf", &mut font_warnings));
    fonts.insert(FontStyle::Bold, load_font(&mut doc, "fonts/RictyDiminished-Bold.ttf", &mut font_warnings));

    // --- 描画処理 (グリッド座標で指定) ---
    let mut page_ops: Vec<Op> = Vec::new();

    for count in 0..(grid_height as u8)-2 {
        add_text(
            &mut page_ops, &fonts, &config,
            &format!("{}行目のテキスト", (count + 1) as u8),
            0.0, 1.0 * count as f32,
            None, None, None,
        );
    }
    add_text(
        &mut page_ops, &fonts, &config,
        &("あ".repeat(31) + "い"),
        0.0, 16.0,
        None, None, None,
    );
    add_text(
        &mut page_ops, &fonts, &config,
        &("a".repeat(63) + "i"),
        0.0, 17.0,
        None, None, None,
    );

    // --- PDFの生成と保存 ---
    // 計算済みの物理的な大きさでページを生成
    let page_width_mm: Mm = page_width_pt.into();
    let page_height_mm: Mm = page_height_pt.into();
    let page: PdfPage = PdfPage::new(page_width_mm, page_height_mm, page_ops);

    let save_opts: PdfSaveOptions = PdfSaveOptions { subset_fonts: true, ..Default::default() };
    let mut save_warnings: Vec<PdfWarnMsg> = Vec::new();
    let pdf_bytes: Vec<u8> = doc.with_pages(vec![page]).save(&save_opts, &mut save_warnings);

    fs::write("outputs/output_grid.pdf", &pdf_bytes)?;
    if !font_warnings.is_empty() {
        eprintln!("Warnings: font={:?}, save={:?}", font_warnings, save_warnings);
    }
    println!("Wrote output_grid.pdf");
    Ok(())
}