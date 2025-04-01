use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn load(input_file: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = File::open(input_file)?;
    Ok(BufReader::new(file))
}

pub fn convert<T: Read>(reader: &mut BufReader<T>) -> Result<String, Box<dyn std::error::Error>> {
    let mut code = String::new();
    for line in reader.lines() {
        code.push_str(&line?);
        code.push('\n');
    }
    Ok(code)
}