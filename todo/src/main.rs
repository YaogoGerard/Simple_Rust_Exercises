use std::fs;
use std::io;

const FILE_PATH: &str = "todos.json";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Status {
    InProgress,
    Completed,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Task {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Priority,
    status: Status,
}

impl Task {
    fn new(
        id: i32,
        title: String,
        description: Option<String>,
        priority: Priority,
        status: Status,
    ) -> Self {
        Self {
            id,
            title,
            description,
            priority,
            status: Status::InProgress,
        }
    }
    fn update_priority(&mut self, new_priority: Priority) {
        self.priority = new_priority;
    }
    fn update_status(&mut self, new_status: Status) {
        self.status = new_status;
    }
}

fn save_tasks(tasks: &[Task]) {
    if let Ok(json) = serde_json::to_string_pretty(tasks) {
        let _ = fs::write(FILE_PATH, json);
    }
}

fn load_tasks() -> Vec<Task> {
    if let Ok(content) = fs::read_to_string(FILE_PATH) {
        if let Ok(tasks) = serde_json::from_str(&content) {
            return tasks;
        }
    }
    Vec::new()
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();
    let mut next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    println!("{:=^30}", "TODO List APP");

    loop {
        println!("1.Add a Task");
        println!("2.View all task");
        println!("3.Set a task as completed");
        println!("4.Change task priority");
        println!("5.Delete a task");
        println!("0.Leave TODO");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Reading Error");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter the name of the task");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Reading Error");
                let title = title.trim().to_string();
                println!("Enter the description (optionnal, press Enter key to skip)");
                let mut desc_input = String::new();
                io::stdin()
                    .read_line(&mut desc_input)
                    .expect("Reading Error");
                let description: Option<String> = if desc_input.trim().is_empty() {
                    None
                } else {
                    Some(desc_input.trim().to_string())
                };

                println!("Set the Priority (1=Low, 2=Medium, 3=High) :");
                let mut prio_input = String::new();
                io::stdin()
                    .read_line(&mut prio_input)
                    .expect("Reading Error");
                let priority = match prio_input.trim() {
                    "1" => Priority::Low,
                    "2" => Priority::Medium,
                    "3" => Priority::High,
                    _ => Priority::Medium,
                };

                let task = Task::new(next_id, title, description, priority, Status::InProgress);
                tasks.push(task);
                println!("Task added succesfully! (ID: {})", next_id);
                next_id += 1;
            }
            2 => {
                println!("\n{:=^30}", " Task List ");
                if tasks.is_empty() {
                    println!("No task in the list.");
                } else {
                    for task in &tasks {
                        let status = match task.status {
                            Status::InProgress => "In Progress",
                            Status::Completed => "Completed",
                        };

                        let priority = match task.priority {
                            Priority::Low => "Low",
                            Priority::Medium => "Medium",
                            Priority::High => "High",
                        };

                        println!(
                            "ID: {} | {} | Priority: {} | {}",
                            task.id, task.title, priority, status
                        );

                        if let Some(desc) = &task.description {
                            println!("   Description: {}", desc);
                        }
                        println!("{:─^30}", "=");
                    }
                }
            }
            3 => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                // Show tasks first
                for task in &tasks {
                    println!("ID: {} | {}", task.id, task.title);
                }

                println!("\nEnter task ID to mark as completed:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Reading error");

                let id: i32 = match input.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid ID!");
                        continue;
                    }
                };

                if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                    task.update_status(Status::Completed);
                    println!("Task {} marked as completed!", id);
                } else {
                    println!("Task with ID {} not found.", id);
                }
            }
            4 => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }

                for task in &tasks {
                    println!("ID: {} | {}", task.id, task.title);
                }

                println!("\nEnter task ID to change priority:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Reading error");

                let id: i32 = match input.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid ID!");
                        continue;
                    }
                };

                if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                    println!("New priority (1=Low, 2=Medium, 3=High):");
                    let mut prio = String::new();
                    io::stdin().read_line(&mut prio).expect("Reading error");

                    let new_priority = match prio.trim() {
                        "1" => Priority::Low,
                        "2" => Priority::Medium,
                        "3" => Priority::High,
                        _ => Priority::Medium,
                    };

                    task.update_priority(new_priority);
                    println!("Priority updated successfully!");
                } else {
                    println!("Task not found.");
                }
            }
            5 => {
                if tasks.is_empty() {
                    println!("No tasks to delete.");
                    continue;
                }

                for task in &tasks {
                    println!("ID: {} | {}", task.id, task.title);
                }

                println!("\nEnter task ID to delete:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Reading error");

                let id: i32 = match input.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid ID!");
                        continue;
                    }
                };

                if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                    let removed = tasks.remove(pos);
                    println!("Task '{}' deleted successfully.", removed.title);
                } else {
                    println!("Task with ID {} not found.", id);
                }
            }
            0 => {
                save_tasks(&tasks);
                println!("Thanks, Good bye!!!");
                break;
            }
            _ => println!("Invalid choice"),
        }
        if choice != 0 {
            save_tasks(&tasks);
        }
    }
}
