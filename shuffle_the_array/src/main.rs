// Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
//
// Return the array in the form [x1,y1,x2,y2,...,xn,yn].
//
//
//
// Example 1:
//
// Input: nums = [2,5,1,3,4,7], n = 3
// Output: [2,3,5,4,1,7]
// Explanation: Since x1=2, x2=5, x3=1, y1=3, y2=4, y3=7 then the answer is [2,3,5,4,1,7].
//
// Example 2:
//
// Input: nums = [1,2,3,4,4,3,2,1], n = 4
// Output: [1,4,2,3,3,2,4,1]
//
// Example 3:
//
// Input: nums = [1,1,2,2], n = 2
// Output: [1,2,1,2]
//
//
//
// Constraints:
//
// 1 <= n <= 500
// nums.length == 2n
// 1 <= nums[i] <= 10^3


fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32>{
    // let lh = nums[0..n as usize].to_vec();
    // let rh = nums[n as usize..].to_vec();

    let mut result = Vec::new();
    for index in 0..n {
        let lf_item = nums.get(index as usize).unwrap();
        let rh_item = nums.get(index as usize + n as usize).unwrap();
        result.push(*lf_item);
        result.push(*rh_item);
    }

    result
}

fn main() {

    let nums = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    // let nums = vec![1,2,3,4,4,3,2,1];
    // let n = 4;
    let ret = shuffle(nums, n);
    println!("ret = {:?}", ret);
    println!("Hello, world!");
}
