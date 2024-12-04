use std::{error::Error, path::Path};

use regex::Regex;

use crate::util::read_lines;

pub fn day3part1<P>(filename: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let re =
        Regex::new(r"mul\((\d+),(\d+)\)").map_err(|e| format!("Invalid regex pattern: {}", e))?;

    let mut sum: usize = 0;

    for line_result in read_lines(filename)? {
        let line = line_result?;

        for caps in re.captures_iter(&line) {
            let num1_str = &caps[1];
            let num2_str = &caps[2];

            let num1: usize = num1_str
                .parse()
                .map_err(|e| format!("Failed to parse '{}': {}", num1_str, e))?;
            let num2: usize = num2_str
                .parse()
                .map_err(|e| format!("Failed to parse '{}': {}", num2_str, e))?;

            sum += num1 * num2;
        }
    }

    Ok(sum)
}

pub fn day3part2<P>(filename: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let re = Regex::new(
        r#"(?P<do_cmd>do\(\))|(?P<dont_cmd>don't\(\))|mul\((?P<num1>\d+),(?P<num2>\d+)\)"#
    )
    .map_err(|e| format!("invalid regex pattern: {}", e))?;

    let mut sum = 0;
    let mut is_active = true;

    for line_result in read_lines(filename)? {
        let line = line_result?;

        for caps in re.captures_iter(&line) {
            if let Some(_) = caps.name("do_cmd") {
                is_active = true;
            } else if let Some(_) = caps.name("dont_cmd") {
                is_active = false;
            } else if let (Some(num1_match), Some(num2_match)) =
                (caps.name("num1"), caps.name("num2"))
            {
                if is_active {
                    let num1: usize = num1_match
                        .as_str()
                        .parse()
                        .map_err(|e| format!("Failed to parse '{}': {}", num1_match.as_str(), e))?;
                    let num2: usize = num2_match
                        .as_str()
                        .parse()
                        .map_err(|e| format!("Failed to parse '{}': {}", num2_match.as_str(), e))?;

                    sum += num1 * num2;
                }
            }
        }
    }
    Ok(sum)
}
