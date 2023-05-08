

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        // method body would be defined here
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddr{
        kind: IpAddrKind::V4(127, 0, 0, 1), // or IpAddrKind::V4(127, 0, 0, 1
        address: String::from("127.1.1.1"),
    };

    let localhost = IpAddrKind::V4(12, 0, 0, 1);


    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x = 5;
    let y = Some(5);




    
    //let sum = x + y; // error: mismatched types
    let sum = x + y.unwrap_or(0); // unwrap returns the value inside the Some variant
    println!("Sum: {}", sum);

    let val = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value: {}", val);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value{
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("not one of the expected values"),
    }

    let some_value = Some(3);
    if let Some(3) = some_value{ //in if let you only hav to use the pattern you care about not the else case
        println!("three");
    }





}

fn route(ip_kind: IpAddrKind){

}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        Some(i) => Some(i + 1),
        _=> None,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1 // return value
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state); // state is the value inside the Coin::Quarter variant
            25
        },
    }
}