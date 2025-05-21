

fn main() {
    let rect1 = Rect{
        width:45,
        length:50,
    };


    // println!("Area of the rectangle is : {} ",
    //     area(&rect1)
    // );

    println!("rect1 is {rect1:#?}")
}
#[derive(Debug)]
struct Rect {
    width:u64,
    length:u64,
}
fn area(rectangle : &Rect) -> u64 {
rectangle.width * rectangle.length
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length:u32
}

impl Rectangle {
    fn area(&self)-> u32{
        self.width * self.length
    }
    fn can_hold(&self, rect : &Rectangle)-> bool{
        self.width > rect.width && self.length > rect.length
    }
}
fn main(){
    let rect1 = Rectangle{
        width:20,
        length:30
    };
    let rect2 = Rectangle{
        width:25,
        length:25
    };
    let rect3 = Rectangle{
        width:2 ,
        length:4
    };

    // println!("The area is {} ",
    // rect1.area());
    // Self keyword is now use in the context of the struct

    println!{"rect1 can contain rect2? {}", rect1.can_hold(&rect2)};
    println!{"rect1 can contain rect3? {}", rect1.can_hold(&rect3)};
}