use anyhow::Result;
use printpdf::*;
use std::fs;

fn load_font(doc: &mut PdfDocument, path: &str, warns: &mut Vec<PdfWarnMsg>) -> FontId {
    let bytes = fs::read(path).expect("font read failed");
    let parsed = ParsedFont::from_bytes(&bytes, 0, warns).expect("font parse failed"); // from_bytes
    doc.add_font(&parsed) // add_font
}

fn main() -> Result<()> {
    // ドキュメント作成
    let mut doc = PdfDocument::new("Source Han Code JP styles");

    // フォント読み込み（同名はお手元のファイル名に合わせてください）
    let mut font_warnings = Vec::new();
    let font_regular   = load_font(&mut doc, "fonts/SourceHanCodeJP-Regular.otf",   &mut font_warnings);
    let font_bold      = load_font(&mut doc, "fonts/SourceHanCodeJP-Bold.otf",      &mut font_warnings);
    let font_regularit = load_font(&mut doc, "fonts/SourceHanCodeJP-RegularIt.otf", &mut font_warnings);
    let font_boldit    = load_font(&mut doc, "fonts/SourceHanCodeJP-BoldIt.otf",    &mut font_warnings);

    // A4 縦 1 ページにテキストだけを配置（原点は左下）
    let start_pos = Point { x: Mm(20.0).into(), y: Mm(260.0).into() };

    let page_ops = vec![
        Op::StartTextSection,                                            // テキストセクション開始
        Op::SetTextCursor { pos: start_pos },                            // 開始位置
        Op::SetLineHeight { lh: Pt(24.0) },                              // 行送り

        // Regular
        Op::SetFontSize { size: Pt(18.0), font: font_regular.clone() },  // フォント＋サイズ
        Op::WriteText {
            items: vec![TextItem::Text("Regular：源ノ角ゴ Code JP（日本語サンプル）".to_string())],
            font: font_regular.clone(),
        },
        Op::AddLineBreak,

        // Bold
        Op::SetFontSize { size: Pt(18.0), font: font_bold.clone() },
        Op::WriteText {
            items: vec![TextItem::Text("Bold：太字のサンプルです。123 ABC".to_string())],
            font: font_bold.clone(),
        },
        Op::AddLineBreak,

        // RegularIt（イタリック）
        Op::SetFontSize { size: Pt(18.0), font: font_regularit.clone() },
        Op::WriteText {
            items: vec![TextItem::Text("RegularIt：イタリック（斜体）のサンプル。".to_string())],
            font: font_regularit.clone(),
        },
        Op::AddLineBreak,

        // BoldIt（ボールド＋イタリック）
        Op::SetFontSize { size: Pt(18.0), font: font_boldit.clone() },
        Op::WriteText {
            items: vec![TextItem::Text("BoldIt：太字＋斜体のサンプル。".to_string())],
            font: font_boldit.clone(),
        },
        // Op::EndTextSection はページ終端で自動的に挿入されます
    ];

    let page = PdfPage::new(Mm(210.0), Mm(297.0), page_ops);

    let save_opts = PdfSaveOptions { subset_fonts: true, ..Default::default() };
    let mut save_warnings = Vec::new();
    let pdf_bytes = doc.with_pages(vec![page]).save(&save_opts, &mut save_warnings);

    fs::write("output_fonts.pdf", &pdf_bytes)?;
    if !font_warnings.is_empty() || !save_warnings.is_empty() {
        eprintln!("Warnings: font={:?}, save={:?}", font_warnings, save_warnings);
    }
    println!("Wrote output_fonts.pdf");
    Ok(())
}
