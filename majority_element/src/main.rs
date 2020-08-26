use std::collections::HashMap;
//所有权规则；可变借用和不可变借用不能同时存在，
//在一个不可变作用域中，不能在有可变借用了
//要想在一个不可变的作用域中，有一个可变的东西。直接将这个不可变借用变成可变借用就可以了

fn majority_element(nums: Vec<i32>) -> i32 {

    let mut hashmap = HashMap::new();
    let mut count = -1;
    let len_half = nums.len() / 2;
    for &item in nums.iter(){
        if let Some(value) = hashmap.get_mut(&item) {
            if *value + 1 > len_half as i32 {
                count = item;
                break;
            }else {
                *value += 1;
            }
        }else {
            hashmap.insert(item, 1i32);
        }
    }
    count
}
fn main() {
    assert_eq!(majority_element(vec![1,2,5,9,5,9,5,5,5]), 5);
    assert_eq!(majority_element(vec![3, 2]), -1);
    assert_eq!(majority_element(vec![2,2,1,1,1,2,2]), 2);
    // println!("Hello, world!");
}
