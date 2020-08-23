fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    let temp = start_time.iter().
        zip(end_time.iter())
        .map(|value|
            (*value.0, *value.1)
        )
        .collect::<Vec<(i32, i32)>>()
        .iter()
        .filter(|&&value| {
            value.0 <= query_time && query_time <= value.1
        }).collect::<Vec<&(i32, i32)>>().len();

    temp as i32
}
fn main() {
    assert_eq!(busy_student(vec![1,2,3], vec![3, 2, 7], 4), 1);
    assert_eq!(busy_student(vec![4], vec![4], 4), 1);
    assert_eq!(busy_student(vec![4], vec![4], 5), 0);
    assert_eq!(busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7), 0);
    assert_eq!(busy_student(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], vec![10,10,10,10,10,10,10,10,10], 5), 5);

    println!("Hello, world!");
}
