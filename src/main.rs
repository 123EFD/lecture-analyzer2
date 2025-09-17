use clap::{Parser, Subcommand};
use::anyhow::Result;
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
    Resources {
        input:String,
    }
    //add more subcommand if any 
}

//match is use to handle each subcommand variant 
/*if let Err(e) = run_analysis(&input, &export, summary_sentences) {
                eprintln!("Error: {}", e);   --> This one later add for error handling*/ 

fn main() ->Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //1. Parse CLI arguments{
    let cli = Cli::parse();
    match cli.command {
        Commands::Analyze { input, export, summary_sentences } => {
            let lecture_text:String = pdf::extract_text(&input)?;
            let keywords:Vec<String>  = analyze::extract_keywords(&lecture_text);
            let summary: Vec<String> = analyze::extract_summary(&lecture_text, summary_sentences);
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
            let keywords:Vec<String>  = analyze::extract_keywords(&lecture_text);
            println!("Extracted Keywords:");
            for keyword in keywords {
                println!("- {}", keyword);
            } 
        }
        Commands::Summary { input, summary_sentences } => {
            let lecture_text:String = pdf::extract_text(&input)?;
            let summary: Vec<String> = analyze::extract_summary(&lecture_text, summary_sentences);
            println!("Extracted Summary:");
            for sentence in summary {
                println!("- {}", sentence);
            } 
        }
        Commands::Resources { input } => {
            let lecture_text:String = pdf::extract_text(&input)?;
            let keywords:Vec<String>  = analyze::extract_keywords(&lecture_text);
            let resources: Vec<String> = utils::suggest_resources(&keywords)?;
            println!("Suggested Resources:");
            for resource in resources {
                println!("- {}", resource);
            }
        }
    }

    Ok(())
}
//cargo run -- analyze my_lecture.pdf --export summary.pdf
