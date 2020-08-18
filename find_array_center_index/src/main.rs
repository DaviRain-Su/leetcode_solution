fn pivot_index(nums: Vec<i32>) -> i32 {
    // S 是数组的和，当索引 i 是中心索引时，位于 i 左边数组元素的和 leftsum 满足 S - nums[i] - leftsum。
    // 我们只需要判断当前索引 i 是否满足 leftsum==S-nums[i]-leftsum 并动态计算 leftsum 的值
    
    let sum = nums.iter().sum::<i32>();
    let len = nums.len();
    if len <= 2 {
        return -1;
    }

    for i in 0..len {
        let lh_sum: i32= nums[..i].iter().sum();
        println!("lh_sum = {}", lh_sum);
        if lh_sum == sum - lh_sum - nums[i] {
            println!("rh sum = {}", sum - lh_sum - nums[i]);
            return i as i32;
        }
    }
    -1 // if nums is empty return
}

fn main() {
    //test case 1
    let nums = vec![1, 7, 3, 6, 5, 6];
    // test case 2
    // let nums = vec![1, 2, 3];
    // test case 3
    // let nums  = vec![];
    let ret = pivot_index(nums);

    println!("ret = {}", ret);
}

