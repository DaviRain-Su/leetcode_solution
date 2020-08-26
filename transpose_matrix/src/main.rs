fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //仅仅是由原来的行遍历改成列遍历就OK🙆‍了
    let row = a.len();
    let col = a[0].len();

    let mut result:Vec<Vec<i32>> = Vec::new();
    let mut temp : Vec<i32> = Vec::new();
    for c in 0..col {
        // let mut temp : Vec<i32> = Vec::new();
        for r in 0..row {
            temp.push(a[r][c]);
        }
        result.push(temp);
        temp = vec![];
    }

    // vec![vec![]]
    result
}

fn main() {
    assert_eq!(transpose(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]),vec![vec![1,4,7],vec![2,5,8],vec![3,6,9]]);
    assert_eq!(transpose(vec![vec![1,2,3],vec![4,5,6]]), vec![vec![1,4],vec![2,5],vec![3,6]]);
    // println!("Hello, world!");
}
