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
    /// Add a new task to the todo list
    Add {
        /// The name of the task
        name: String,
        /// A description of the task
        description: String,
        /// The due date of the task in format DD-MM-YYYY HH:MM
        date: String,
        /// The category of the task
        category: String,
    },
    /// Mark a task as done
    Done {
        /// The name of the task to mark as done
        name: String,
    },
    /// Update an existing task
    Update {
        /// The name of the task to update
        name: String,
    },
    /// Delete a task from the todo list
    Delete {
        /// The name of the task to delete
        name: String,
    },
    /// Select and display tasks based on a predicate
    Select {
        /// The predicate to filter tasks. Use '\*' to select all tasks.
        /// For filtering, use the format: "* where <condition>"
        /// Example: "\* where date < '2023-12-31 00:00' and category=work and status=on and description like project"
        predicate: Vec<String>,
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
            let predicate_str = if predicate.is_empty() {
                "*".to_string()
            } else {
                predicate.join(" ")
            };
            service.select_tasks(&predicate_str);
        }
        None => {
            println!("No command was used");
        }
    }
}
