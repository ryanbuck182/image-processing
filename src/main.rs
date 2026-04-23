use image_processing::benchmark::{full_benchmark, parallel_benchmark, sequential_benchmark, parallel_2_benchmark};
const K_VALUE: usize = 3;

fn main() {
    benchmark_menu();
}

fn benchmark_menu() {
    loop{
        println!("Benchmark menu:");
        println!("1. Run full benchmark");
        println!("2. Run sequential benchmark");
        println!("3. Run parallel benchmark (rayon)");
        println!("4. Run parallel benchmark (threadpool)");
        println!("5. Exit");
        println!("Enter your choice: ");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim().parse::<u32>().expect("Invalid input");
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
    
}

