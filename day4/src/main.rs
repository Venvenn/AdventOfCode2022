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
        let (result1, result2) = contains_other_range(line_string);
        score1 += result1;
        score2 += result2;
    }

    print!("Part1 Score = {} \n", score1);
    print!("Part2 Score = {} \n", score2);
}

fn contains_other_range(line: String) -> (i32, i32)
{
    let elfs:Vec<&str> = line.split(",").collect();

    print!("Elfs = {} \n", line);

    let elf1_zones:Vec<&str> = elfs[0].split("-").collect();
    let elf2_zones:Vec<&str> = elfs[1].split("-").collect();

    let elf1 = Elf
    {
        min_range: elf1_zones[0].parse().unwrap(),
        max_range: elf1_zones[1].parse().unwrap()
    }; 
    let elf2 = Elf
    {
        min_range: elf2_zones[0].parse().unwrap(),
        max_range: elf2_zones[1].parse().unwrap()
    }; 

    let mut elf_score1 = 0;
    let mut elf_score2 = 0;

    if check_elf_range(&elf1, &elf2, false) || check_elf_range(&elf2, &elf1, false)
    {
        elf_score1 = 1;
    }

    if check_elf_range(&elf1, &elf2, true) || check_elf_range(&elf2, &elf1, true) 
    {
        elf_score2 = 1;
    }

    (elf_score1,elf_score2)
}

fn check_elf_range(elf1: &Elf, elf2: &Elf, overlap: bool) -> bool
{
    if overlap
    {
        (elf1.min_range <= elf2.max_range && elf1.min_range > elf2.min_range) || {elf1.min_range >= elf2.min_range && elf1.max_range <= elf2.max_range}
    }
    else
    {
        elf1.min_range >= elf2.min_range && elf1.max_range <= elf2.max_range
    }
}

fn load_file(file_path: &str) -> Lines<BufReader<File>>
{
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader.lines()
}

struct Elf
{
    min_range: i32,
    max_range: i32
}