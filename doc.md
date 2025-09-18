# Project Documentation: Core Rust Source Files

This document summarizes the syntax, concepts, and key variables used in the main source files of the project. It also explains a previously encountered error and how to resolve it.

---

## 1. `export.rs`

**Purpose:**  
Handles exporting summaries, keywords, and resources to a PDF file using the `printpdf` crate.

**Key Concepts and Syntax:**
- **External crates:** Uses `printpdf` for PDF generation and `textwrap` for word wrapping.
- **Functions:**
  - `export_summary_to_pdf(...)`: Main function to generate and save a PDF with title, keywords, summary, and resources.
  - `draw_wrapped_text(...)`: Helper to wrap and print long text blocks.
  - `add_spacing(...)`: Helper to add vertical space between sections.
- **Variables:**  
  - `doc`, `layer`: PDF document and drawing layer references  
  - `current_y`, `start_x`: Coordinates for content placement  
  - `font_pdf`, `font_bold_pdf`: Font references  
  - `keywords`, `resources`: Vectors containing keywords and resource links

**Error Encountered:**  
If the `resources` vector is empty, the "Resources:" section in the PDF is blank.  
**Solution:**  
Ensure that keyword extraction (`extract_keywords`) returns meaningful (short and valid) keywords, not long or invalid strings, so that resource generation works correctly.

---

## 2. `analyze.rs`

**Purpose:**  
Extracts keywords and summaries from lecture text.

**Key Concepts and Syntax:**
- **Stopwords filtering:** Uses a `HashSet` to filter out common English words.
- **Keyword extraction:** Tokenizes text, counts frequency, and selects the top N keywords.
- **Summary extraction:** Splits text into sentences and selects the first N as a summary.
- **Variables:**  
  - `stopwords`: Set of words to ignore in keyword extraction
  - `freq`: HashMap for keyword frequency counting
  - `keywords`: Final vector of keywords

**Error Encountered:**  
A common error is treating the type `Vec<String, usize>` instead of `Vec<(String, usize)>`.  
**Solution:**  
Use `Vec<(String, usize)>` for storing word-frequency pairs.

---

## 3. `pdf.rs`

**Purpose:**  
Extracts plain text content from PDF files.

**Key Concepts and Syntax:**
- **External crate:** Uses `pdf_extract` for PDF parsing.
- **Function:**
  - `extract_text(path: &str)`: Returns the extracted text as a `Result<String, ...>`.
- **Variables:**  
  - `path`: File path to the PDF  
  - `text`: Extracted text

---

## 4. `utils.rs`

**Purpose:**  
Suggests resource links (e.g., Wikipedia) based on extracted keywords.

**Key Concepts and Syntax:**
- **HTTP requests:** Uses `reqwest` for fetching web pages.
- **HTML parsing:** Uses `scraper` crate to extract relevant links from HTML.
- **Variables:**  
  - `resources`: Vector of suggested resource URLs  
  - `keywords`: Input keywords for generating resources  
  - `stopwords`: Used for filtering in other modules

**Error Encountered:**  
If keywords are long or invalid, resource generation will fail or return an empty vector.  
**Solution:**  
Ensure that only valid, short keywords are passed to this function.

---

## 5. `main.rs`

**Purpose:**  
Entry point with CLI parsing and command dispatch.

**Key Concepts and Syntax:**
- **CLI parsing:** Uses `clap` crate for command-line argument handling.
- **Command enums:** Defines possible subcommands (Analyze, Keywords, Summary, Resources).
- **Variables:**  
  - `cli`: Parsed CLI input  
  - `lecture_text`, `keywords`, `summary`, `resources`: Data passed between modules

---

## **Common Error and Its Resolution**

### Error:
> Keyword extraction returns long, concatenated strings, causing resource lookup to fail and the PDF to show an empty Resources section.

**Root Cause:**  
Incorrect keyword extraction logic: long strings instead of individual words.

**Solution:**  
- Use tokenization, stopwords filtering, and frequency counting to return a list of short, valid keywords.
- **Correct code example:**  
  ```rust
  let mut keywords: Vec<(String, usize)> = freq.into_iter().collect();
  ```

---

**Note:**  
This documentation is designed to be updated as the codebase evolves or more advanced NLP methods are introduced.
