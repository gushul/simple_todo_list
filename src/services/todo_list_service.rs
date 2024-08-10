use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::path::PathBuf;
use std::env;
use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use crate::models::task::Task;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoListService {
    tasks: HashMap<String, Task>,
}

impl TodoListService {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, name: String, description: String, date: DateTime<Utc>, category: String) {
        let task = Task {
            name: name.clone(),
            description,
            date,
            category,
            status: false,
        };
        self.tasks.insert(name, task);
        if let Err(e) = self.save_to_file() {
            eprintln!("Error saving changes: {e}");
        }
        println!("Task added successfully!");
    }

    pub fn mark_done(&mut self, name: &str) {
        if let Some(task) = self.tasks.get_mut(name) {
            task.status = true;
            if let Err(e) = self.save_to_file() {
                eprintln!("Error saving changes: {e}");
            }
            println!("Task marked as done!");
        } else {
            println!("Task not found!");
        }
    }

    pub fn update_task(&mut self, name: &str) {
        if let Some(task) = self.tasks.get(name) {
            println!("Enter new details (press Enter to keep current value):");
            
            print!("New name ({}): ", task.name);
            io::stdout().flush().unwrap();
            let mut new_name = String::new();
            io::stdin().read_line(&mut new_name).unwrap();
            new_name = new_name.trim().to_string();
            
            print!("New description ({}): ", task.description);
            io::stdout().flush().unwrap();
            let mut new_description = String::new();
            io::stdin().read_line(&mut new_description).unwrap();
            new_description = new_description.trim().to_string();
            
            print!("New category ({}): ", task.category);
            io::stdout().flush().unwrap();
            let mut new_category = String::new();
            io::stdin().read_line(&mut new_category).unwrap();
            new_category = new_category.trim().to_string();
            
            let mut updated_task = task.clone();
            if !new_name.is_empty() && new_name != task.name {
                updated_task.name = new_name.clone();
            }
            if !new_description.is_empty() {
                updated_task.description = new_description;
            }
            if !new_category.is_empty() {
                updated_task.category = new_category;
            }

            if updated_task.name == task.name {
                self.tasks.insert(name.to_string(), updated_task);
            } else {
                self.tasks.remove(name);
                self.tasks.insert(updated_task.name.clone(), updated_task);
            }

            if let Err(e) = self.save_to_file() {
                eprintln!("Error saving changes: {e}");
            }
            println!("Task updated successfully!");
        } else {
            println!("Task not found!");
        }
    }

    pub fn delete_task(&mut self, name: &str) {
        if self.tasks.remove(name).is_some() {
            if let Err(e) = self.save_to_file() {
                eprintln!("Error saving changes: {e}");
            }
            println!("Task deleted successfully!");
        } else {
            println!("Task not found!");
        }
    }

    pub fn select_tasks(&self, predicate: &str) {
        println!("self.tasks.values(): {:?}", self.tasks.values());
        println!("predicate: {predicate:?}");
        //
        //
        if predicate.is_empty() {
            println!("No tasks match the given criteria.");
        }
           
        if predicate == "*" {
            for task in self.tasks.values() {
            println!("{task:#?}");
            }
            return;
        }

        //
        let filtered_tasks: Vec<&Task> = self.tasks.values()
            .filter(|task| self.evaluate_predicate(task, predicate))
            .collect();

        if filtered_tasks.is_empty() {
            println!("No tasks match the given criteria.");
        } else {
            for task in filtered_tasks {
                println!("{task:#?}");
            }
        }
    }

    fn evaluate_predicate(&self, task: &Task, predicate: &str) -> bool {
        if predicate == "*" {
            return true;
        }

        let conditions: Vec<&str> = predicate.split(" and ").collect();
        conditions.iter().all(|&condition| {
            let parts: Vec<&str> = condition.split_whitespace().collect();
            if parts.len() != 3 {
                return false;
            }

            let (field, operator, value) = (parts[0], parts[1], parts[2]);
            match field {
                "date" => self.compare_date(&task.date, operator, value),
                "category" => self.compare_string(&task.category, operator, value),
                "status" => self.compare_status(task.status, operator, value),
                "description" => self.compare_string(&task.description, operator, value),
                _ => false,
            }
        })
    }

    fn compare_string(&self, field: &str, operator: &str, value: &str) -> bool {
        match operator {
            "=" => field == value.trim_matches('"'),
            "like" => field.contains(value.trim_matches('"')),
            _ => false,
        }
    }

    fn compare_date(&self, date: &DateTime<Utc>, operator: &str, value: &str) -> bool {
        let compare_date = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M").unwrap();
        match operator {
            "<" => date.naive_local() < compare_date,
            "<=" => date.naive_local() <= compare_date,
            "=" => date.naive_local() == compare_date,
            ">=" => date.naive_local() >= compare_date,
            ">" => date.naive_local() > compare_date,
            _ => false,
        }
    }

    fn compare_status(&self, status: bool, operator: &str, value: &str) -> bool {
        let compare_status = value.trim_matches('"') == "on";
        operator == "=" && status == compare_status
    }

    fn get_file_path() -> PathBuf {
        let env = env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());
        let file_name = match env.as_str() {
            "test" => "test_db.json",
            _ => "db.json",
        };
        env::var("TODO_FILE").map_or_else(|_| PathBuf::from(file_name), PathBuf::from)
    }


    fn save_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        let path = Self::get_file_path();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file() -> Self {
        let path = Self::get_file_path();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path);

        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => {
                        if contents.trim().is_empty() {
                            println!("File is empty. Creating a new TodoListService.");
                            let new_service = Self::new();
                            if let Err(e) = new_service.save_to_file() {
                                eprintln!("Error saving new TodoListService: {e}. Proceeding with in-memory service.");
                            }
                            new_service
                        } else {
                            match serde_json::from_str(&contents) {
                                Ok(service) => service,
                                Err(e) => {
                                    eprintln!("Error parsing JSON: {e}. Creating a new TodoListService.");
                                    let new_service = Self::new();
                                    if let Err(e) = new_service.save_to_file() {
                                        eprintln!("Error saving new TodoListService: {e}. Proceeding with in-memory service.");
                                    }
                                    new_service
                                }
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading file: {e}. Creating a new TodoListService.");
                        let new_service = Self::new();
                        if let Err(e) = new_service.save_to_file() {
                            eprintln!("Error saving new TodoListService: {e}. Proceeding with in-memory service.");
                        }
                        new_service
                    }
                }
            },
            Err(e) => {
                eprintln!("Error opening file: {e}. Creating a new TodoListService.");
                let new_service = Self::new();
                if let Err(e) = new_service.save_to_file() {
                    eprintln!("Error saving new TodoListService: {e}. Proceeding with in-memory service.");
                }
                new_service
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::env;

    fn setup() {
        env::set_var("APP_ENV", "test");
    }

    #[test]
    fn test_add_task() {
        setup();
        let mut todo_list = TodoListService::new();
        let name = "Test Task".to_string();
        let description = "Test Description".to_string();
        let date = Utc::now();
        let category = "Test Category".to_string();

        todo_list.add_task(name.clone(), description.clone(), date, category.clone());

        assert!(todo_list.tasks.contains_key(&name));
        let task = todo_list.tasks.get(&name).unwrap();
        assert_eq!(task.description, description);
        assert_eq!(task.category, category);
        assert!(!task.status);
    }

    #[test]
    fn test_mark_done() {
        setup();
        let mut todo_list = TodoListService::new();
        let name = "Test Task".to_string();
        todo_list.add_task(name.clone(), "Description".to_string(), Utc::now(), "Category".to_string());

        todo_list.mark_done(&name);

        let task = todo_list.tasks.get(&name).unwrap();
        assert!(task.status);
    }

    #[test]
    fn test_delete_task() {
        setup();
        let mut todo_list = TodoListService::new();
        let name = "Test Task".to_string();
        todo_list.add_task(name.clone(), "Description".to_string(), Utc::now(), "Category".to_string());

        todo_list.delete_task(&name);

        assert!(!todo_list.tasks.contains_key(&name));
    }

    #[test]
    fn test_select_tasks() {
        setup();
        let mut todo_list = TodoListService::new();
        todo_list.add_task("Task 1".to_string(), "Unit test 1".to_string(), Utc::now(), "Category1".to_string());
        todo_list.add_task("Task 2".to_string(), "Unit test 2".to_string(), Utc::now(), "Category2".to_string());

        let filtered_tasks: Vec<&Task> = todo_list.tasks.values()
            .filter(|task| todo_list.evaluate_predicate(task, "category = \"Category1\""))
            .collect();

        assert_eq!(filtered_tasks.len(), 1);
        assert_eq!(filtered_tasks[0].name, "Task 1");
    }
}
