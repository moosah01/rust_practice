fn main() {
    println!("Hello Welcome to Rust Programming Language");
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let x = 5; // to be used for shadowing
    println!("The value of x is: {}", x);
    let x = x + 1; //shadowing the prev variable
    println!("The value of x is: {}", x);

    //SCALAR Datatypes

    //Integers
    let x = 2; //default i32
    let y: u32 = 3; //explicitly defining the datatype
    let a = 98_222; //default i32 // Decimal
    let b = 0xff; //default i32 // Hex
    let c = 0o77; //default i32 // Octal
    let d = 0b1111_0000; //default i32 // Binary
    let e = b'A'; //default u8 // Byte (u8 only)

    let f: u8 = 255; //u8 only => used for wrapping

    //Floating-point numbers   
    let g = 2.0; //default f64
    let h: f32 = 3.0; //explicitly defining the datatype

    //Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    //Booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    //Character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; //unicode scalar value

    //COMPOUND Datatypes

    //Tuple
    let tup1: (i32, f64, u8) = (500, 6.4, 1); //explicitly defining the datatype
    let tup2 = ("Moosa Hashim Pullup Count", 300); //implicit datatype
    let (x, y, z) = tup1; //destructuring the tuple

    let val1 = tup1.1;
    let val2 = tup2.0;
    println!("The value of val1 is: {}", val1);
    println!("The value of val2 is: {}", val2);

    //Array => fixed length
    let error_codes = [404, 500, 200, 201, 202, 203, 204, 205, 206, 207];
    let not_found = error_codes[0];

    let byte = [0;8]; //create an array of size 8 and initialise all with value 0
    println!("The value of byte is: {}", byte[0]);

    my_function();
    let sum = my_function2(11, 22);
    println!("The value of sum is: {}", sum);

    //Control Flow
    let number = 5;
    if number < 10 {
        println!("condition was true");
    } else if number < 22{
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number  = if condition { 5 } else {6} ; 
    println!("The value of number is: {}", number);


    // COntrol Flow Loops
    let mut counter = 0;
    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter; // adding counter after break will allow you to return counter
        }
    };

    println!("The value of result is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFT OFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is {}", element);
    }

    for number in 1..4 { //last number is exclusive
        println!("{}!", number);
    }

    



}


fn my_function() {
    println!("Hello from my_function")
}

fn my_function2(x: i32, y: i32) -> i32{
    println!("Hello from my_function2");
    println!("The sum of your numbers is {}", x+y);
    let sum = x+y;
    return sum;
}