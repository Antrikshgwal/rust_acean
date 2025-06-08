fn main() {
    // enum Transaction {
    //     Pending(String),
    //     Successful(String),
    //     Failed(String)
    // }
    // let upi = Transaction::Pending(String::from("pending"));

    // match upi {
    //     Transaction::Pending(status) => println!("Transaction status: {}", status),
    //     Transaction::Successful(status) => println!("Transaction status: {}", status),
    //     Transaction::Failed(status) => println!("Transaction status: {}", status),
    // }

    enum USstate {
        Alabama,
        Alaska
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(USstate),
    }
}

fn value_in_cents(coin:Coin)-> u8 {
    match coin {
        Coin::Penny => 1,
        Coin:: Nickel => 5,
        Coin::Dime => 10,
        Coin:: Quarter(state)=>{
println!("The coin has state {state:?}");
        25} ,

    }
}

