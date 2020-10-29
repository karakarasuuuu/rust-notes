fn main() {
    let n = 10;
    println!("n = {}, fibo = {}", n, fibo(n));
}

fn fibo(n: i32) -> i32 {
    if n == 0 {0}
    else if n == 1 {1}
    else {
        fibo(n - 1) + fibo(n - 2)
    }
}