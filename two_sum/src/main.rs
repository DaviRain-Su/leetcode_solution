//
// Given an array of integers, return indices of the two numbers such that they add up to a specific target.
//
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
//
// Example:
//
// Given nums = [2, 7, 11, 15], target = 9,
//
// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].



use std::collections::HashMap;

fn two_sum_v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap = HashMap::new();
    //
    // 将输入的数组，按照key为数组中的索引位置的值为key，
    // hashmap的value， 是数组中每个位置的索引作为value
    // 构造hashmap, 当数组中出现重复的元素是，后续的元素会将之前插入的
    // 相同元素的值进行覆盖，因此，重复元素的key对应的value也将更新
    // 所以这里在后续遍历数组的时候，需要遍历出索引的位置信息
    println!("Hashmap  = {:?}", hashmap);
    for (index, item ) in nums.iter().enumerate() {
        hashmap.insert(*item, index);
    }
    println!("Hashmap  = {:?}", hashmap);

    let mut vec : Vec<i32> = Vec::new();
    //这里需要遍历出的索引的位置信息，因为当数组中出现重复的元素时，map中存放的就会
    //没有之前的重复元素的信息，此时存放在map中的是最新的重复元素的信息
    for (value1 , item) in nums.iter().enumerate() {
        let temp = target - *item;
        // 目标和减去从头遍历的每一个数组元素，
        //将得到的这个元素在map中查找是否存在这个值
        //存在将会进入if语句
        if let Some(&value2) = hashmap.get(&temp) {
            if value1 != value2 {
                //这里能够保证的不会插入同一个元素到数组中去，通过value1 != value2保证
                vec.push(value1 as i32);
                vec.push(value2 as i32);
                break;
            }
        }
    }

    vec
}

fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap = HashMap::new();
    let mut vec: Vec<i32> = Vec::new();
    for (value1, &item ) in nums.iter().enumerate() {
        let temp = target - item;
        if let Some(&value2) = hashmap.get(&temp) {
            vec.push(value2 as i32);
            vec.push(value1 as i32);
        }
        hashmap.insert(item, value1);
    }

    vec
}


fn main() {
    let num = vec![2, 7, 11, 15];
    let target = 9;
    let ret = two_sum_v2(num, target);
    println!("ret = {:?}", ret);
}
