use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() 
{
    count_calories("input.txt", 3)
}

fn count_calories(file_path: &str, number_to_count: i32)
{
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut current = 0;
    let mut elves : Vec<i32> = Vec::new();

    for line in reader.lines()
    {
        let number_string = line.unwrap();
        if number_string == ""
        {
            elves.push(current);
            current = 0;
        }
        else
        {
            let number = number_string.parse::<i32>().unwrap();
            current += number;
        }
    }

    elves.sort();

    // Part 1
    print!("Part 1:\n");
    print!("Highest: {} \n", elves.last().unwrap().to_string());

    let start_id: usize = elves.len() - number_to_count as usize;

    let slice = &elves[start_id..];

    let mut sum = 0;
    for elf in slice
    {
        sum += elf;
    }

    // Part 2
    print!("Part 2:\n");
    print!("Sum of 3 Highest: {} \n", sum);
}
