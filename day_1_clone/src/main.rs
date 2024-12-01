mod solution;
use std::fs::File;
use std::env;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let mut args_iter = env::args().into_iter().skip(1);
    let file_name = args_iter.next().expect("Needs File Name");

    let f = File::open(file_name)?;

    let buf = BufReader::new(f);

    let option = args_iter.next().expect("Needs option").parse::<i32>()?; 

    if option == 1 {
        solution::part_1(buf)?;
    }
    else if option == 2 {
        solution::part_2(buf)?;
    }

    Ok(())
}
