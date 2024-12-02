use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day1part1<P>(filename: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut first = Vec::new();
    let mut second = Vec::new();
    let mut distance = 0;

    for line in read_lines(filename)? {
        let (first_num, second_num) = clean_line(line.unwrap()).unwrap();
        first.push(first_num);
        second.push(second_num);
    }

    first.sort();
    second.sort();

    for pair in first.iter().zip(second.iter()) {
        distance += pair.0.abs_diff(*pair.1)
    }

    Ok(distance)
}

pub fn day1part2<P>(filename: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut first = Vec::new();
    let mut second = Vec::new();
    let mut similarity = 0;

    for line in read_lines(filename)? {
        let (first_num, second_num) = clean_line(line.unwrap()).unwrap();
        first.push(first_num);
        second.push(second_num);
    }

    let mut count = first
        .into_iter()
        .map(|x| (x, 0))
        .collect::<HashMap<usize, usize>>();

    for val in second {
        if count.contains_key(&val) {
            count.entry(val).and_modify(|cnt| *cnt += 1).or_default();
        }
    }

    for entry in count.iter() {
        similarity += entry.0 * entry.1;
    }

    Ok(similarity)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn clean_line(line: String) -> Option<(usize, usize)> {
    let mut iter = line.split_whitespace();

    let first = iter.next()?.parse::<usize>().ok()?;
    let second = iter.next()?.parse::<usize>().ok()?;
    Some((first, second))
}
