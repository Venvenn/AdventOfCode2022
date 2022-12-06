use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

const LOWER_OFFSET: u32 = 96;
const UPPER_OFFSET: u32 = 38;

fn main() 
{
    let lines = load_file("input.txt");

    let mut score1 = 0;
    let mut score2 = 0;
    let mut line_count = 0;
    let mut line_arary: [String; 3] = Default::default();

    for line in lines.into_iter() 
    {
        let line_string = line.unwrap();
        line_arary[line_count] = line_string.clone();
        score1 += parse_line(line_string);


        line_count+=1;

        if(line_count >= 3)
        {
            line_count = 0;
            score2 += get_score_of_unique(&line_arary);
        }
    }

    print!("Part1 Score = {}", score1);
    print!("Part2 Score = {}", score2);
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
    let mut char_map = HashMap::new();

    for c in string1.chars()
    {
        char_map.insert(c, false);
    }

    for string in strings[..].iter() 
    {
        for character in string.chars().into_iter()
        {
            let mapCharacter = &character.clone();
            if char_map.contains_key(mapCharacter)
            {
                char_map.get_mut(&character) = true;
            }
        }

        for mapChar in char_map
        {
            if mapChar.1
            {
                char_map.remove(&mapChar.0);
            }
        }
    }

    let mut score = 0;

    for char in char_map.keys()
    {
        let offset: u32 = match char.is_uppercase() 
        {
            true => UPPER_OFFSET,
            false => LOWER_OFFSET
        };

        score += (*char as u32) - offset;
    }
   
    score
}

