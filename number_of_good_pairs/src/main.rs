// Given an array of integers nums.
//
// A pair (i,j) is called good if nums[i] == nums[j] and i < j.
//
// Return the number of good pairs.
//
//
//
// Example 1:
//
// Input: nums = [1,2,3,1,1,3]
// Output: 4
// Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
//
// Example 2:
//
// Input: nums = [1,1,1,1]
// Output: 6
// Explanation: Each pair in the array are good.
//
// Example 3:
//
// Input: nums = [1,2,3]
// Output: 0
//
//
//
// Constraints:
//
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100




use std::collections::HashMap;

fn num_identical_pais(nums: Vec<i32>) -> i32 {
    // one Way

    // let mut count = 0;
    // for i in 0..nums.len() {
    //     let mut j = i + 1;
    //     while j < nums.len() {
    //         let val1 = nums.get(i).unwrap();
    //         let val2 = nums.get(j).unwrap();
    //         if *val1 == *val2 {
    //             println!("i = {}, j = {}", i, j);
    //             count += 1;
    //         }
    //         j += 1;
    //     }
    // }
    //
    // count


    let mut hash_map = HashMap::<i32, Vec<i32>>::new();
    let mut count = 0;
    for (index, &item) in nums.iter().enumerate(){
        if let Some(value) = hash_map.get_mut(&item){
            value.push(index as i32);
        }else {
            hash_map.insert(item as i32, vec![index as i32]);
        }
    }

    // println!("hash_map = {:#?}", hash_map);

    let count_vec = |nums : Vec<i32>| {
        let len = nums.len();
        return if len < 2 {
            0
        } else {
            (0..len).sum()
        }
    };

    for (_ , value) in hash_map {
        count += count_vec(value);
    }

    count as i32
}

fn main() {

    // let nums = vec![1, 2, 3, 1, 1, 3];
    assert_eq!(num_identical_pais(vec![1, 2, 3, 1, 1, 3]), 4);
    // assert_eq!(num_identical_pais(vec![1, 1, 1, 1]), 6);
    // assert_eq!(num_identical_pais(vec![1, 2, 3]), 0);
    println!("Hello, world!");
}
