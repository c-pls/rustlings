// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    String::from("blue")
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");

    let data = "Guilding star";

    let s = data.to_string();

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3 = &s1 + &s2; // note s1 has been moved here and can no longer be used
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // print!("{s1}");
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s} {s1}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{s}")
}
