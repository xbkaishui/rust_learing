fn string_memory_test() {
    let s = String::from("hello world");
    println!("{}", s);
    let s2 = s;
    println!("{}", s2);
    let len = calculate_length(&s2);
    println!("data {}, len {}", s2, len);

    let mut str = String::from("hello bye ");
    change(&mut str);
    println!("after change {}", str);
}

// by inference, not take ownership 
fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(some_str: &mut String) {
    some_str.push_str(".world");
}

fn find_word(s: & String, word: &String) -> String {
    let t = "hello";
    let result = String::from("haha") ;
    return result;
}

fn main() {
    string_memory_test();
}
