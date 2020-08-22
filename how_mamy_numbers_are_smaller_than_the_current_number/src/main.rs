// Given the array nums, for each nums[i] find out how many numbers in the array are smaller than it. That is, for each nums[i] you have to count the number of valid j's such that j != i and nums[j] < nums[i].
//
// Return the answer in an array.
//
//
//
// Example 1:
//
// Input: nums = [8,1,2,2,3]
// Output: [4,0,1,1,3]
// Explanation:
// For nums[0]=8 there exist four smaller numbers than it (1, 2, 2 and 3).
// For nums[1]=1 does not exist any smaller number than it.
// For nums[2]=2 there exist one smaller number than it (1).
// For nums[3]=2 there exist one smaller number than it (1).
// For nums[4]=3 there exist three smaller numbers than it (1, 2 and 2).
//
// Example 2:
//
// Input: nums = [6,5,4,8]
// Output: [2,1,0,3]
//
// Example 3:
//
// Input: nums = [7,7,7,7]
// Output: [0,0,0,0]
//
//
//
// Constraints:
//
// 2 <= nums.length <= 500
// 0 <= nums[i] <= 100



use std::collections::HashMap;
fn smaller_number_than_current(nums: Vec<i32>) -> Vec<i32> {
    // let mut result = Vec::new();
    // for index in 0..nums.len() {
    //     let mut count = 0;
    //     let &index_val = nums.get(index).unwrap();
    //     // lh
    //     let mut lh_index = index;
    //     loop {
    //         // println!("lh_index = {}", lh_index);
    //         if lh_index == 0 {
    //             break;
    //         } else if 0 < lh_index {
    //             let &lh_val = nums.get(lh_index-1).unwrap();
    //             println!("lh_val = {}", lh_val);
    //             if lh_val < index_val {
    //                 count += 1;
    //             }
    //             lh_index -= 1;
    //         }
    //     }
    //     //r
    //     let mut rh_index = index;
    //     loop {
    //         // println!("rh_index = {}", rh_index);
    //         if rh_index == nums.len() - 1 {
    //             break;
    //         } else if rh_index < nums.len() {
    //             let &rh_val = nums.get(rh_index + 1).unwrap();
    //             println!("rh_val = {}", rh_val);
    //             if rh_val < index_val {
    //                 count += 1;
    //             }
    //             rh_index += 1;
    //         }
    //     }
    //     result.push(count);
    // }
    // result

// 方法2

    let mut temp_nums = nums.clone();
    temp_nums.sort();
    let mut hash_map = HashMap::new();
    let mut count = 0;
    hash_map.insert(temp_nums[0], count);
    for i in 1..temp_nums.len() {
        count += 1;
        if temp_nums[i] != temp_nums[i - 1] {
            hash_map.insert(temp_nums[i], count);
        }
    }

    let mut result = vec![0; nums.len()];
    for i in 0..nums.len() {
        let &value = hash_map.get(&nums[i]).unwrap();
        result[i] = value;
    }

    result
}

fn main() {

    assert_eq!(smaller_number_than_current(vec![8, 1, 2, 2, 3]), vec![4,0,1,1,3]);
    println!("ret = {:?}", smaller_number_than_current(vec![8, 1, 2, 2, 3]));
    // assert_eq!(smaller_number_than_current(vec![6, 5, 4, 8]), vec![2, 1, 0, 3]);
    // assert_eq!(smaller_number_than_current(vec![7, 7, 7, 7]), vec![0, 0, 0, 0]);
    // println!("Hello, world!");
}
