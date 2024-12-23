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
    
    let mut columns = Vec::with_capacity(1000);

    for (line_num, line) in lines.enumerate() {
        let numbers = line?
            .split_whitespace()
            .enumerate()
            .map(|(val_num, val)| {
                val.parse().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Line {}, value {}: {}", line_num + 1, val_num + 1, e)
                    )
                })
            })
            .collect::<io::Result<Vec<i32>>>()?;
            
        columns.push(numbers);
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
            let safe_count = lines.iter()
                .filter(|line| is_safe(line))
                .count();
            println!("Found {} safe lines", safe_count);
            let safe_dampened_count = lines.iter()
                .filter(|line| is_safe_dampened(line))
                .count();
            println!("Found {} safe dampened lines", safe_dampened_count);
        }
        Err(error) => {
            eprintln!("Error processing file: {}", error);
            std::process::exit(1);
        }
    }
}
