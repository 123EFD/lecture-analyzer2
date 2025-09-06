mod utils;
mod analyze;
mod export;
mod pdf;

use::anyhow::Result;

fn main() -> Result<()> {
    // 1. Load lecture note PDF
    let text: String = pdf::extract_text("input.pdf")?;

    // 2. Analyze keywords (RAKE)
    let analysis: String = analyze::extract_text(&text)?;

    // 3. Export sentences (TextRank)
    let sentences: Vec<String> = analyze::extract_sentences(&text);

    // 4. Suggest reference links
    let links: Vec<String> = utils::suggest_links(&keywords)?;

    //5. Export to summary PDF
    export::export_summary("summary.pdf", &keywords, &sentences, &links)?;

    Ok(())
}
