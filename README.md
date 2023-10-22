# Performance Report

## 1. Introduction
I wrote a Python program `main.py` that run 10000 times to add the sum of the first 10,000 positive integers and then subtract the first 10,000 positive integers.  The code is as follows:
```python
import time

start_time = time.time()

for j in range(10000): 
    sum = 0
    for i in range(10000):
        sum += i 
    for i in range(10000):
        sum -= i
end_time = time.time()
elapsed_time = end_time - start_time

print(f"Result: {sum}")
print(f"Python elapsed time: {elapsed_time:.2f} seconds")
```

Then I rewrote the same program `main.rs` in Rust.  The code is as follows:
```rust
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
```
## 2. Performance mesurement
I use the `time` command in the terminal to run both the Python code and the Rust executable file and measure their execution time:
```
$ time python main.py
$ time ./main
```

## 3. Result
The result is as follows:
| Language | Execution Time | CPU Usage |
| :---: | :---: | :---: |
|Python | 8.89 seconds | 99% |
| Rust | 0.65 seconds | 74% |

## 4. Conclusion
As you can see, the Rust code is significantly faster than the Python code, and it also uses less CPU. This is because Rust is a compiled language, while Python is an interpreted language.  The Rust compiler compiles the code into machine code, which is then executed by the CPU.  The Python interpreter interprets the code line by line and executes it.  Therefore, the Rust code is faster than the Python code.