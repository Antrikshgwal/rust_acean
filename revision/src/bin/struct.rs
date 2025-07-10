// calculating area of rectangle while using structs

struct Rect {
    width:u32 ,
    height:u32,
}

fn main(){
    let rect = Rect{
        width: 4,
        height: 5
    };

    println!{
        "Area is {}", area(&rect)
    };
}

fn area(s:&Rect) -> u32{
    s.width*s.height
}