use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

fn main() 
{
    let lines = load_file("input.txt");

    let mut score1 = 0;
    let mut score2 = 0;

    for line in lines.into_iter() 
    {
        let line_string = line.unwrap();
    }

    print!("Part1 Score = {} \n", score1);
    print!("Part2 Score = {} \n", score2);

}


fn load_file(file_path: &str) -> Lines<BufReader<File>>
{
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader.lines()
}

fn move_box()
{
    
}