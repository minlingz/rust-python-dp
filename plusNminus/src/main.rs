use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut sum = 0;
    for _j in 0..10000 {
        
        for i in 0..10000 {
            sum += i;
        }
        for i in 0..10000 {
            sum -= i;
        }
    }
    println!("Result: {}", sum);
    let duration = start.elapsed(); 
    println!("Rust elapsed time: is: {:?}", duration);
}