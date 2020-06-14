pub fn digits(n: i32) -> Vec<i32>{
    //Given a number return a list of the digits.
    let s: String = n.to_string();
    let mut vec = Vec::new();
    for c in s.chars() {
        let i = c.to_string().parse::<i32>().unwrap();
        vec.push(i)
    }

    return vec
}

fn main() {
    for x in &digits(123) {
        println!("{}", x);
    }
}
