use std::io;

fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let fib_num = loop {
        let mut nth_number = String::new();

        println!();
        println!("Input a number to find the fibonacci output");
        io::stdin()
            .read_line(&mut nth_number)
            .expect("That was not valid");

        match nth_number.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };

    println!("{}", fibonacci(fib_num));
}
