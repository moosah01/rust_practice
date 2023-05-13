use csv::Reader;
use std::error::Error;
use std::path::Path;
use csv::Writer;
use std::fs::OpenOptions;
use std::collections::HashMap;

use serde::Deserialize; // <- Make sure you have this line

// [dependencies]
// csv = "1.1.6"
// serde = { version = "1.0", features = ["derive"] }

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    grade: f32,
}

fn grade_to_letter(grade: f32) -> char {
    match grade as u8 {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

fn append_to_csv(name: &str, grade: f32) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().write(true).append(true).open("grades.csv")?;
    let mut writer = Writer::from_writer(file);

    writer.write_record(&[name, &grade.to_string()])?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut student_grades: HashMap<String, (f32, u32)> = HashMap::new();
    let path = Path::new("grades.csv");
    let mut reader = Reader::from_path(path)?;

    for result in reader.deserialize() {
        let record: Record = result?;
        let entry = student_grades.entry(record.name).or_insert((0.0, 0));
        entry.0 += record.grade;
        entry.1 += 1;
    }

    let mut averages: Vec<(String, f32)> = student_grades
        .into_iter()
        .map(|(name, (total, count))| (name, total / count as f32))
        .collect();

    averages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (name, avg) in averages {
        let letter = grade_to_letter(avg);
        println!("{}: {} ({})", name, avg, letter);
    }

    append_to_csv("NewStudent", 85.0)?;


    Ok(())
}