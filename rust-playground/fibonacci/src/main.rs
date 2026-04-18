use std::io;

fn main() {
    println!("=== Fibonacci number calculator ===");
    println!("Please type a natural number N to obtain the Fibonacci sequence up to the N-th number.");

    let mut N = String::new();
    io::stdin().read_line(&mut N).expect("Failed to read line");
    let N: u64 =  N.trim().parse().expect("Failed to parse string to integer");

    let (mut a, mut b) = (0u64, 1u64);
    for i in 0..=N
    {
        println!("F({i}) = {a}");
        (a, b) = (b, a + b);
    }
    println!("Done");
}
