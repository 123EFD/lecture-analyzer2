mod utils;
mod analyze;
mod export;
mod pdf;

use::anyhow::Result;

fn main() -> Result<()> {
    // Steps:Parse CLI arg.s for input/output files
    let pdf_path = "lecture.pdf";
    let output_path = "summary.pdf";
    let summary_sentences = 5;

    // 1. Extract text from PDF
    let lecture_text: String = pdf::extract_text(pdf_path)?;

    // 2.(a) Analyze text: keywords extraction
    let keywords:Vec<String>  = analyze::extract_keywords(&lecture_text)?;

    // 2.(b) Analyze text: summary generation
    let summary: Vec<String> = analyze::extract_summary(&lecture_text, summary_sentences)?;

    // 3. Suggest reference links
    let resources: Vec<String> = utils::suggest_resources(&keywords)?;

    //4. Export to summary PDF
    export::export_summary_to_pdf(output_path, &keywords, &summary.join(""), &resources)?;

    Ok(())
}
