fn string_memory_test() {
    let s = String::from("hello world");
    println!("{}", s);
    let s2 = s;
    println!("{}", s2);
    let len = calculate_length(&s2);
    println!("data {}, len {}", s2, len);
}

// by inference, not take ownership 
fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn main() {
    string_memory_test();
}
