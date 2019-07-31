pub fn factorial(e: i32){
    let mut result = 1;
    for i in 1..e {
        result *= i;
    }

    println!("{}", result);
}

pub fn sum(mut e: i32){
    let mut result = 0;

    while e != 0{
        result += e % 10;
        e = e / 10;
    }

    println!("{}", result);
}

pub fn fizz_buzz(e: i32){
    for i in 1..e{
        match i {
            i if i % 15 == 0 => print!("FizzBuzz, "),
            i if i % 5 == 0 => print!("Buzz, "),
            i if i % 3 == 0 => print!("Fizz, "),
            _ => print!("{}, ", i),
        }
    }
}

use std::collections::HashMap;

pub fn word_puzzle(code: &str){
    let map: HashMap<String, String> = [
        (String::from("A"), String::from("4")), 
        (String::from("E"), String::from("3")),
        (String::from("Q"), String::from("9")),
        ].iter().cloned().collect();

    let mut result = String::new();

    for c in code.chars() {
        let big: String = c.to_uppercase().collect::<String>();
        
        let option = match map.contains_key(&big){
            true => map.get(&big),
            _ => Some(&big),
        };
        
        if let Some(x) = option {
            result.push_str(x);
        }
    }

    println!("{}", result);
}