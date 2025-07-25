use std::io;

#[derive(Debug)]
struct Task{
    task_name: String,
    task_status: Status,
}
#[derive(Debug)]
enum Status {
    Pending,
    InProgress,
    Completed,
}
fn main(){
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        println!("Enter task name (or 'exit' to quit):");
        let mut task_name = String::new();
        io::stdin().read_line(&mut task_name).expect("Failed to read line");
        let task_name = task_name.trim();

        
        if task_name == "exit" {
            break;
        }
        println!("Task Status (Pending->1, InProgress->2, Completed->3):");

        let mut task_status = String::new();
        io::stdin().read_line(&mut task_status).expect("Failed to read line");
        let task_status = task_status.trim();

        let status = match task_status {
            "1" => Status::Pending,
            "2" => Status::InProgress,
            "3" => Status::Completed,
            _ => {
                println!("Invalid status. Defaulting to Pending.");
                Status::Pending
            }
        };

        let task = Task {
            task_name: task_name.to_string(),
            task_status: status,
        };
        
        tasks.push(task);
        println!("Task '{}' added.", task_name);
    }
    println!("Tasks added: {:?}", tasks);
    for task in tasks {
        println!("Task Name: {}, Status: {:?}", task.task_name, task.task_status);
        match task.task_status {
            Status::Pending => println!("This task is pending."),
            Status::InProgress => println!("This task is in progress."),
            Status::Completed => println!("This task is completed."),
        }
    }
}