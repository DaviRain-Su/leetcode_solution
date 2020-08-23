// On a plane there are n points with integer coordinates points[i] = [xi, yi]. Your task is to find the minimum time in seconds to visit all points.
//
// You can move according to the next rules:
//
// In one second always you can either move vertically, horizontally by one unit or diagonally (it means to move one unit vertically and one unit horizontally in one second).
// You have to visit the points in the same order as they appear in the array.
//
//
// Example 1:
//
//
// Input: points = [[1,1],[3,4],[-1,0]]
// Output: 7
// Explanation: One optimal path is [1,1] -> [2,2] -> [3,3] -> [3,4] -> [2,3] -> [1,2] -> [0,1] -> [-1,0]
// Time from [1,1] to [3,4] = 3 seconds
// Time from [3,4] to [-1,0] = 4 seconds
// Total time = 7 seconds
// Example 2:
//
// Input: points = [[3,2],[-2,2]]
// Output: 5
//
//
// Constraints:
//
// points.length == n
// 1 <= n <= 100
// points[i].length == 2
// -1000 <= points[i][0], points[i][1] <= 1000

use std::cmp::max;

fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    // let mut count = 0;
    // let temp = points.iter().map(|value|{
    //     (value[0], value[1])
    // }).collect::<Vec<(i32, i32)>>();
    //
    // // println!("temp = {:?}", temp);
    //
    // for i in 1..temp.len() {
    //     let l = (temp[i].0 - temp[i - 1].0).abs();
    //     let r = (temp[i].1 - temp[i - 1].1).abs();
    //     let ret = max(l,r);
    //     count += ret;
    // }
    //
    // count
    let mut last = &points[0];
    let mut t = 0;

    for p in points.iter() {
        let (dx, dy) = ((p[0]-last[0]).abs(), (p[1]-last[1]).abs());
        t += dx.max(dy);
        last = p;
    }

    t
}

fn main() {
    assert_eq!(min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]), 7);
    assert_eq!(min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]), 5);
    // println!("Hello, world!");
}
