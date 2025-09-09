use pdf_extract::extract_text;
use std::error::Error;

pub fn extract_text(path: &str) -> Result<String, Box<dyn Error>> {
    let text = extract_text(path)?;
    Ok(text)
}