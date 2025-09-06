use stopwords::{Language, Stopwords, NLTK};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn extract_text(text:&str) -> Vec<String> {
    // placeholder for RAKE keyword extraction
    text.split_whitespace()
    
}

pub fn extract_sentences(text:&str) -> Vec<String> {
    //placeholder: real Textrank logic goes here
}