fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let len = arr.len();

    let mut count = 0;
    for i in 0..len {
        for j in i+1..len {
            for k in j+1..len {
                 // temp_vec.push((arr[i], arr[j], arr[k]));
                count += if
                        (arr[i] - arr[j]).abs() <= a &&
                        (arr[j] - arr[k]).abs() <= b &&
                        (arr[i] - arr[k]).abs() <= c
                    {1} else {0};
            }
        }
    }
    count
}

fn main() {
    assert_eq!(count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
    assert_eq!(count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0,1), 0);
    // println!("Hello, world!");
}
