use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod test_is_safe;
#[cfg(test)]
mod test_is_safe_dampened;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut columns: Vec<Vec<i32>> = Vec::with_capacity(1000);

    for (line_number, line) in lines.enumerate() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();
        let mut line_values: Vec<i32> = Vec::with_capacity(values.len());

        for (value_number, value) in values.iter().enumerate() {
            let int = value.parse::<i32>().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Line {}, value {}: {}",
                        line_number + 1,
                        value_number + 1,
                        e
                    ),
                )
            })?;
            line_values.push(int);
        }
        // if line_values[0] > line_values[1] && line_values[1] <= line_values[2] {
        //     println!("Line from file: {}, converted lines: {:?}", line, line_values);
        // }
        columns.push(line_values);
    }

    Ok(columns)
}

fn is_ascending(a: i32, b: i32) -> Option<bool> {
    if a < b {
        Some(true)
    } else if a > b {
        Some(false)
    } else {
        None
    }
}

fn adheres_to_conditions(a: i32, b: i32, ascending: bool) -> bool {
    if ascending {
        return a < b && b - a <= 3;
    } else {
        return a > b && a - b <= 3;
    }
}

fn is_safe_internal(numbers: &Vec<i32>) -> bool {
    if numbers.len() < 2 {
        return false; // A single number or empty vector is considered unsafe
    }

    let ascending = match is_ascending(numbers[0], numbers[1]) {
        Some(asc) => asc,
        None => return false,
    };

    for i in 1..numbers.len() {
        // println!(
        //     "i: {}, numbers[i-1]: {}, numbers[i]: {}",
        //     i,
        //     numbers[i - 1],
        //     numbers[i]
        // );
        if !adheres_to_conditions(numbers[i - 1], numbers[i], ascending) {
            return false;
        }
    }
    true
}

pub fn is_safe(numbers: &Vec<i32>) -> bool {
    is_safe_internal(numbers)
}

pub fn is_safe_dampened(numbers: &Vec<i32>) -> bool {
    if !is_safe_internal(numbers) {
        // println!("{:?} Not safe, trying to dampen", numbers);
        for i in 0..numbers.len() {
            let mut modified = numbers.clone();
            modified.remove(i);
            if is_safe_internal(&modified) {
                // println!("{:?} not safe, {:?} is safe after removing {}", numbers, modified, i);
                return true;
            }
        }
        return false;
    }
    return true;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    match read_lines(&args[1]) {
        Ok(lines) => {
            println!("found {} rows", lines.len());
            let safe_lines: Vec<bool> = lines
                .iter()
                .map(|line| is_safe(line))
                .filter(|safe| *safe)
                .collect();
            println!("Found {} safe lines", safe_lines.len());
            let safe_dampened_lines: Vec<bool> = lines
                .iter()
                .map(|line| is_safe_dampened(line))
                .filter(|safe| *safe)
                .collect();
            println!("Found {} safe dampened lines", safe_dampened_lines.len());
        }
        Err(error) => {
            eprintln!("Error processing file: {}", error);
            std::process::exit(1);
        }
    }
}
