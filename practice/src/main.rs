// // TO-DO list
use std::io;
// fn main() {
//     let mut user_input  = String::new();
//     let mut tasks: Vec<String> = Vec::new();
//     println!("Welcome to the To-Do List");
//     loop {
//     println!("What do you want? 1.press A to add task 2.Press R to read tasks 3.Press Q to quit 4 Press E to remove task");
//     std::io::stdin()
//     .read_line(&mut user_input)
//     .expect("Failed to read the line");
//      user_input = user_input.trim().to_uppercase();


// if user_input == "A" {
// println!("Enter Task:");
// let mut task = String::new();
// std::io::stdin().read_line(&mut task).expect("Failed to read task");
// tasks.push(task.clone());
// println!("Task added: {}", task);
// }else if user_input == "R" {
//     for task in tasks.iter(){
//         println!("Task: {}", task);
//     }
// }else if user_input == "Q" {

//     break
// }else if user_input == "E" {
//     println!("Enter the task to remove:");
//     let mut task_to_remove = String::new();
//     std::io::stdin().read_line(&mut task_to_remove).expect("Failed to read task");
//      task_to_remove = task_to_remove.trim();
//      tasks.pop();
//      println!("Task removed: {}", task_to_remove);

// }
// else if user_input == "Q" {
//     println!("Exiting the program");
//     break;
// }
// else {
//     println!("Invalid input");}
//     // println!("{}", user_input);
//     user_input.clear();
//     }
// }


// Banking System

impl Account {
    fn add_customer(&mut self /* Pointer to the customer struct */) -> bool{

    self.acc_No = acc_no.trim().to_string();
    self.name = name.trim().to_string();
    self.balance = 0; // Initialize balance to 0
    accounts.push(self.clone());
    println!("Account created successfully for {} with account number {}", self.name, self.acc_No);

        true

    }

    fn check_balance(&self){
        for account in accounts.iter_mut(){
            if account.acc_no == *acc_no {

                println!("Your balance is {}", account.balance);
                return;
            }
          }
    }

    fn withdraw(&self , acc_no: &String, amt : u32){


        for account in accounts.iter_mut(){
            if account.acc_no == *acc_no {

                if account.balance < amt {
                    println!("Not enough balance");
                    return
                }
                account.balance -= amt;
                println!("Withdrawn {} to account number {}. New balance is {}", amt, acc_no, account.balance);
                return;
            }
          }
    }
    fn deposit(&self, acc_no: &String, amt : u32){
      for account in accounts.iter_mut(){
        if account.acc_no == *acc_no {
            account.balance += amt;
            println!("Deposited {} to account number {}. New balance is {}", amt, acc_no, account.balance);
            return;
        }
      }
    }
}
struct Account{
    acc_No : String,
    name : String,
    balance : u32
}

fn main(){


    let mut accounts : Vec<Account> = Vec::new();

    println!("Welcome to Bank!!")
    loop{
    let mut user_input = String::new();
    let mut acc_no = String::new();
    let mut amt = String::new();
    println!("1. Press O to open account 2. Press D to deposit 3. Press W to withdraw 4. Press M to check Balance 5. Press Q to quit");
    std::io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read the line");
    if user_input == "O"{
        println!("Enter account number");
        std::io::stdin()
        .read_line(&mut acc_no)
        .expect("Failed to read the line");
        println!("Enter name");
        std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read the line");
        let account = Account{
            acc_no = acc_no,
            name = name,
            balance = 0
        }
        add_customer(&mut account);
    }else if user_input == "D" {
        println!("Enter account number");
        std::io::stdin()
        .read_line(&mut acc_no)
        .expect("Failed to read the line");
        println!("Enter amount to deposit");
        std::io::stdin()
        .read_line(&mut amt)
        .expect("Failed to read the line");
        let amount: u32 = amt.trim().parse().expect("Please enter a valid number");
        deposit(&acc_no, amount);
    } else if user_input == "M" {
        println!("Enter account number");
        std::io::stdin()
        .read_line(&mut acc_no)
        .expect("No account found")
        check_balance(&mut acc_no)
    }else if user_input == "W" {
        println!("Enter account number");
        std::io::stdin()
        .read_line(&mut acc_no)
        .expect("Failed to read the line");
        println!("Enter amount to deposit");
        std::io::stdin()
        .read_line(&mut amt)
        .expect("Failed to read the line");
        let amount: u32 = amt.trim().parse().expect("Please enter a valid number");
        withdraw(&acc_no, amount);
    }


    }
}