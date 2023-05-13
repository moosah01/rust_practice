use std::fs::OpenOptions;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::error::Error;

fn word_count(file_name: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        count += line.split_whitespace().count();
    }

    Ok(count)
}

fn append_to_file(file_name: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).append(true).open(file_name)?;
    writeln!(file, "{}", content)?;

    Ok(())
}

fn print_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "text.txt";

    append_to_file(file_name, " + New text to append")?;

    let count = word_count(file_name)?;

    println!("The file contains {} words.", count);

    print_file(file_name)?;

    Ok(())
}