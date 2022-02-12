fn main() {
    println!("Hello, new_variables!");
    new_variables();
    println!("Hello, new_const!");
    new_const();
}

fn new_variables() {
    let mut x = 5;
    println!("The value x is: {}", x);
    x = 6;
    println!("The value x is: {}", x);
}

fn new_const() {
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
    println!("Const THREE_HOURS_IN_SECOND is: {}", THREE_HOURS_IN_SECOND);
}