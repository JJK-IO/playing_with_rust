use walkdir::WalkDir;
use regex::Regex;
use csv::Writer;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }
    let project_path = &args[1];
    let output_file = "extracted_strings.csv";

    let re = Regex::new(r#""([^"\\]*(?:\\.[^"\\]*)*)""#).unwrap();
    let mut wtr = Writer::from_path(output_file)?;

    for entry in WalkDir::new(project_path) {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "swift") {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            for (line_number, line) in reader.lines().enumerate() {
                let line = line?;
                for cap in re.captures_iter(&line) {
                    // Check if the match is not escaped
                    if !line[..cap.get(0).unwrap().start()].ends_with('\\') {
                        wtr.write_record(&[
                            path.to_string_lossy().to_string(),
                            line_number.to_string(),
                            cap[1].to_string()
                        ])?;
                    }
                }
            }
        }
    }

    wtr.flush()?;
    println!("Data extracted to {}", output_file);
    Ok(())
}

