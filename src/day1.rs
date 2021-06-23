use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn sum_pair(nums: Vec<i32>) -> Option<i32> {
    return match find_sum_pair(nums) {
        Some(x) => Some(x.0 * x.1),
        None => None,
    }
}

pub fn sum_trio(nums: Vec<i32>) -> Option<i32> {
    return match find_sum_trio(nums) {
        Some(x) => Some(x.0 * x.1 * x.2),
        None => None,
    }
}

pub fn read_input(filepath: &str) -> Option<Vec<i32>> {
    let mut out_vec = Vec::<i32>::new();

    let file = match File::open(filepath) {
        Ok(x) => x,
        Err(_) => return None,
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        out_vec.push(line.unwrap().parse().unwrap());
    }

    return Some(out_vec);
}

fn find_sum_pair(nums: Vec<i32>) -> Option<(i32,i32)> {
    let mut x: i32 = -1;
    let mut y: i32 = -1;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            let i_val = nums[i];
            let j_val = nums[j];
            if i != j {
                if i_val + j_val == 2020 {
                    x = i_val;
                    y = j_val;
                }
            }
        }
    }
    if x >= 0 && y >= 0 {
        return Some((x,y));
    } else {
        return None;
    }
}

fn find_sum_trio(nums: Vec<i32>) -> Option<(i32,i32, i32)> {
    let mut x: i32 = -1;
    let mut y: i32 = -1;
    let mut z: i32 = -1;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            for k in 0..nums.len() {
                let i_val = nums[i];
                let j_val = nums[j];
                let k_val = nums[k];
                if i != j && i != k && j != k {
                    if i_val + j_val + k_val == 2020 {
                        x = i_val;
                        y = j_val;
                        z = k_val;
                    }
                }
            }
        }
    }
    if x >= 0 && y >= 0 && z >= 0 {
        return Some((x,y,z));
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pair() {
        let mut test_nums = Vec::<i32>::new();

        test_nums.push(1721);
        test_nums.push(979);
        test_nums.push(366);
        test_nums.push(299);
        test_nums.push(675);
        test_nums.push(1456);

        assert_eq!(find_sum_pair(test_nums), Some((299, 1721)));
    }

    #[test]
    fn test_find_trio() {
        let mut test_nums = Vec::<i32>::new();

        test_nums.push(1721);
        test_nums.push(979);
        test_nums.push(366);
        test_nums.push(299);
        test_nums.push(675);
        test_nums.push(1456);

        assert_eq!(find_sum_trio(test_nums), Some((675, 366, 979)));
    }

    #[test]
    fn test_find_sum_pair() {
        let mut test_nums = Vec::<i32>::new();

        test_nums.push(1721);
        test_nums.push(979);
        test_nums.push(366);
        test_nums.push(299);
        test_nums.push(675);
        test_nums.push(1456);

        assert_eq!(sum_pair(test_nums), Some(514579));

    }

    #[test]
    fn test_find_sum_trio() {
        let mut test_nums = Vec::<i32>::new();

        test_nums.push(1721);
        test_nums.push(979);
        test_nums.push(366);
        test_nums.push(299);
        test_nums.push(675);
        test_nums.push(1456);

        assert_eq!(sum_trio(test_nums), Some(241861950));
    }

    #[test]
    fn test_read_input() {
        let filename = String::from("Data/day1.txt");

        assert_eq!(read_input(&filename).unwrap().len(), 200);
    }
}