#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

fn test_enum() {
    let v4 = IpAddrKind::V4; 
    println!("{:#?}", v4);

    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    println!("{:#?}", some_number);
    println!("{:#?}", absent_number);

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_coin(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("pennry");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10, 
        Coin::Quarter => 25
    }
}

fn main() {
    // test_enum();
    let coin = Coin::Penny;
    let value = value_in_coin(coin);
    println!("value is {}", value);
}
