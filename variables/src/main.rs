fn test() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
fn test_2() {
    let num = 5;
    if num < 10 {
        println!("The num is less than 10");
    } else {
        println!("The num is greater than 10");
    }
}

fn test_loop() {
    let mut counter = 5;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("The counter is: {}", counter);
    println!("The result is: {}", result);

    let a = [10, 20, 30, 40, 50];
    for ele in a {
        println!("value {}", ele);
    }
}

fn test_scope() {
    {
        let s = "hello world";
        println!("The scope is: {}", s);
    }

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("scope s is {}", s);

}

fn main() {
    //   test();
    // test_2();
    // test_loop();
    test_scope()
}
