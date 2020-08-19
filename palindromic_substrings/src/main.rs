fn count_substrings(s: String) -> i32 {
    let len = s.len();
    let temp_str = &s as &str;
    let mut count = 0;
    for gap in 1..=len {
        for index in 0..len {
            if len < index + gap { continue; }
            let inner_str = &temp_str[index..index+gap];
            if is_palindromic(inner_str) {
                count += 1;
            }
        }
    }
    count
}


fn is_palindromic(substring: &str) -> bool {
    let len = substring.len();
    return if len == 0 {
        true
    } else {
        for i in 0..(len / 2) {
            let lhs = &substring[i..i + 1];
            let rhs = &substring[len - i - 1..len - i];
            if lhs.eq(rhs) {
                continue;
            } else {
                return false;
            }
        }
        //遍历完所有正确返回true
        true
    }
}

fn main() {
    // let ret = is_palindromic("ab");
    // aaaaa == 15
    // aaa == 6
    // abc == 3
    let s = String::from("aaaaa"); // 15
    let ret = count_substrings(s);
    println!("ret = {}", ret);
}
