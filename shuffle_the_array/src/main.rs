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
