struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr.overflowing_add(self.next);
        
        if new_next.0 < self.next {
            return None;  // if overflow occurred return None
        }

        self.curr = self.next;
        self.next = new_next.0;

        Some(self.curr)
    }
}

struct Collatz {
    current: Option<u64>,
}

impl Collatz {
    fn new(start: u64) -> Collatz {
        Collatz { current: Some(start) }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let ret = self.current;
        self.current = self.current.and_then(|curr| {
            if curr == 1 {
                None
            } else if curr % 2 == 0 {
                Some(curr / 2)
            } else {
                Some(3 * curr + 1)
            }
        });
        ret
    }
}

fn main() {
    let fib = Fibonacci::new();
    for i in fib.take(10) {
        println!("{}", i);
    }

    println!("break");
    println!("break");

    let collatz = Collatz::new(12);
    for i in collatz {
        println!("{}", i);
    }
}