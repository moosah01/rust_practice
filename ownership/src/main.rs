fn main() {
    // ------ Ownership Rules------
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {// s is not valid here as it is not declared
        let s = "hello"; // s is valid from this point forward
        //this a string literal and is stored in binary, it is fixed and cannot be mutated
    } // this scope is now over, and s is no longer valid

    {
        let s = String::from("hello"); // s is valid from this point forward
        //it is a dynamic string and is stored in heap, it can be mutated
    }

    let x = 5;
    let y = x; // x is COPIED to y, both are valid

    let s1 = String::from("hello"); // => declared in heap
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}, world!", s1); // this will throw an error will say s1 has been moved

    //SHALLOW COPY
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 is cloned to s2, both are valid
    println!("s1 = {}, s2 = {}", s1, s2);

    let mut s = String::from("hello"); // s comes into scope
    s = takes_ownership(s); // s's value moves into the function...
    println!("{}", s); // this will throw an error will say s has been moved

    // to fix it, you add the line at the end of the fn takes_ownership to return the value

    let x = 5;
    makes_copy(x); // x would move into the function,
    println!("{}", x); // but i32 is Copy, so it’s okay to still use x afterward

    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1); // s1 is moved into takes_and_gives_back, which also moves its return value into s2
    println!("{}", s2);

    // ------ References and Borrowing ------

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // s1 is moved into calculate_length, which moves its return value into s2
    println!("By borrowing ownership -> The length of '{}' is {}.", s2, len);


    //now with borrowing
    let s1 = String::from("hello");
    let len = calculate_length_borrow(&s1); // s1 is borrowed, so it is still valid
    println!("By Sending Reference -> The length of '{}' is {}.", s1, len);


    // ------ Mutable References ------
    let mut s = String::from("hello");
    change(&mut s); // s is borrowed as mutable, so it is still valid
    // you can not borrow a mutable reference more than once in a particular scope
    // to avoid race conditions

    // you cannot have a mutable reference while you have an immutable one
    // you can have multiple immutable references in one scope

    let r1 = &mut s;
    // let r2 = &mut s; // this will throw an error
    println!("{}", r1);


    let r1  = &s;
    let r2  = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s; // if you were to shift this line above before r1 it wouldnt work
    //b/c r1 and r2 are out of scope
    println!("{}", r3);

    // ------ Dangling References ------

    // The Rules of References
    // 1. At any given time, you can have either one mutable reference 
    //    or any number of immutable references.
    // 2. References must always be valid.


    // ------ The Slice Type ------
    let mut s = String::from("Moosa Hashim");

    let moosa = &s[..5]; // this is a slice of the string last is exclusive
    let hashim = &s[6..]; // this is a slice of the string last is exclusive

    let word = first_word(&s); // word will get the value 5
    println!("{}", word);
    s.clear(); // this empties the String, making it equal to ""


    let s2 = "Moosa Hashim";
    let word = first_word_splice(s2);
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // this is a slice of the array
    println!("{:?}", slice);
}


fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string // some_string is returned and moves out to the calling function
} // some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
} // Here, `a_string` goes out of scope and `drop` is called. The backing memory is freed.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_length_borrow(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to array of bytes
    for (i, &item) in bytes.iter().enumerate() { // iterate over the array of bytes
        if item == b' ' { // if the item is an empty space
            return i; // return the index
        }
    }
    s.len() // if no space is found, return the length of the string
}

fn first_word_splice(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert string to array of bytes
    for (i, &item) in bytes.iter().enumerate() { // iterate over the array of bytes
        if item == b' ' { // if the item is an empty space
            return &s[..i]; // return the slice of the string from 0 to i
        }
    }
    &s[..] // if no space is found, return the slice of the entire string
}