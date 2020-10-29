fn main() {
    let degree = 35;
    println!("C: {}, F: {}", degree, c_to_f(degree));
}

fn c_to_f(c: i32) -> i32 {
    9 * c / 5 + 32
}