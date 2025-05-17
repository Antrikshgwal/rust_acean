// // Ownership: The set of rules that govern how a rust program manages memory
// // Rules : 1. Every Rust value has an owner
// //         2. There can be only one owner at a time.
// //         3. When the scope of an owner goes out, its value gets dropped.


// fn main() {
//     // Variable Scope
// // s not valid, since not initialized yet
//    // let s = "coward"  // s valid from now on

// // Using String() type for understanding ownership
//    let mut s = String:: from("coward");
//    s.push_str("ice");
//       //    println!("{s}");

//   //  let s1 = String::from("hello!");
//    // let s2 = s1 ; // this will throw an error as copying is not allowed for heap data, as it only copies the pointer not the actual data, also this will also lead to double free error as s1 and s2 both points to the same memory location

//   //  let s2  = s1.clone();



//   //  println!("s1= {s1}, s2 = {s2}"); // moving s1 to s2

// //    fn main() {
// //     let s = String::from("hello");  // s comes into scope

// //     takes_ownership(s);            // s's value moves into the function...
// //                                     // ... and so is no longer valid here
// // println!(" s: {s}");
// //     let x = 5;                      // x comes into scope

// //     makes_copy(x);                  // because i32 implements the Copy trait,
// //                                     // x does NOT move into the function,
// //     println!("{}", x);              // so it's okay to use x afterward

// // } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// // fn takes_ownership(some_string: String) { // some_string comes into scope
// //     println!("{some_string}");
// // } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// // fn makes_copy(some_integer: i32) { // some_integer comes into scope
// //     println!("{some_integer}");
// // } // Here, some_integer goes out of scope. Nothing special happens.

// let mut s3 = String::from(" This is s3");
// // let len = calculate_length(&s3); // Here we are referencing s3, by referencing we can access the value of the variable without moving or changing ownership

// // Attempting to referencing same variable s3 more than once

// let r1 = &mut s3 ; // r1 referencing s3

// let r2 = &mut s3 ; // r2 tries to reference s3
// println!{"{}", r1};// r1 is used here so r2 cannot reference s3

// println!{"{}", r2};


// println!("{s3}")


// } // s again not valid as scope is over

// // fn calculate_length(val : &mut String) -> usize {
// //   return val.len()
// // }
// // fn change(val: &mut String){
// //   val.push_str(" first attempt to change s3 ");
// // }
// // fn ekaurchange(val : &mut String){
// //   val.push_str(" Im trying to change s3");
// // }

// fn main(){
//   // s ka khel shuru bhi nhi hua
//   let reference_to_nothing = dangle()
//   println!("{}", reference_to_nothing);

// }
// fn dangle()-> &String {  // dangle returns a refernce to a string
//   let s = String :: from("S ka Khel shuru");
//   &s // returning reference to s
// } // but s ka khel khatam, value of s is dropped



fn main (){
  let  s1 = String::from("Pretty little baby");
  let res = return_first_word(&s1);
  println!("{res}");
}
fn return_first_word(val : &String) -> &str{
  let bytes = val.as_bytes();
   for(i, &item) in bytes.iter().enumerate(){
    if item == b' '{
      return &val[0..i];
    }
   }
   &val[..]
  }