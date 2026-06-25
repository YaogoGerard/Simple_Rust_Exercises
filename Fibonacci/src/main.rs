/*The first and second Fibonacci numbers are both 1. For n>2,
the n’th Fibonacci number is calculated recursively as the sum of the n-1’th and n-2’th Fibonacci numbers.
Write a function fib(n) that calculates the n’th Fibonacci number. */

fn fib(n: i32) -> i32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n: i32 = 2;
    println!("fib({n})= {}", fib(n));
}
