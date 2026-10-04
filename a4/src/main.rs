use std::env;

fn main() {
    //holt cmd argumente
    let cmd_args: Vec<String> = env::args().collect();

    let mut n: u64 = match cmd_args[1].parse() {
        Ok(n) if n > 0 => n,
        _ => {
            println!("posivie zahl benötigt");
            return;
        }
    };

    println!("{n}");

    while n != 1 {
        if n % 2 == 0 {
            n = n / 2
        } else {
            n = 3 * n + 1
        }

        println!("{n}");
    }
}