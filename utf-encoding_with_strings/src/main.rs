fn main() {
    let mut s1 = String::from("Mera");
    let mut s2 = String::from(" Khel");
    let s3 = String::from(" Khatam");
    let s4 = String::from(" hai");
// s1.push_str(&s2);
let s = format!{"{s1} {s2} {s3} {s4}"};
    println!("{s}");
}
