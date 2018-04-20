// rustc -C opt-level=3 fib.rs

use std::env;


fn fib(i: u64) -> u64 {
    if i < 2 {
        1
    } else {
        fib(i - 1) + fib(i - 2)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: fib integer");
        return;
    }

    match args[1].parse::<u64>() {
        Ok(i) => {
            println!("{}", fib(i));
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
}
