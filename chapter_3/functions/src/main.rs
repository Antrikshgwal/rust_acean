fn main(){
    println!("This is an function");
    // print_labeled_measurment(56,'f');
    let x = 3 ;
    let y = plus_one(x) ; // not mutating but binding for the first time
    println!("{y}");
}
// fn print_labeled_measurment (value:i32, unit_label : char){
//     println!("Labeled measurment is:{value} {unit_label}");
// }

// Statements and expressions
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.
// ; => converts a expression into a statement

fn plus_one(value:u32) -> u32{
value+1
}