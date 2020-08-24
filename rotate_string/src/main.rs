fn rotate_string(a: String, b: String) -> bool {
    if a.is_empty() && b.is_empty() {
        return true;
    }

    let len = a.len();

    for i in 1..len + 1 {
        let lh_temp = &a[..i];
        let rh_temp = &a[i..];
        let temp = format!("{}{}", rh_temp, lh_temp);
        if temp == b {
            return true;
        }
    }
    false
}
fn main() {
    assert_eq!(rotate_string("abcde".to_string(), "cdeab".to_string()), true);
    assert_eq!(rotate_string("abcde".to_string(), "abced".to_string()), false);
    println!("Hello, world!");
}
