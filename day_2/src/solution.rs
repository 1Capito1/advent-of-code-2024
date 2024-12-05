use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(PartialEq)]
enum Direction {
    StrictlyIncreasing,
    StrictlyDecreasing,
    None,
}

pub fn part_1(buf: BufReader<File>) -> Result<()> { 
    let mut safe_reports = 0;
    for line in buf.lines() {
        let line = line?;
        let report: Vec<i32> = line.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut first_direction: Direction = Direction::None;
        let mut is_safe = true;
        let mut last = report[0];
        for nums in &report[1..] {
            let nums = *nums;
            let diff = nums.abs_diff(last);

            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
            if first_direction == Direction::None {
                if nums > last {
                    first_direction = Direction::StrictlyIncreasing;
                }
                else if nums < last {
                    first_direction = Direction::StrictlyDecreasing;
                }   
                else if nums == last {
                    is_safe = false;
                    break;
                }
                last = nums;
                continue;
            }
            else if nums > last {
                if first_direction == Direction::StrictlyIncreasing {
                    last = nums;
                    continue;
                }
                else {
                    is_safe = false;
                    break;
                }
            }
            else if nums < last {
                if first_direction == Direction::StrictlyDecreasing {
                    last = nums;
                    continue;
                }
                else {
                    is_safe = false;
                    break;
                }
            }
            else {
                is_safe = false;
                break
            };
        }
        if is_safe {
            safe_reports += 1;
        }
    }
    println!("{safe_reports}");
    Ok(())
}

pub fn part_2(buf: BufReader<File>) -> Result<()> {
    let mut safe_reports = 0;
    for (i, line) in buf.lines().enumerate() {
        let line = line?;
        let report: Vec<i32> = line.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut first_direction: Direction = Direction::None;
        let mut is_safe = true;
        let mut last = report[0];
        let mut ignore_index = -1;
        for nums in &report[1..] {
            let nums = *nums;
            let diff = nums.abs_diff(last);

            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
            if first_direction == Direction::None {
                if nums > last {
                    first_direction = Direction::StrictlyIncreasing;
                }
                else if nums < last {
                    first_direction = Direction::StrictlyDecreasing;
                }   
                else if nums == last {
                    if ignore_index == -1 {
                        ignore_index = i as i32;
                        continue;
                    }
                    is_safe = false;
                    break;
                }
                last = nums;
                continue;
            }
            else if nums > last {
                if first_direction == Direction::StrictlyIncreasing {
                    last = nums;
                    continue;
                }
                else {
                    is_safe = false;
                    break;
                }
            }
            else if nums < last {
                if first_direction == Direction::StrictlyDecreasing {
                    last = nums;
                    continue;
                }
                else {
                    is_safe = false;
                    break;
                }
            }
            else {
                is_safe = false;
                break
            };
        }
        if is_safe {
            safe_reports += 1;
        }
    }
    println!("{safe_reports}");

    Ok(())
}
