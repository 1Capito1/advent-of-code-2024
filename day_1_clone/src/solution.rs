#![allow(dead_code)]
#![allow(unused_variables)]
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};



pub fn part_1(buf: BufReader<File>) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();
    for line in buf.lines() {
        let line = line?;
        let mut line_nums = line.split_whitespace();

        list_1.push(line_nums.next().unwrap().parse::<i32>()?);
        list_2.push(line_nums.next().unwrap().parse::<i32>()?);
    }

    list_1.sort();
    list_2.sort();

    let mut distance: Vec<i32> = Vec::new();

    for (i, x) in list_1.iter().enumerate() {
        distance.push((x - list_2[i]).abs());
    }

    println!("{}", distance.iter().sum::<i32>());

    Ok(())
}

pub fn part_2(buf: BufReader<File>) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut list : Vec<i32> = Vec::new();

    for line in buf.lines() {
        let line = line?;

        let mut line_nums = line.split_whitespace();

        let x = line_nums.next().unwrap().parse::<i32>()?;

        if !map.contains_key(&x) {
            map.insert(x, 0);
        }
        let y = line_nums.next().unwrap().parse::<i32>()?;
        list.push(y);
    }

    for key in map.clone().keys() {
        let count = list.iter().filter(|x| *x == key).count() as i32;

        *map.get_mut(&key).unwrap() = key * count;
    }

    println!("{}", map.values().sum::<i32>());
    Ok(())
}
