fn number_of_steps (num: i32) -> i32 {
    let mut temp_num = num;
    let mut count = 0;
    loop {
        if temp_num == 0 {
            break;
        }

        if temp_num % 2 == 0 {
            temp_num /= 2;
        }else {
            temp_num -= 1;
        }
        count += 1;
    }
    count
}

fn main() {
    assert_eq!(number_of_steps(14), 6);
    assert_eq!(number_of_steps(8), 4);
    assert_eq!(number_of_steps(123), 12);
    // println!("Hello, world!");
}
