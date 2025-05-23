// TO-DO list
use std::io;
fn main() {
    let mut user_input  = String::new();
    let mut tasks: Vec<String> = Vec::new();
    println!("Welcome to the To-Do List");
    loop {
    println!("What do you want? 1.press A to add task 2.Press R to read tasks 3.Press Q to quit 4 Press E to remove task");
    std::io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read the line");
     user_input = user_input.trim().to_uppercase();


if user_input == "A" {
println!("Enter Task:");
let mut task = String::new();
std::io::stdin().read_line(&mut task).expect("Failed to read task");
tasks.push(task.clone());
println!("Task added: {}", task);
}else if user_input == "R" {
    for task in tasks.iter(){
        println!("Task: {}", task);
    }
}else if user_input == "Q" {

    break
}else if user_input == "E" {
    println!("Enter the task to remove:");
    let mut task_to_remove = String::new();
    std::io::stdin().read_line(&mut task_to_remove).expect("Failed to read task");
     task_to_remove = task_to_remove.trim();
     tasks.pop();
     println!("Task removed: {}", task_to_remove);

}
else if user_input == "Q" {
    println!("Exiting the program");
    break;
}
else {
    println!("Invalid input");}
    // println!("{}", user_input);
    user_input.clear();
    }
}