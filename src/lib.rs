use std::fs::{self, OpenOptions};
use std::io::Write;

pub fn add(task: &str) {
    let mut file = OpenOptions::new().write(true).append(true).create(true).open("todo.txt").expect("Something went wrong went creating `todo.txt` before adding the give task.");
    if let Err(e) = writeln!(file, "{}", task) {
        eprintln!("This error is expected when adding any tasks to the todo.txt, here is the error: {}", e);
    }        
}
    
pub fn list() {
    cat("todo.txt");
}

pub fn update(id: &str) {
    println!("The function is being implemented. The {} is not updated yet", id);
} 

pub fn delete(id: usize) {
    let content = fs::read_to_string("todo.txt").expect("Can't open the file");
    let updated = content.lines().enumerate().filter_map(|(n, s)| (n != id)
        .then_some(s.to_string() + "\n")).collect::<String>(); 
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("todo.txt")
        .expect("Something went wrong when creating `todo.txt` before adding the given task.");
    file.write_all(updated.as_bytes()).expect("This is expected");
}
    
pub fn clear() {
    fs::remove_file("todo.txt").expect("Can't clear the todo.txt");
}
// Preview file's content.
fn cat(filename: &str) {
    fs::read_to_string(filename).expect("Unable to list all the tasks. \n If you haven't added any tasks, please add any before proceeding").lines().enumerate().for_each(|(num, line)| println!("{} {}", num, line));
}

pub fn help() {
    println!(r#"
You should write a task like this
`[YOUR TASK] +[PROJECT'S or GROUP'S NAME]` 
To follow the `todo.txt` convention. And you can quickly sort the task list based on the GROUP'S name.

Usage "todotxt-rs [OPTION] [TASK]"
    - add: Add [TASK] to the `todo.txt` file
    - list: List all tasks.
    - del: Delete the given task
        "#)
}
