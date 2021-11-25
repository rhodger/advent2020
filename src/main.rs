mod day1;
mod day2;
mod day3;

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    solve_3_2();
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

fn solve_2_2() {
    println!("Solving for 2-2");
    let file = File::open("Data/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut count: u32 = 0;

    for line in reader.lines() {
        let password = day2::Password::new(&line.unwrap());
        if password.is_ok() {
            if password.unwrap().test_alternate_validity() { count = count + 1 }
        }
    }

    println!("Solution: {}(?)", count);
}

fn solve_3_1() {
    let mut slope = day3::Slope::new("Data/day3.txt");
    let trees = slope.iterate_to_completion();

    println!("{}", trees);
}

fn solve_3_2() {
    let vectors: [(i32,i32); 5] = [
        (1, 1),
        (1, 3),
        (1, 5),
        (1, 7),
        (2, 1),
    ];

    let mut total_trees = 1;
    for vector in vectors {
        let mut slope = day3::Slope::new("Data/day3.txt");
        slope.set_vector(vector.0, vector.1);
        total_trees = total_trees * slope.iterate_to_completion();
    }


    println!("{}", total_trees);
}