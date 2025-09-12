use printpdf::*;
use std::fs::File;

const PAGE_HEIGHT:f64 = 297.0;
const PAGE_WIDTH:f64 = 170.0;
const MARGIN_TOP:f64 = 277.0;
const MARGIN_BOTTOM:f64 = 20.0;

fn draw_wrapped_text (
    doc: &PdfDocumentReference,
    mut layer: PdfLayerReference,
    text: &str,
    font: &IndirectFontRef,
    font_size: f64,
    start_x: f64,
    mut current_y: f64,
    page_width: f64,
    line_height: f64,
) -> (f64,PdfLayerReference) {
    let mut current_x: f64 = start_x;

    for line in text.split('\n'){
        for word in line.split_whitespace() {
            let word_width: f64 = font.get_width(word,font_size as i64) as f64 / 1000.0 * font_size;
            
            // check if word fits in current line
            if current_x + word_width > start_x + page_width  {
                current_x = start_x;
                current_y -= line_height;
            }

            //page break
            if current_y < MARGIN_BOTTOM {
                let (page, new_layer_id) = doc.add_page(Mm(210.0), Mm(PAGE_HEIGHT), "Layer 1");
                layer = doc.get_page(page).get_layer(new_layer_id);
                current_y= MARGIN_TOP;
                current_x = start_x;
            }

            layer.use_text(word, font_size, Mm(current_x), Mm(current_y), font);
            // add space width
            let space_width = font.get_width(" ", font_size as i64) as f64 / 1000.0 * font_size;
            current_x += word_width + space_width;
        }
        current_x = start_x;
        current_y -= line_height;
        //page break after a line
        if current_y < MARGIN_BOTTOM {
            let (page, new_layer_id) = doc.add_page(Mm(210.0), Mm(PAGE_HEIGHT), "Layer 1");
            layer = doc.get_page(page).get_layer(new_layer_id);
            current_y= MARGIN_TOP;
        }
    }

    (current_y,layer) //return the final y position after drawing
}

fn add_spacing(
    doc: &PdfDocumentReference,
    mut layer: PdfLayerReference,
    current_y: f64,
    line_height: f64,
    multiplier: f64,
) -> (f64,PdfLayerReference) {
    let mut new_y: f64 = current_y - line_height * multiplier;

    if new_y < MARGIN_BOTTOM {
        let (page,new_layer_id) = doc.add_page(Mm(210.0), Mm(PAGE_HEIGHT), "Layer 1");
        layer = doc.get_page(page).get_layer(new_layer_id);
        new_y = MARGIN_TOP;
    }
    //no new page needed, return current layer
    ( new_y, layer)
}

pub fn export_summary_to_pdf(
    outpath_path:&str,
    summary: &str,
    keywords:&[String],
    resources:&[String],
) ->Result<(), Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Lecture Summary", Mm(210.0), Mm(PAGE_HEIGHT), "Layer 1"); // A4 size
    let mut layer = doc.get_page(page1).get_layer(layer1);

    //layout variables
    let start_x: f64 = 20.0; // starting x position in mm
    let mut current_y: f64 = MARGIN_TOP; // starting y position in mm;
    let title_font_size: f64 = 18.0;
    let section_font_size: f64 = 14.0;
    let bullet_font_size: f64 = 11.0;
    let line_height: f64 = 14.0; // line height in mm

    //different fonts for title, sections and bullet text(might add other fonts later)
    let font: IndirectFontRef = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold: IndirectFontRef = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    //Title
    layer.use_text("Lecture Summary", title_font_size, Mm(start_x), Mm(current_y), &font_bold);
    let (y,layer) = add_spacing(&doc,layer, current_y, line_height, 2.0);
    current_y = y;
    layer = layer;

    //Keywords Section
    layer.use_text("Keywords:", section_font_size, Mm(start_x), Mm(current_y), &font_bold);
    let (y,layer) = add_spacing(&doc, layer, current_y, line_height, 1.0);
    current_y = y;
    layer = layer;
    for kw in keywords {
        let (y,layer) = draw_wrapped_text(
            &doc,
            layer,
            &format!("• {}", kw),
            &font,
            bullet_font_size,
            start_x + 5.0,
            current_y,
            PAGE_WIDTH,
            line_height,
        );
        current_y = y;
        layer = layer;
    }
    let (y,layer) =add_spacing(&doc, layer, current_y, line_height, 2.0);
    current_y = y;
    layer = layer;

    //Summary section
    layer.use_text("Summary:", section_font_size, Mm(start_x), Mm(current_y), &font_bold);
    let (y,layer) = add_spacing(&doc, layer, current_y, line_height, 1.0);
    current_y = y;
    layer = layer;
    let (y,layer) = draw_wrapped_text(
        &doc,
        layer,
        summary,
        &font,
        bullet_font_size,
        start_x + 5.0,
        current_y,
        PAGE_WIDTH,
        line_height,
    );
    current_y = y;
    layer = layer;
    let (y,layer) = add_spacing(&doc, layer, current_y, line_height, 2.0);
    current_y = y;
    layer = layer;

    //Resources Section
    layer.use_text("Resources:", section_font_size, Mm(start_x), Mm(current_y), &font_bold);
    let (y,layer) = add_spacing(&doc, layer, current_y, line_height, 1.0);
    current_y = y;
    layer = layer;

    for r in resources {
        draw_wrapped_text(
            &doc,
            layer,
            &format!("• {}", r),
            &font,
            bullet_font_size,
            start_x + 5.0,
            current_y,
            PAGE_WIDTH,
            line_height,
        );
        current_y = y;
        layer = layer;
    }
    
    //Save PDF
    doc.save(&mut std::fs::File::create(outpath_path)?)?;
    Ok(())

}