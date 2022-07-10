fn test_vec() {
    let mut vec: Vec<u32> = Vec::new();
    vec.push(12);
    vec.push(13);
    vec.push(14);
    println!("vec: {:?}", vec);

    let second = &vec[1];
    println!("second {} ", second);
    vec.push(15);
}

fn test_borrow() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    println!("The first element is: {}", first);

    v.push(6);
    // can't use to print 
    // println!("The first element is: {}", first);
}

fn test_string() {
    let s = "init contents";
    let s = s.to_string();
    println!("{}", s);

    let hello = "Здравствуйте";
    let answer = &hello[0..4];
    println!("{}", answer);

}

fn test_map() {
    use std::collections::HashMap;
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("map {}", map);
    // println!("field_name {}", field_name);

}

fn main() {
    // test_vec();
    // test_borrow();
    test_map();
}
