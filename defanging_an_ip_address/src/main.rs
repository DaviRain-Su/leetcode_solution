fn defang_i_paddr(address: String) -> String {
    let mut result = String::new();
    for char_i in address.chars() {
        if char_i == '.' {
            result.push_str("[.]");
        }else {
            result.push(char_i);
        }
    }
    result
}

fn main() {
    assert_eq!(defang_i_paddr("1.1.1.1".to_string()), "1[.]1[.]1[.]1".to_string());
    assert_eq!(defang_i_paddr("255.100.50.0".to_string()), "255[.]100[.]50[.]0".to_string());
}
