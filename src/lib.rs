use std::fs::OpenOptions;
use std::io::Write;

pub struct Todo {
    name: String,
    project: String
}

impl Todo {
    pub fn new(name: String, project: String) -> Self{
        Todo { name: name, project: project }
    }
    pub fn add(task: &str) {
        let mut file = OpenOptions::new().write(true).create(true).open("todo.txt").expect("Something went wrong went creating `todo.txt` before adding the give task.");
        if let Err(e) = writeln!(file, "{}", task) {
            eprintln!("This error is expected when adding any tasks to the todo.txt, here is the error: {}", e);
        } 
    }
    
    pub fn list() {
        cat("todo.txt");
    }
    
    
}


// Preview file's content.
fn cat(filename: &str) {
    
}

pub fn help() {
    println!(r#"
Usage "todotxt-rs [OPTION] [TASK]"
    - add: Add [TASK] to the `todo.txt` file
    - list: List all tasks.
    - del: Delete the given task
        "#)
}
