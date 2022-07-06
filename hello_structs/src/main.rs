struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width = self.width / 2;
        self.width * self.height
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn test_struct() {
    let user1 = User {
        email: String::from("xbkaishui@126.com"),
        active: false,
        username: "xbkaishui".to_string(),
        sign_in_count: 10,
    };
    println!("test user1 {}", user1.username);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("test user1 {}", user1.email);
}

/**
 * We’ve chosen &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
 */
fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

fn test_rect() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("The rectangle is {:#?}", rect1);

    let mut rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("rect hold {}", rect1.can_hold(rect2));

    println!("The square rectangle is {:#?}", Rectangle::square(12));

}


fn main() {
    // test_struct();
    test_rect();
}
