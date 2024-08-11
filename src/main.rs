mod models;
mod services;
mod utils;

use clap::{Parser, Subcommand};
use services::TodoListService;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        description: String,
        date: String,
        category: String,
    },
    Done {
        name: String,
    },
    Update {
        name: String,
    },
    Delete {
        name: String,
    },
    Select {
        predicate: String,
    },
}

fn main() {
    env::set_var("APP_ENV", "production");

    let cli = Cli::parse();
    let mut service = TodoListService::load_from_file();

    match &cli.command {
        Some(Commands::Add { name, description, date, category }) => {

            match utils::date::parse(date) {
                Ok(parsed_date) => {
                    service.add_task(name.clone(), description.clone(), parsed_date, category.clone());
                },
                Err(e) => {
                    eprintln!("Error parsing date: {e}");
                }
            }
        }
        Some(Commands::Done { name }) => {
            service.mark_done(name);
        }
        Some(Commands::Update { name }) => {
            service.update_task(name);
        }
        Some(Commands::Delete { name }) => {
            service.delete_task(name);
        }
        Some(Commands::Select { predicate }) => {
            println!("Selecting tasks with predicate: {predicate}");
            service.select_tasks(predicate);
        }
        None => {
            println!("No command was used");
        }
    }
}
