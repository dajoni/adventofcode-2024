use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Represents columnar data read from a file
/// where each line contains values that are split into columns
struct ColumnData {
    /// Vector of columns, where each column is a vector of integers
    columns: Vec<Vec<i32>>,
    /// Number of columns in the data
    num_columns: usize,
    /// Number of items (rows) in each column
    num_items: usize,
}

fn read_columns<P: AsRef<Path>>(filename: P) -> io::Result<ColumnData> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    
    let mut columns: Vec<Vec<i32>> = vec![Vec::new(), Vec::new()];
    let mut total_items = 0;
    let expected_columns = 2;
    
    for (line_number, line) in lines.enumerate() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();
        
        if values.len() != expected_columns {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                format!("Line {} contains {} values, expected {}", 
                    line_number + 1, 
                    values.len(), 
                    expected_columns
                )
            ));
        }
        
        let first = values[0].parse::<i32>()
            .map_err(|e| io::Error::new(
                io::ErrorKind::InvalidData, 
                format!("Line {}: {}", line_number + 1, e)
            ))?;
        let second = values[1].parse::<i32>()
            .map_err(|e| io::Error::new(
                io::ErrorKind::InvalidData, 
                format!("Line {}: {}", line_number + 1, e)
            ))?;
        
        columns[0].push(first);
        columns[1].push(second);
        total_items += 1;
    }

    Ok(ColumnData {
        num_columns: expected_columns,
        num_items: total_items,
        columns,
    })
}

fn calculate_similarities(column_data: &ColumnData) -> Vec<i32> {
    let mut similarities = Vec::with_capacity(column_data.num_items);
    
    for i in 0..column_data.num_items {
        let value = column_data.columns[0][i];
        
        // Find the first occurrence of value using binary search
        let first = match column_data.columns[1].binary_search(&value) {
            Ok(pos) => {
                // Found a match, now find bounds of all equal elements
                let start = column_data.columns[1][..=pos].partition_point(|&x| x < value);
                let end = column_data.columns[1][pos..].partition_point(|&x| x == value) + pos;
                end - start
            }
            Err(_) => 0, // Value not found
        };
        
        similarities.push(value * first as i32);
    }
    similarities
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    match read_columns(&args[1]) {
        Ok(mut column_data) => {
            println!("found {} columns with {} items each", 
                column_data.num_columns, 
                column_data.num_items
            );
            for col in column_data.columns.iter_mut() {
                col.sort_unstable();
            }
            let similarities = calculate_similarities(&column_data);
            let sum: i32 = similarities.iter().sum();
            println!("Sum of similarity: {}", sum);
        }
        Err(error) => {
            eprintln!("Error processing file: {}", error);
            std::process::exit(1);
        }
    }
}
