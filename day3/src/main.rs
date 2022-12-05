use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

const LOWER_OFFSET: u32 = 96;
const UPPER_OFFSET: u32 = 38;

fn main() 
{
    let lines = load_file("input.txt");

    let mut score = 0;
    let mut line_count = 0;
    let mut line_arary = &[String; 3];
    for line in lines.into_iter() 
    {
        score += parse_line(line.unwrap());
        line_arary[line_count] = &line;

        line_count+=1;
    }

    print!("Part1 Score = {}", score)
}

fn load_file(file_path: &str) -> Lines<BufReader<File>>
{
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader.lines()
}

fn parse_line(line: String) -> u32
{
    let half_line_length = line.len() / 2;
    let (compartment1, compartment2) = line.split_at(half_line_length);
    let part1_array = [compartment1.to_string(), compartment2.to_string()];
    let part1_score = get_score_of_unique(&part1_array);

    part1_score
}

fn get_score_of_unique(strings: &[String]) -> u32
{
    let string1 = &strings[0];
    let mut charVec: Vec<char> = string1.chars().collect();

    for string in strings[1..].iter() 
    {
        let mut i = charVec.len(); 
        while i > 0 
        {
            i -= 1;
            let character = charVec.get(i).unwrap();

            if(!string.contains(*character))
            {
                charVec.remove(i);     
            }
        }
    }

    let mut score = 0;

    for char in charVec
    {
        let offset: u32 = match char.is_uppercase() 
        {
            true => UPPER_OFFSET,
            false => LOWER_OFFSET
        };

        score += (char as u32) - offset;
    }
   
    score
}

