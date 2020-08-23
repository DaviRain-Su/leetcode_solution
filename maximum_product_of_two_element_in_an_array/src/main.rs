fn max_product(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let size = nums.len();
    (nums[size - 2] - 1) * (nums[size - 1] - 1)
}


fn main() {
    assert_eq!(max_product(vec![3, 4, 5]), 12);
    assert_eq!(max_product(vec![1, 5, 4, 5]), 16);
    assert_eq!(max_product(vec![3,7]), 12);
    println!("Hello, world!");
}
