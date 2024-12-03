use std::fs::File;
use std::io::{self, BufReader, BufRead};


fn read_lines(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let sorted1 = sort_numbers(parts[0]);
            let mut chars: Vec<char> = sorted1.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            chars.reverse();
            let sorted1: String = chars.into_iter().collect();
            decode_difference(&sorted1);
            
            let sorted2 = sort_numbers(parts[1]);
            let mut chars: Vec<char> = sorted2.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            chars.reverse();
            let sorted2: String = chars.into_iter().collect();
            decode_difference(&sorted2);
            // TODO: add sorted1[n] sorted2[n] 
            //      where n is an iterator of the length of each set.
            println!("{} {}", sorted1, sorted2);
        }
    }
    Ok(())
}


// take a reference to a string of numbers example "312142" and return a single byte.
fn decode_difference(sorted_string: &str) -> char {
    let sorted_string = sorted_string;
    let bytes = sorted_string.as_bytes();
    let first_byte: u32 = bytes[0].into();
    let first_byte_decodes = char::from_u32(first_byte).unwrap();
    println!("Byte 1: {}", first_byte_decodes);
    return first_byte_decodes;
            
}


fn sort_numbers(numbers: &str) -> String {
    let mut nums: Vec<u32> = numbers.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    nums.sort_unstable();
    nums.iter().map(|&n| n.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() -> io::Result<()> {
    read_lines("list.txt").expect("ERROR");

    Ok(())
}

