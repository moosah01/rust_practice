fn main() {
    
    let n = 10;
    let fib_n = fibonacci(n);
    println!("The {}th Fibonacci number is {}", n, fib_n);

    let n = 20;
    let fib_dyn_n = fibonacciDyn(n);
    println!("The {}th Fibonacci number via dyn programming is {}", n, fib_dyn_n);


    let lyrics = generate_lyrics();
    println!("{}", lyrics);

}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn fibonacciDyn(n: u32) -> u32 {
    let mut memo = vec![0; n as usize + 1];
    if n <=1 {
        return n;
    }

    memo[0] = 0;
    memo[1] = 1;

    for i in 2..=n as usize {
        memo[i] = memo[i-1] + memo[i-2];
    }

    memo[n as usize]
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn generate_lyrics() -> String {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves, and",
        "three French hens,",
        "four calling birds,",
        "five golden rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,"
    ];

    let mut lyrics = String::new();

    for i in 0..12 {
        lyrics += &format!("On the {} day of Christmas,\nMy true love gave to me:\n", days[i]);
        for j in (0..i+1).rev() {
            if j == 0 && i != 0 {
                lyrics += "And ";
            }
            lyrics += gifts[j];
            if j != 0 {
                lyrics += "\n";
            }
        }
        lyrics += "\n";
    }

    return lyrics;
}