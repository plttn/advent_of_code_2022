use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    //pop from vector and then remove?
    let mut sorted_input = input.clone();

    sorted_input.sort();
    let length_input = sorted_input.len();
    let top_three_calories = &sorted_input[length_input-3..];

    top_three_calories.iter().sum::<u32>().into()
}
