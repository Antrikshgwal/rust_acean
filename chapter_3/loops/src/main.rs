// loop, while and for
// loop label - we can specify which loop we have to exit by labeling the loop, loop labels start with ' .

fn main() {
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");
// let mut number = 3 ;
//     loop{
//         if(number == 0 ){
//             break;
//         }
// println!{"number : {number}"};
// number -= 1 ;

//     }
//     println!("LIFTOFF!");


// Fibbonaci sequence

let  n = 45 ;



   println!("Fibonacci({}) = {}", n, fibbonaci(n));
}

fn fibbonaci(val : u32) -> u32{

     if val == 0 {
        return 0 ;

    }else if val == 1{
return 1;
    }
let mut count = 2 ;
let mut  a = 0;
let mut  b = 1;
while count <= val {

    let temp = b;
    b = b+a ;
    a = temp;
    count += 1;
}
b
}