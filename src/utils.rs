use anyhow::Result;
use reqwest::blocking::get;
use scraper::{Html, Selector};

//Fetch real reference links for keywords (Wikipedia links as placeholders)
pub fn suggest_resources(keywords: &[String]) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {

    // Placeholder: In a real implementation, this might query an API or database
    let mut resources = Vec::new();

    for kw in keywords {
        let url = format!("https://en.wikipedia.org/wiki/{}", kw.replace(" ", "_"));

        //Fetch to page
        let resp: String = get(&url)?.text()?;

        let document = Html::parse_document(&resp);

        //select the first few internal links 
        let selector = Selector::parse("#mw-content-text a[href]").unwrap();

        for element in document.select(&selector).take(5) {
            if let Some(link) = element.value().attr("href") {
                if link.starts_with("/wiki/") && !link.contains(":") {
                    let full_link = format!("https://en.wikipedia.org{}", link);
                    resources.push(full_link);
                }
            }
        }
        resources.push(url); //add the main wiki page as well
    }
    Ok(resources)
    
}

//Input type: &String (a single string, not multiple keywords).
//The for kw in keywords loop won’t compile because String is not iterable.
//#mw-content-text filters out sidebars, navigation, ads, and unrelated links.
//a[href] selects only collecting real hyperlinks
//("/wiki/") filters out external links, citations, and special pages. Only gets the main article links.