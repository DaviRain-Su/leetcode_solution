// We are given a list nums of integers representing a list compressed with run-length encoding.
//
// Consider each adjacent pair of elements [freq, val] = [nums[2*i], nums[2*i+1]] (with i >= 0).  For each such pair, there are freq elements with value val concatenated in a sublist. Concatenate all the sublists from left to right to generate the decompressed list.
//
// Return the decompressed list.
//
//
//
// Example 1:
//
// Input: nums = [1,2,3,4]
// Output: [2,4,4,4]
// Explanation: The first pair [1,2] means we have freq = 1 and val = 2 so we generate the array [2].
// The second pair [3,4] means we have freq = 3 and val = 4 so we generate [4,4,4].
// At the end the concatenation [2] + [4,4,4] is [2,4,4,4].
//
// Example 2:
//
// Input: nums = [1,1,2,3]
// Output: [1,3,3]
//
//
//
// Constraints:
//
// 2 <= nums.length <= 100
// nums.length % 2 == 0
// 1 <= nums[i] <= 100



fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    //
    // // 偶数组
    // let vec_1 = nums
    //     .iter()
    //     .enumerate()
    //     .filter(|(index, _)|
    //         index % 2 != 0
    //     )
    //     .map(|(_, &value)|
    //         value as i32
    //     )
    //     .collect::<Vec<i32>>();
    //
    // //奇数组
    // let vec_2 = nums
    //     .iter()
    //     .enumerate()
    //     .filter(|(index,_)|
    //         index % 2 == 0
    //     ).map(|(_, &value)|
    //         value as i32
    //     )
    //     .collect::<Vec<i32>>();
    //
    // let temp_tuple = vec_2.iter()
    //     .zip(vec_1.iter())
    //     .collect::<Vec<(&i32, &i32)>>();
    //
    //
    // let mut result = Vec::new();
    //
    // for (&freq, &val) in temp_tuple {
    //     let temp = vec![val; freq as usize];
    //     result.push(temp);
    // }
    //
    // let ret = result
    //     .into_iter()
    //     .flatten()
    //     .collect::<Vec<i32>>();
    // // println!("ret = {:?}", ret);
    // ret


    let mut size = 0;

    for i in (0..nums.len()).step_by(2) {
        size += nums[i];
    }

    let mut result = vec![0; size as usize];
    let mut k = 0;

    for i in (1..nums.len()).step_by(2) {
        for _ in 0..nums[i-1] {
            result[k] = nums[i];
            k += 1;
        }
    }

    result
}


fn main() {

    assert_eq!( decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
    assert_eq!(decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3]);
    // println!("Hello, world!");
}
