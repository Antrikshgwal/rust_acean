fn main (){
    let strng = String::from("Finished");
    let result = first_letter(&strng);
}

fn first_letter(s: &String){
    let bytes = s.as_bytes() ; // as_bytes is an read only method which reads the underlying bytes of the s

    for (i , item) in bytes.iter().enumerate(){
        if *item == b' ' {
i
        }
    }
}



