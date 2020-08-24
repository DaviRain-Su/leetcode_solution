fn day_of_year(data: String) -> i32 {
    let is_leap = |year: u32| {
        year % 100 != 0 && year % 4 == 0 || year % 400 == 0
    };

    let array = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let temp_tuple = data
        .split('-')
        .map(|value| value
            .parse::<u32>()
            .unwrap()
        )
        .collect::<Vec<u32>>();


    println!("temp_tuple = {:?}", temp_tuple);

    let mut days = 0;

    if is_leap(temp_tuple[0]) { //even
        if 2 < temp_tuple[1] {
            days += array[0..(temp_tuple[1]- 1) as usize].iter().sum::<u32>();
            days += temp_tuple[2];
            days += 1;
        }else if 1 < temp_tuple[1] {
            days += array[0..(temp_tuple[1] - 1) as usize].iter().sum::<u32>();
            days += temp_tuple[2];
        }else{
            days += temp_tuple[2];
        }
    }else { // odd
        if 1 < temp_tuple[1] {
            days += array[0..(temp_tuple[1] - 1) as usize].iter().sum::<u32>();
            days += temp_tuple[2];
        }else{
            days += temp_tuple[2];
        }
    }

    days as i32
}

fn main() {
    assert_eq!(day_of_year("2019-01-09".to_string()), 9);
    assert_eq!(day_of_year("2019-02-10".to_string()), 41);
    assert_eq!(day_of_year("2004-03-01".to_string()), 61);
    println!("Hello, world!");
    assert_eq!(day_of_year("2013-11-29".to_string()), 333);
}
