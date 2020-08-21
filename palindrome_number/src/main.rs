fn is_palindrome(x: i32) -> bool {
    let temp = x
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>().unwrap_or(0);

    temp == x
}

fn main() {
    let x = 10;
    let ret = is_palindrome(x);
    println!("ret = {}", ret);
}
