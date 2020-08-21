extern crate valid_parentheses;

use std::collections::HashMap;
use valid_parentheses::stack::Stack;

fn is_valid(s: String) -> bool{

    let mut hashmap = HashMap::new();
    hashmap.insert('(', ')');
    hashmap.insert('{', '}');
    hashmap.insert('[', ']');
    hashmap.insert('#', '#');
    hashmap.insert(')', '#');
    hashmap.insert('}', '#');
    hashmap.insert(']', '#');

    let temp_vec  = s.chars().collect::<Vec<char>>();
    let mut temp_stack = Stack::<char>::new();

    for item in temp_vec {
        if item == '('  || item == '{' || item == '[' {
            temp_stack.push(item);

        } else if item == ')' || item == '}' || item == ']' {
            let temp = temp_stack.peek().unwrap_or(&'#');
            let rhs_temp = hashmap.get(temp);
            if *rhs_temp.unwrap() == item {
                temp_stack.pop();
            } else {
                temp_stack.push(item);
            }
        }
    }

    temp_stack.is_empty()
}


fn main() {

    // let s = String::from("]");
    let s = "([)]".to_owned();
    let ret = is_valid(s);
    println!("ret = {}", ret);
}
