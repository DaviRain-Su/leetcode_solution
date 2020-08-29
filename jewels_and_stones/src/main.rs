fn num_jewels_in_stones(j: String, s: String) -> i32 {
    s.chars().fold(0, |mut acc, value| {
        if j.contains(value) {
            acc += 1;
        }
        acc
    })
}

fn main() {
    assert_eq!(
        num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
    assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}
