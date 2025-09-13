use std::error::Error;

//Extract text from PDF using pdf-extract crate
pub fn extract_text(path: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text: String = pdf_extract::extract_text(path)?;
    Ok(text)
}