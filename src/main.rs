mod day1;
mod day2;

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    solve_2_1();
}

fn solve_1_1() {
    println!("Solving for 1-1");
    let filename = String::from("Data/day1.txt");
    let data = day1::read_input(&filename);
    if data.is_some() {
        let nums = data.unwrap();
        let final_answer = day1::sum_pair(nums);
        if final_answer.is_some() {
            println!("Solution is {}", final_answer.unwrap());
        } else {
            println!("Couldn't solve");
        }
    } else {
        println!("Failed to load from file");
    }
    println!("Done");
}

fn solve_1_2() {
    println!("Solving for 1-2");
    let filename = String::from("Data/day1.txt");
    let data = day1::read_input(&filename);
    if data.is_some() {
        let nums = data.unwrap();
        let final_answer = day1::sum_trio(nums);
        if final_answer.is_some() {
            println!("Solution is {}", final_answer.unwrap());
        } else {
            println!("Couldn't solve");
        }
    } else {
        println!("Failed to load from file");
    }
    println!("Done");
}

fn solve_2_1() {
    println!("Solving for 2-1");
    let filename = String::from("Data/day2.txt");
    let file = File::open("Data/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut count: u32 = 0;

    for line in reader.lines() {
        let password = day2::Password::new(&line.unwrap());
        if password.is_ok() {
            if password.unwrap().test_validity() { count = count + 1 }
        }
    }

    println!("Solution: {}(?)", count);
}