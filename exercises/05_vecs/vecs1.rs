fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;

    let mut v: Vec<i32> = Vec::new();
    for i in &a {
        v.push(*i);
    }

    // let v = vec![10, 20, 30, 40];
    (a, v)
}

fn main() {
    // You can optionally experiment here.
    let mut v = vec![1, 2, 3];

    v.push(100);

    for i in &mut v {
        println!("{i}");
    }

    let a = &v[2];

    println!("{a}");

    let t = v.get(100);

    match t {
        Some(_t) => println!("Good to go, got {_t}"),
        None => println!("Wrong"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
