fn subtract_product_and_sum(n: i32) -> i32 {
    let s = n.to_string()
        .chars().map(|char| char.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    // println!("s ={:?}",s);

    let  product = s.iter().fold(1, |acc, &x| acc * x );
    let sum = s.iter().fold(0, |acc, &x| acc + x);

    product - sum
}
fn main() {
    // println!("Hello, world!");
    assert_eq!(subtract_product_and_sum(234), 15);
    assert_eq!(subtract_product_and_sum(4421), 21);
}
