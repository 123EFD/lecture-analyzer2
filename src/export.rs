use printpdf::*;
use std::fs::File;
use ::std::io::BufWriter;
use std::path::{Path, PathBuf};
use anyhow::Result;
use textwrap::wrap;


const PAGE_HEIGHT:f64 = 297.0;
const MARGIN_TOP:f64 = 277.0;
const MARGIN_BOTTOM:f64 = 20.0;

//designed for long text blocks(summaries/paragraphs) ->wrap words to next line when reaching page margin
fn draw_wrapped_text (
    doc: &PdfDocumentReference,
    mut layer: PdfLayerReference,
    text: &str,
    font_pdf: &IndirectFontRef,
    font_size: f64,
    start_x: f64,
    mut current_y: f64,
    max_chars_per_line: usize,
    line_height: f64,
) -> (f64,PdfLayerReference) {
    let wrapped_lines: Vec<std::borrow::Cow<'_, str>> = wrap(text, max_chars_per_line);

    for line in wrapped_lines{
        layer.use_text(line, font_size, Mm(start_x), Mm(current_y), font_pdf);
        current_y -= line_height;

            //page break
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
    output_path:&str,
    summary: &str,
    keywords:&[String],
    resources:&[String],
) ->Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    let font_pdf: IndirectFontRef = doc.add_external_font(File::open("fonts/roboto/Roboto-Regular.ttf")?)?;
    let font_bold_pdf: IndirectFontRef = doc.add_external_font(File::open("fonts/roboto/Roboto-Bold.ttf")?)?;

    //Title
    layer.use_text("Lecture Summary", title_font_size, Mm(start_x), Mm(current_y), &font_bold_pdf);
    let (y,layer_) = add_spacing(&doc,layer, current_y, line_height, 2.0);
    current_y = y;
    layer = layer_;

    //Keywords Section
    layer.use_text("Keywords:", section_font_size, Mm(start_x), Mm(current_y), &font_bold_pdf);
    let (y,layer_) = add_spacing(&doc, layer, current_y, line_height, 1.0);
    current_y = y;
    layer = layer_;

    //iterate over keyword and print as a separate bullet point
    for kw in keywords {
        let bullet: String = format!("• {}", kw);
        layer.use_text(&bullet, bullet_font_size, Mm(start_x + 5.0), Mm(current_y), &font_pdf);
        current_y -= line_height;
        //Page break for keywords
        if current_y < MARGIN_BOTTOM {
            let (page, new_layer_id) = doc.add_page(Mm(210.0), Mm(PAGE_HEIGHT), "Layer 1");
            layer = doc.get_page(page).get_layer(new_layer_id);
            current_y = MARGIN_TOP;
        }
    }
    let (y,layer_) =add_spacing(&doc, layer, current_y, line_height, 2.0);
    current_y = y;
    layer = layer_;

    //Summary section
    layer.use_text("Summary:", section_font_size, Mm(start_x), Mm(current_y), &font_bold_pdf);
    let (y,layer_) = add_spacing(&doc, layer, current_y, line_height, 1.0);
    current_y = y;
    layer = layer_;
    let (y,layer_) = draw_wrapped_text(
        &doc,
        layer,
        summary,
        &font_pdf,
        bullet_font_size,
        start_x + 5.0,
        current_y,
        80,
        line_height,
    );
    current_y = y;
    layer = layer_;
    let (y,layer_) = add_spacing(&doc, layer, current_y, line_height, 2.0);
    current_y = y;
    layer = layer_;

    //Resources Section
    layer.use_text("Resources:", section_font_size, Mm(start_x), Mm(current_y), &font_bold_pdf);
    let (y,layer_) = add_spacing(&doc, layer, current_y, line_height, 1.0);
    current_y = y;
    layer = layer_;

    for r in resources {
        let (y,layer_) =draw_wrapped_text(
            &doc,
            layer,
            &format!("• {}", r),
            &font_pdf,
            bullet_font_size,
            start_x + 5.0,
            current_y,
            80,
            line_height,
        );
        current_y = y;
        layer = layer_;
    }


    
    //Save PDF(Ensure output path is user/project directory)
    let safe_dir: &Path = Path::new("output");
    std::fs::create_dir_all(safe_dir)?; //create output directory if not exists
    let safe_path: PathBuf = safe_dir.join(output_path);
    let file: File = File::create(&safe_path)?;
    let mut buf_writer: BufWriter<File> = BufWriter::new(file);
    doc.save(&mut buf_writer)?;
    Ok(())

}