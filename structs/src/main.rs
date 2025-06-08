use std::fs::File;
// use std::io::ErrorKind;
use std::io::{self,Read};
fn main() {

    // get_username_from_file();
// let greeting_file_result = File::open("hello.txt");
// match greeting_file_result {
//     Ok(file) => file,
//     Err(error) =>  match error.kind() {
//         ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc ,
//             Err(error) => panic!("Error creating file {e:?}"),
//         },
//         _ => {
//             panic!("Problem opening the file : {error:?}")
//         }
//     }
// }

// let greeting_file_result = File::open("hello.txt").expect("Problem opening the file");

}

pub trait Summary {
    fn Summarize(&self) -> String;
    
}

// fn get_username_from_file() -> Result<String, io::Error> {
//     let username_result = File::open("hello.txt");

//     let mut username_file = match username_result {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(error) => Err(error),
//     }
// }