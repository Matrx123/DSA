fn main() {
    println!("Factorial of number n");

    let n = 4;
    let factorial = get_factorial(n);

    println!("Factorial of no {:?} is {:?}", n, factorial);
}

fn get_factorial(num: i32) -> i32 {
    if num > 0 { num * get_factorial(num - 1) } else { 1 }
}
