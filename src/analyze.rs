use stopwords::{Language, Stopwords, NLTK};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn extract_keywords(text:&str) -> Vec<String> {
    //1.Load Stopwords
    let stopwords: HashSet<String> = NLTK::stopwords(Language::English)
        .unwrap()
        .into_iter()
        .map(|s| s.to_lowercase().to_string())
        .collect();

    //2. Split text into candidates_phrases from stopwords and punctuations 
    let re = Regex::new(r#"[.,;!?()\[\]"\n]"#).unwrap();
    let cleaned_text = re.replace_all(text, ""); 

    let words: Vec<String> = cleaned_text
        .split_whitespace()
        .map(|w| w.trim().to_lowercase())
        .filter(|w| !w.is_empty() && !stopwords.contains(w))
        .collect();

    let mut candidates_phrases: Vec<String> = Vec::new();
    let mut phrase: Vec<String> = Vec::new();

    for word in words {
        if stopwords.contains(&word) {
            if !phrase.is_empty() {
                candidates_phrases.push(phrase.join(" "));
                phrase.clear();
            }
        } else {
            phrase.push(word);
        }
    }
    //Push last phrase if not empty
    if !phrase.is_empty() {
        candidates_phrases.push(phrase.join(" "));
    }

    //3.Calculate word frequency and degree
    let mut word_freq: HashMap<String,usize>=HashMap::new();
    let mut word_degree: HashMap<String,usize>=HashMap::new();
    for phrase in &candidates_phrases {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let degree = if words.len() > 1 { words.len() - 1 } else { 0 };
        for word in &words {
            *word_freq.entry(word.to_string()).or_insert(0) += 1;
            *word_degree.entry(word.to_string()).or_insert(0) += degree;
        }
    }

    let mut word_score: HashMap<String,f64>=HashMap::new();
    for (word, &freq) in &word_freq {
        let degree: Option<&usize> = word_degree.get(word).unwrap_or(&0); 
        word_score.insert(word.clone(), (*degree as f64 + freq as f64) / freq as f64);
    }

    //Score candidates_phrases
    let mut phrase_score: HashMap<String, f32> = HashMap::new();
    for phrase in &candidates_phrases {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let mut score: f32 = words
            .iter()
            .map(|w| word_score.get(&w.to_string()).unwrap_or(&0.0))
            .sum();
        phrase_score.insert(phrase.clone(), score);
    }

    //Sort and return top N phrases/keywords
    let mut phrases: Vec<(&String, &f32)> = phrase_score.iter().collect();
    phrases.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap()); //a=&String, b=&f32
    phrases.into_iter().take(10).map(|(p, _)| p.clone()).collect()
}

    //Split text convert into sentences (basic version)
    fn split_text_into_sentences(text:&str) -> Vec<String> {
        let re = Regex::new(r"(?m)(?<!\w\.\w.)(?<![A-Z][a-z]\.)(?<=\.|\?)\s").unwrap();
        re.split(text)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    //Helper function for sentence similarity by word overlap
    fn sentence_similarity(s1:&str, s2:&str) -> f32 {
        let set1: HashSet<&str> = s1.split_whitespace().collect();
        let set2: HashSet<&str> = s2.split_whitespace().collect();
        let common: HashSet<&&str> = set1.intersection(&set2).count() as f32;
        if common.is_empty() {
            0.0
        } else {
            (common.len() as f32) / ((set1.len() + set2.len()) as f32 / 2.0)
        }
    }

    //Textrank for sentence extraction
pub fn extract_summary(text:&str, num_sentences: usize) -> Vec<String> {
    let sentences_storage: Vec<String> = split_text_into_sentences(text);

    //Similariy matrix
    let mut similarity:Vec<Vec<f32>>=vec![vec![0.0; sentences_storage.len()]; sentences_storage.len()];
    for i in 0..sentences_storage.len() {
        for j in 0..sentences_storage.len() {
            if i != j {
                similarity[i][j] = sentence_similarity(&sentences_storage[i], &sentences_storage[j]);
            }
        }
    }

    //Initialize sentence scores
    let mut scores:Vec<f32> = vec![1.0; sentences_storage.len()];
    let mut new_scores:Vec<f32> = vec![1.0;sentences_storage.len()];

    //Iterative scoring (UPDATE SCORES)
    for _ in 0..20 {
        for i in 0..sentences_storage.len(){
            let mut sum: f32 = 0.0;
            //Sum of weighted scores from other sentences
            for j in 0..sentences_storage.len() {
                if i != j {
                    //weighted by similarity(i,j)
                    sum += similarity[j][i] * scores[j] / similarity[j].iter().sum::<f32>();
                }
            }
            new_scores[i] = 0.15 + 0.85 * sum; //Damping factor
        }
        scores = new_scores.clone();
    }

    //Select top N sentences
    let mut indexed:Vec<(usize, f32)> = scores
                                            .iter()
                                            .enumerate()
                                            .map(|(i, &score)| (i, score))
                                            .collect();
    indexed.sort_by(|a: &(usize, f32), b: &(usize, f32)| b.1.partial_cmp(&a.1).unwrap());
    indexed
        .iter()
        .take(num_sentences)
        .map(|(i, _)| sentences_storage[i].clone())
        .collect()
}

