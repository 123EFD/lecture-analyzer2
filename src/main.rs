#![allow(unused_imports)]
use clap::{Parser, Subcommand};
use axum::{
    extract::Multipart, response::{Html, IntoResponse}, routing::{get, post},  Json, Router
};
use std::net::SocketAddr; //SocketAddr: Represents a socket address (IP + port)
use tower_http::services::ServeDir; //ServeDir: Lets you serve static files (HTML, CSS, JS)
use serde::Serialize;
use::anyhow::Result;
use axum::Server;

mod utils;
mod analyze;
mod export;
mod pdf;

/*customize command-line help and metadata for CLI tool -> long-about is for detailed description
run with --help/-h/--version, this will show the below info*/

#[derive(Parser)]
#[command(author, version, about = "Analyze lecture PDFs to extract keywords, summaries, and suggest resources", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        input:String,
        #[arg(long)]
        export:String,
        #[arg(long, default_value_t = 5)]
        summary_sentences: usize,
    },
    Keywords {
        input:String,
    },
    Summary {
        input:String,
        #[arg(long, default_value_t = 5)]
        summary_sentences: usize,
    },
    Entities {
        input:String,
    },
    Resources {
        input:String,
    },
    //Start the web server for uploading and summarizing PDFs
    Serve,
    //add more subcommand if any 
}

//For API response
#[derive(Serialize)]
struct SummaryResponse {
    summary:Vec<String>,
    keywords:Vec<String>,
    resources:Vec<String>,
}

/*Async functions that process web requests.
  tokio::main --> Nedded for async runtime(Axum require it)
*/

//Handles the logic when a user uploads a PDF (request handler)
async fn summarize_api(mut multipart: Multipart) -> impl IntoResponse {
    //1.Extract PDF from multipart form
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            let data = field.bytes().await.unwrap();
            //Save PDF temporarily (or process directly if supported)
            let temp_path: &'static str = "temp_upload.pdf";
            std::fs::write(temp_path,&data).unwrap();

            //2. Use your exisitng pipeline
            let lecture_text:String = match pdf::extract_text(temp_path) {
                Ok(text) => text,
                Err(_e) => return Json(SummaryResponse{
                    summary: vec!["Faild to extract text".into()],
                    keywords: vec![],
                    resources: vec![],
                }),
            };

            let keywords: Vec<String> = analyze::extract_keywords_ner(&lecture_text);
            let summary: Vec<String> = analyze::extract_summary(&lecture_text,5,&keywords);
            let resources: Vec<String> = match utils::suggest_resources(&keywords) {
                Ok(r) => r,
                Err(_) => vec![],
            };

            //cleanup
            let _  = std::fs::remove_file(temp_path); //"let _" ignore the result

            return Json(SummaryResponse { 
                summary,
                keywords,
                resources,
            });
        }
    }
    
    //If no file was found
    Json(SummaryResponse { 
        summary: vec!["No file uploaded".into()], 
        keywords: vec![], 
        resources: vec![] 
    })
}

//Start the Axum web server and defines what to do for each route
async fn run_server() {
    let static_files: ServeDir = ServeDir::new("./static");
    //::<()> or : Router<()> when creating your Router if you are not using shared state.
    let app = Router::new()
        .route("/", get(|| async { Html(std::fs::read_to_string("./static/index.html").unwrap()) }))
        .route("/api/summarize", post(summarize_api))
        //Serve static assets like style.css at /static/*
        .nest_service("/static", axum::routing::get_service(static_files).handle_error(|_| async { (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error") }));

    //start service request with app 
    let addr =  SocketAddr::from(([127,0,0,1], 8080));
    println!("Server running at http://{}", addr);
    
    //create a server that listens on the specified address and serves the app
    axum::Server::bind(&addr)
        .serve(app.into_make_service()) //provide client's socket address to handlers
        .await
        .unwrap();
}

//match is use to handle each subcommand variant 
/*if let Err(e) = run_analysis(&input, &export, summary_sentences) {
                eprintln!("Error: {}", e);   --> This one later add for error handling*/ 

fn main() ->Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //1. Parse CLI arguments{
    let cli = Cli::parse();
    match cli.command {
        Commands::Serve => {
            /*Run the async server (block on it)-->bridge between sync Rust code and async code
            lets you "block" the current thread and wait for the async function to finish, running it to completion.*/
            //future :a value representing a computation that will finish later
            tokio::runtime::Runtime::new()?.block_on(run_server()); 
        }
        Commands::Analyze { input, export, summary_sentences } => {
            let lecture_text:String = pdf::extract_text(&input)?;
            let keywords:Vec<String>  = analyze::extract_keywords_ner(&lecture_text);
            let summary: Vec<String> = analyze::extract_summary(&lecture_text, summary_sentences,&keywords);
            let resources: Vec<String> = utils::suggest_resources(&keywords)?;
            println!("Exporting resources, count: {}", resources.len());
            for r in &resources {
                println!("Resource: {}", r);
            }
            export::export_summary_to_pdf(&export, &summary.join("\n"), &keywords, &resources)?;
            println!("Analysis complete. Summary exported to {}", export);
        }
        Commands::Keywords { input } => {
            let lecture_text:String = pdf::extract_text(&input)?;
            let keywords:Vec<String>  = analyze::extract_keywords_ner(&lecture_text);
            println!("Extracted Keywords:");
            for keyword in keywords {
                println!("- {}", keyword);
            } 
        }
        Commands::Summary { input, summary_sentences } => {
            let lecture_text:String = pdf::extract_text(&input)?;
            let keywords:Vec<String>  = analyze::extract_keywords_ner(&lecture_text);
            let summary: Vec<String> = analyze::extract_summary(&lecture_text, summary_sentences,&keywords);
            println!("Extracted Summary:");
            for sentence in summary {
                println!("- {}", sentence);
            } 
        }
        Commands::Resources { input } => {
            let lecture_text:String = pdf::extract_text(&input)?;
            let keywords:Vec<String>  = analyze::extract_keywords_ner(&lecture_text);
            let resources: Vec<String> = utils::suggest_resources(&keywords)?;
            println!("Suggested Resources:");
            for resource in resources {
                println!("- {}", resource);
            }
        }
        Commands::Entities { input } => {
            let lecture_text:String = pdf::extract_text(&input)?;
            let entities: Vec<rust_bert::pipelines::ner::Entity> = analyze::extract_entities_ner(&lecture_text);
            println!("Extracted Entities:");
            for entity in entities {
                println!("Word: {}, Label: {}, Score: {}", entity.word, entity.label, entity.score);
            } 
        }
    }

    Ok(())
}
//cargo run -- analyze my_lecture.pdf --export summary.pdf
