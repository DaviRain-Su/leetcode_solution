fn reverse(x: i32) -> i32 {
    let mut flag = false;
    let mut abs_x;
    if 0 < x {
        flag = true;
        abs_x = x;
    } else {
        abs_x = x.abs();
    }
    // println!("abs_x = {}", abs_x);

    let mut result :i32 = 0;

    loop {
        if abs_x / 10 == 0 && abs_x % 10 == 0 {
            break;
        } else {
            let temp = abs_x % 10;
            // println!("temp = {}", temp);
            abs_x = abs_x / 10;
            result += temp;

            if abs_x == 0 {
                result *= 1;
            } else {
                //这里会发生溢出，如果发生溢出的话这个函数返回的是None
                // if let Some(value) = result.checked_mul(10) {
                //     result = value;
                //     // println!("result = {}", result);
                // } else { // 匹配不到值返回的是0
                //     return 0;
                // }

                let ret = result.checked_mul(10);
                match ret {
                    Some(value) => result = value,
                    None => { return  0; },
                }
            }
        }
    }

    if flag { // if flag true
        // result = result;
    } else { // if flag false
        result *= -1;
    }
    result
}

fn reverse_v2(x: i32) -> i32 {
    // println!("{}", x.signum());
    x.abs()
        .to_string()
        .chars().rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0) * x.signum()

}
fn main() {
    // let x = 1534236469;
    // let x = 123;
    let x = -321;
    // let x = 120;
    let ret = reverse_v2(x);

    println!("x = {}, ret = {}", x, ret);
    // println!("Hello, world!");
}
