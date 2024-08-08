
fn first_word(text : &String) -> &str{
    let bytes = text.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &text[..i];
        }
    }
    return text;
}
fn main() {
    let text = String::from("shubh sharma");
    println!("{}",first_word(&text)); 
}