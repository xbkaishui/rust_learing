pub mod lib;
use crate::lib::*;

fn find_max_num(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
}

fn largest<T: PartialOrd + Copy >(list: &[T]) -> T {
    let mut largest = list[0];
    for &number in list {
        if number > largest { largest = number;}
    }
    largest
}

fn test_tweet() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);
}

fn test_lifetime() {
    {
        let mut r;

        {
            let x = 5;
            r = &x;
        }
       
        println!("r: {}", r);
    }

}

fn main() {
    // find_max_num();
    // test_tweet();
}
