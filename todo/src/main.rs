use std::io;

enum Status {
    InProgress,
    Completed,
}
enum Priority {
    Low,
    Medium,
    High,
}

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
        description: String,
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

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id = 1;

    println!("{:=^30}","TODO List APP");

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
                println!("Enter the description(optionnal)");
            }

            0 => {
                println!("Thanks, Good bye!!!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
