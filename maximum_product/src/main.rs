fn maximum_product(nums: Vec<i32>) -> i32 {
    // let mut temp = nums;
    // temp.sort();
    // let len = temp.len();
    // let mut sum = 1;
    // for item in len-3..len{
    //    sum *= temp[item]
    // }
    // sum
    // let mut temp = nums;

    let flag = nums.iter().all(|&x| x >= 0);
    println!("flag = {}", flag);
    let mut temp = nums;
    temp.sort();
    let mut product_1 = 1;
    let mut product_2 = 1;
    let len = temp.len();

    for item in len-3..len{
        product_1 *= temp[item];
    }

    product_2 = temp[0] * temp[1];
    product_2 *= temp[len-1];

    if product_1 > product_2 {
        product_1
    }else {
        product_2
    }
}
fn main() {
    // println!("Hello, world!");
    assert_eq!(maximum_product(vec![1, 2, 3]), 6);
    assert_eq!(maximum_product(vec![1, 2, 3, 4]), 24);
    assert_eq!(maximum_product(vec![-4,-3,-2,-1,60]), 720);
    assert_eq!(maximum_product(vec![7,3,1,0,0,6]), 126);
    assert_eq!(maximum_product(vec![-710,-107,-851,657,-14,-859,278,-182,
                                    -749,718,-640,127,-930,-462,694,969,143,309,
                                    904,-651,160,451,-159,-316,844,-60,611,-169,
                                    -73,721,-902,338,-20,-890,-819,-644,107,404,
                                    150,-219,459,-324,-385,-118,-307,993,202,-147,
                                    62,-94,-976,-329,689,870,532,-686,371,-850,-186,
                                    87,878,989,-822,-350,-948,-412,161,-88,-509,836,
                                    -207,-60,771,516,-287,-366,-512,509,904,-459,683,
                                    -563,-766,-837,-333,93,893,303,908,532,-206,990,
                                    280,826,-13,115,-732,525,-939,-787]
    ), 972256230
    );
}
