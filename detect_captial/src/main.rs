fn detect_capital_use(word: String) -> bool{

    let mut flag = false;
    for item in word.chars() {
        if item.is_ascii_uppercase() {
            flag = true
        } else {
            flag = false
        }
    }
    flag
}

fn main() {
    assert_eq!(detect_capital_use("USA".to_string()), true);
    assert_eq!(detect_capital_use("FlaG".to_string()), false);
    println!("Hello, world!");
}
