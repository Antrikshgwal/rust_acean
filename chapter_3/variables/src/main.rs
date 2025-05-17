fn main() {
//     let  x = 5;
//     let x = x+1;
//     println!("The value of x is: {x}");
//     {
//         let x = x*6;
//         println!("The value of inner scope x is: {x}");
//     }
//     println!("The value of x is: {x}");

// const - immutable at all times, its type needs to be annotated always and must be in uppercase

const GUESS : u32 = 98 ;
println!("{}",GUESS);

// Shadowing and variable mutation

//  let spaces = "   ";   This is allowed as we shadowing a immutable variable
//     let spaces = spaces.len();


    // let mut spaces = "   ";   this is not allowed as we are mutating a string to number u32
    // spaces = spaces.len();


}