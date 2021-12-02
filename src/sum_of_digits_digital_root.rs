// https://www.codewars.com/kata/541c8630095125aba6000c00/rust

// 🌟 16 ➡️  1 ➕ 6 = 7

// 🌟 942 ➡️  9 ➕ 4 ➕ 2 = 15 ➡️  1 ➕ 5 = 6

// 🌟 132189 ➡️  1 ➕ 3 ➕ 2 ➕ 1 ➕ 8 ➕ 9 = 24 ➡️  2 ➕ 4 = 6

use std::io;

fn main() {
    loop {
        println!("Enter number, else for quit.");
        let mut user_input: String = String::default();
        io::stdin()
            .read_line(&mut user_input)
            .expect("read line unsuccessful ❌");
        let user_input: i64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Buy 👋");
                break;
            }
        };
        digital_root(user_input);
    }
}

fn digital_root(n: i64) -> i64 {
    let mut result: i64 = n;
    print!("🌟 {}", result);
    loop {
        let str_num: String = result.to_string();

        if str_num.len() >= 2 {
            print!(" ➡️  ");
            result = i64::from(
                str_num
                    .chars()
                    .map(|num| num.to_digit(10).unwrap())
                    .reduce(|total, num| total + num)
                    .unwrap(),
            );
            print!(
                "{} = {}",
                str_num
                    .chars()
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" ➕ "),
                result
            )
        } else {
            println!("\n");
            break;
        }
    }
    result
}

// OR

fn _digital_root(n: i64) -> i64 {
    if n < 10 {
        n
    } else {
        _digital_root(
            n.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .sum::<u32>() as i64,
        )
    }
}

fn main() {
    tests()
}

fn tests() {
    assert_eq!(digital_root(16), 7);
    assert_eq!(digital_root(132189), 6);
}
