struct User{
    username: String,
    email: String,
    age: u8,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // to print the struct
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    fn perimeter(&self) -> u32{
        2 * (self.width + self.height)
    }
    fn area(&self) -> u32{ //first argument is always self
        self.width * self.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User{
        username: String::from("moosah01"),
        email: String::from("moosah01@gmail.com"),
        age: 22,
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username; 
    println!("Username: {}", name);

    user1.username = String::from("Moosa Hashim");
    
    let user2 = build_user(
        String::from("hello@gmail.com"),
        String::from("hello"),
    );

    let user3 = User{
        email: String::from("moeen@gmail.com"),
        username: String::from("moeen"),
        ..user2 // copy remaining fields from user2
    };
    
    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {:#?}", rect); // to print the struct
    println!("Area of rectangle is: {}", area(&rect));

    let rect1 = Rectangle{
        width: 60,
        height: 70,
    };
    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    println!("Perimeter of rect 1 is  {}", rect1.perimeter());

    let rect3 = Rectangle::square(20);
    println!("Area of rect3 is {}", rect3.area());
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        age: 22,
        sign_in_count: 1,
        active: true,
    }
} 

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}