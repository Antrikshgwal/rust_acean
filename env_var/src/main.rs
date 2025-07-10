use std::env;
use dotenv::dotenv;

fn main(){
    // let key = "AAA";

    // env::set_var(key, "123");

    // env::remove_var(key);

    // let cli_arg = env::var("CLI_ARG"); // env::var() sets environment variable form CLI

    // match cli_arg {
    //     Ok(val) => println!("CLI_ARG:{:?}", val),
    //     Err(e) => println!("Error CLI_ARG: {}", e),
    // }

    dotenv().ok();

    let api_key = env::var("API_KEY");

    match api_key {
        Ok(val) => println!("API_KEY : {:?}", val) ,
        Err(e) => println!("ERROR API_KEY : {}", e)
    }

    println!("..program continues...");
}