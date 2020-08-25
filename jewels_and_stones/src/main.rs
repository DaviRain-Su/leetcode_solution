
fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let mut count = 0;
    for char_i  in s.chars() {
        if j.contains(char_i) {
            count += 1;
        }
    }
    count
}

fn main() {
//    println!("Hello, world!");
    assert_eq!(num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()), 3);
    assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}
