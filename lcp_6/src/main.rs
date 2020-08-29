fn min_count(coins: Vec<i32>) -> i32 {
    coins.iter().fold(0, |mut count, x| {
        if x % 2 == 0 {
            count += x / 2;
        } else {
            count += x / 2;
            count += (x % 2) / 1;
        }
        count
    })
}
fn main() {
    assert_eq!(min_count(vec![4, 2, 1]), 4);
    assert_eq!(min_count(vec![2, 3, 10]), 8);
}
