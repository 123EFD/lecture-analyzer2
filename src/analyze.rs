use rust_bert::pipelines::ner::{NERModel, Entity};
use std::collections::{HashMap, HashSet};

//extract named entity details: word, label, score etc.
pub fn extract_entities_ner(text:&str) -> Vec<Entity> {
    let ner_model: NERModel = NERModel::new(Default::default()).unwrap();
    let entities: Vec<Vec<rust_bert::pipelines::ner::Entity>> = ner_model.predict(&[text]);
    //Return as Vec<Entity>
    entities.get(0).cloned().unwrap_or_else(Vec::new)
}

pub fn extract_keywords_ner(text:&str) -> Vec<String> {
    let ner_model: NERModel = NERModel::new(Default::default()).unwrap();
    let entities: Vec<Vec<rust_bert::pipelines::ner::Entity>> = ner_model.predict(&[text]);
    let mut keywords: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = std::collections::HashSet::new();
    for entity in entities.get(0).unwrap_or(&Vec::new()) {
        if seen.insert(entity.word.clone()) {
            keywords.push(entity.word.clone());
        }
    }
    keywords
}
    #[allow(dead_code)]
    //1.Load Stopwords (basic keyword extraction)
    pub fn extract_keywords(text:&str) -> Vec<String> {
    let stopwords: HashSet<&str> = [
        "a","about","above","after","again","against","all","am","an","and","any","are",
        "aren't","as","at","be","because","been","before","being","below","between","both",
        "but","by","can't","cannot","could","couldn't","did","didn't","do","does","doesn't",
        "doing","don't","down","during","each","few","for","from","further","had","hadn't","has",
        "hasn't","have","haven't","having","he","he'd","he'll","he's","her","here","here's","hers",
        "herself","him","himself","his","how","how's","i","i'd","i'll","i'm","i've","if","in","into","is",
        "isn't","it","it's","its","itself","just","ll","may","me","mightn't","more","most","mustn't","my",
        "myself","needn't","no","nor","not","now","of","off","on","once","only","or","other","our","ours",
        "ourselves","out","over","own","re","s","same","shan't","she","she'd","she'll","she's","should",
        "shouldn't","so","some","such","t","than","that","that's","the","their","theirs","them","themselves","then",
        "there","there's","these","they","they'd","they'll","they're","they've","this","those","through","to","too","under","until",
        "up","ve","very","was","wasn't","we","we'd","we'll","we're","we've","were","weren't","what","what's","when","when's",
        "where","where's","which","while","who","who's","whom","why","why's","will","with","won't","would","wouldn't","y","you","you'd","you'll",
        "you're","you've","your","yours","yourself","yourselves"
    ].iter().cloned().collect();

    let mut freq: HashMap<String, usize> = HashMap::new();

    //splite text into words and count frequency
    for word in text
        .split(|c:char|!c.is_alphabetic())
        .map(|w: &str| w.to_lowercase())
        .filter(|w: &String|w.len() > 2 && w.len() < 20 && !stopwords.contains(w.as_str()))
    {
        *freq.entry(word).or_insert(0) += 1;
    }

    //Get top 10 most frequency keywords
    let mut keywords:Vec<(String,usize)> = freq.into_iter().collect();
    keywords.sort_by(|a, b| b.1.cmp(&a.1)); 
    keywords.into_iter().take(10).map(|(w, _)| w).collect()
}

pub fn extract_summary(text:&str, num_sentences: usize) -> Vec<String> {
    //Simple sentence splitting by '.'
    let sentences: Vec<&str> = text.split('.').collect();
    let mut results: Vec<String> = Vec::new();
    for sentence in sentences.iter().take(num_sentences) {
        if !sentence.trim().is_empty() {
            results.push(sentence.trim().to_string());
        }
    }
    results
}

    /*2. Split text into candidates_phrases from stopwords and punctuations 
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
        let degree: &usize = word_degree.get(word).unwrap_or(&0); 
        word_score.insert(word.clone(), (*degree as f64 + freq as f64) / freq as f64);
    }

    //Score candidates_phrases
    let mut phrase_score: HashMap<String, f32> = HashMap::new();
    for phrase in &candidates_phrases {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let score: f64 = words
            .iter()
            .map(|w| word_score.get(&w.to_string()).unwrap_or(&0.0))
            .sum();
        phrase_score.insert(phrase.clone(), score as f32);
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
            .map(|res|res.expect("Valid regex split").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    //Helper function for sentence similarity by word overlap
    fn sentence_similarity(s1:&str, s2:&str) -> f32 {
        let set1: HashSet<&str> = s1.split_whitespace().collect();
        let set2: HashSet<&str> = s2.split_whitespace().collect();
        let common:usize = set1.intersection(&set2).count() ;
        if common == 0 {
            0.0
        } else {
            (common as f32) / ((set1.len() + set2.len()) as f32 / 2.0)
        }
    }
    */

    //Textrank for sentence extraction
    /*Similariy matrix
    let mut similarity:Vec<Vec<f32>>=vec![vec![0.0; sentences_storage.len()]; sentences_storage.len()];
    for i in 0..sentences_storage.len() {
        for j in 0..sentences_storage.len() {
            if i != j {
                similarity[i][j] = sentence_similarity(&sentences_storage[i], &sentences_storage[j]);
    */



    
            

    /*Initialize sentence scores
    let mut scores:Vec<f32> = vec![1.0; sentences_storage.len()];
    let mut new_scores:Vec<f32> = vec![1.0;sentences_storage.len()];

    //Iterative scoring (UPDATE SCORES)
    for _ in 0..20 {
        for i in 0..sentences_storage.len(){
            let mut sum: f32 = 0.0;
            //Sum of weighted scores from other sentences
            for j in 0..sentences_storage.len() {
                if i != j {
                    let sim_sum:f32 = similarity[j].iter().sum::<f32>();
                    if sim_sum != 0.0 {
                    //weighted by similarity(i,j)
                    sum += similarity[j][i] * scores[j] / sim_sum;
                    } 
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
        .map(|(i, _)| sentences_storage[*i].clone())
        .collect()
    
}
*/

