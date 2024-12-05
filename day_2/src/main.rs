use std::env;
use std::fs::File;
use std::io::BufReader;
use anyhow::Result;

mod solution;
fn main() -> Result<()> {
    let mut args = env::args().into_iter().skip(1);
    let file_name = args.next().unwrap();
    let part = args.next().unwrap().parse::<i32>()?;
    
    let file = File::open(file_name)?;
    let buf = BufReader::new(file);

    if part == 1 {
        solution::part_1(buf)?;
    }
    else if part == 2 {
        solution::part_2(buf)?;
    }


    Ok(())
}
