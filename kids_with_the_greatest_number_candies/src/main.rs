fn kid_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool>
{
    let mut result = Vec::new();
    let &max_num = candies.iter().max().unwrap();
    for &item in candies.iter() {
        if max_num - item <= extra_candies {
            result.push(true);
        } else {
            result.push(false);
        }
    }

    result
}

fn main() {
    let candies = vec![2, 3];
    let extra_candies = 3;
    let ret = kid_with_candies(candies, extra_candies);
    println!("ret = {:?}", ret);
    // println!("Hello, world!");
}
