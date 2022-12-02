use std::clone;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

fn main() 
{
    let mut scoresValues = vec![String::from("A"), String::from("B"), String::from("C"), String::from("X"), String::from("Y"), String::from("Z")];

    let lines = load_file("input.txt");

    let mut totalScore1 = 0;
    let mut totalScore2 = 0;
    for line in lines.into_iter() 
    {
        let (score1, score2) = parse_line_1(line.unwrap(), &scoresValues);
        totalScore1 += score1;
        totalScore2 += score2;
    }

    print!("Part 1 Total Score: {} \n", totalScore1);
    print!("Part 2 Total Score: {} \n", totalScore2);
}

fn load_file(file_path: &str) -> Lines<BufReader<File>>
{
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader.lines()
}


fn parse_line_1(line: String, scoresValues: &Vec<String>) -> (i32, i32)
{
    let side1: &String = &line[..1].to_string();
    let side2: &String = &line[2..].to_string();
    let side2Value =  scoresValues.into_iter().position(|r| r == side2).unwrap() - 2;

    //Part1
    let mut winPoints1: usize = 0;
    match side2.as_str(){
        "X" => 
        {
            match side1.as_str()
            {
                "A" => winPoints1 = 3,
                "B" => winPoints1 = 0,
                "C" => winPoints1 = 6,
                _ => winPoints1 = 0,
            }
        }
        "Y" => 
        {
            match side1.as_str()
            {
                "A" => winPoints1 = 6,
                "B" => winPoints1 = 3,
                "C" => winPoints1 = 0,
                _ => winPoints1 = 0,
            }
        }
        "Z" => 
        {
            match side1.as_str()
            {
                "A" => winPoints1 = 0,
                "B" => winPoints1 = 6,
                "C" => winPoints1 = 3,
                _ => winPoints1 = 0,
            }
        }
        _ => winPoints1 = 0,
    }

    //part2
    let mut winPoints2: usize = 0;
    match side2.as_str(){
        "X" => 
        {
            winPoints2 = 0;
            match side1.as_str()
            {
                "A" => winPoints2 += 3,
                "B" => winPoints2 += 1,
                "C" => winPoints2 += 2,
                _ => winPoints2 += 0,
            }
        }
        "Y" => 
        {
            winPoints2 = 3;
            match side1.as_str()
            {
                "A" => winPoints2 += 1,
                "B" => winPoints2 += 2,
                "C" => winPoints2 += 3,
                _ => winPoints2 += 0,
            }
        }
        "Z" => 
        {
            winPoints2 = 6;
            match side1.as_str()
            {
                "A" => winPoints2 += 2,
                "B" => winPoints2 += 3,
                "C" => winPoints2 += 1,
                _ => winPoints2 += 0,
            }
        }
        _ => winPoints2 = 0,
    }

    ((winPoints1 + side2Value) as i32, winPoints2 as i32)
}
