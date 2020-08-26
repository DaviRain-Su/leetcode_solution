pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut chars_temp: Vec<char> = vec!['0';s.len()];
    let mut chars_s: Vec<char> = s.chars().collect();

    for index in 0..s.len(){
        chars_temp[indices[index] as usize] = chars_s[index];
    }

    let s : String = chars_temp.into_iter().collect();
    s
}

fn main() {
    assert_eq!(restore_string("codeleet".to_string(), vec![4,5,6,7,0,2,1,3]), "leetcode".to_string());
    assert_eq!(restore_string("abc".to_string(), vec![0, 1, 2]), "abc".to_string());
    assert_eq!(restore_string("aiohn".to_string(),vec![3,1,4,2,0]), "nihao".to_string());
    assert_eq!(restore_string("aaiougrt".to_string(), vec![4,0,2,6,7,3,1,5]), "arigatou".to_string());
    assert_eq!(restore_string("art".to_string(), vec![1,0,2]), "rat".to_string());
    // println!("Hello, world!");
}
