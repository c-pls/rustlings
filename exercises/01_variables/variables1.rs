fn main() {
    // TODO: Add the missing keyword.
    let x: i32 = 5;

    println!("x has the value {x}");

    // shadowing
    let x = x * x;

    println!("{x}");

}
