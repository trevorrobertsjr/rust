fn main() {
    let s = String::from("hello world");
    // let word = first_word(&s);
    let word = &s[..first_word(&s)];
    println!("{}", word);
    println!("{}", first_word_str(&s));
    

}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // println!("{}",bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_str(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}