use std::path::PathBuf;

use clap::{Parser, Subcommand};
use upenn_curf_scraper::{update_auth::update_auth, scrape_links::scrape_links, scrape_opportunities::scrape_opportunities, process_opportunities::process_opportunities, filter::filter};

#[derive(Parser)]
#[clap(author="Clayton Hickey", version="v1.0.0", about="For scraping UPenn CURF")]
struct Args {
    #[clap(subcommand)]
    command: Command,
    #[clap(value_parser, short, long, default_value = "./")]
    path: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    UpdateAuth {},
    ScrapeLinks {},
    ScrapeOpportunities {
        #[clap(value_parser, short, long)]
        skip_already_pulled: bool
    },
    ProcessOpportunities {},
    Filter {},
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::UpdateAuth {} => {
            update_auth(args.path).await?;
        },
        Command::ScrapeLinks {} => {
            scrape_links(args.path).await?;
        },
        Command::ScrapeOpportunities { skip_already_pulled } => {
            scrape_opportunities(args.path, skip_already_pulled).await?;
        },
        Command::ProcessOpportunities {} => {
            process_opportunities(args.path).await?;
        },
        Command::Filter {} => {
            filter(args.path).await?;
        }
    }
    Ok(())
}