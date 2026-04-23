use image_processing::benchmark::{
    full_benchmark, parallel_2_benchmark, parallel_benchmark, sequential_benchmark,
};
use std::io::{self, Write};

const K_VALUE: usize = 3;

fn main() {
    benchmark_menu();
}

fn benchmark_menu() {
    loop {
        println!("Benchmark menu:");
        println!("1. Run full benchmark");
        println!("2. Run sequential benchmark");
        println!("3. Run parallel benchmark (rayon)");
        println!("4. Run parallel benchmark (threadpool)");
        println!("5. Exit");
        let choice = input_int("Enter your choice:", 1, 5);

        match choice {
            1 => {
                full_benchmark(K_VALUE);
            }
            2 => {
                sequential_benchmark(K_VALUE);
            }
            3 => {
                parallel_benchmark(K_VALUE);
            }
            4 => {
                parallel_2_benchmark(K_VALUE);
            }
            5 => break,
            _ => println!("Invalid choice"),
        }
    }

    println!("Exiting...")
}

fn input_int(prompt: &str, min: i32, max: i32) -> i32 {
    let input = take_input(prompt);

    // Parse input
    let num: Result<i32, _> = input.parse();

    // Return int or try again
    match num {
        Ok(n) => {
            if n >= min && n <= max {
                n
            } else {
                println!(
                    "Invalid input! Please enter an integer between {} and {}.",
                    min, max
                );
                input_int(prompt, min, max)
            }
        }
        Err(_) => {
            println!("Invalid input! Please enter an integer.");
            input_int(prompt, min, max)
        }
    }
}

fn take_input(prompt: &str) -> String {
    // Prompt the user
    print!("{} ", prompt);
    io::stdout().flush().expect("Unable to print prompt.");

    // Take input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Trim and return
    input.trim().to_string()
}
