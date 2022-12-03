use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input.txt");

pub fn read() -> Input {
    /*
    need to parse this string into numbers
    */
    INPUT
        .trim()
        .split("\n\n")
        .map(parse_elf_calories)
        .collect::<Result<_, _>>()
        .expect("Could not parse input!")

    // unimplemented!()
}

fn parse_elf_calories (input:&str) -> Result<u32, std::num::ParseIntError>{
    let mut count = 0;
    for current in input.lines() {
        count += current.parse::<u32>()?;
    }
    // println!("{:}", count);
    Ok(count)
}