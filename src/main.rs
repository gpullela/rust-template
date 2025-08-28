fn main() {
    println!("Hello, world!");
    let a = 5;
    let b = 10;
    add_and_print(a, b);
}

//Write another function that adds two integers together and prints the resulting sum
fn add_and_print(a: i32, b: i32) {
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);
}

