use std::{fs::{File, OpenOptions}, io::{BufReader, BufWriter, Write, BufRead}, path::Path};
use chrono::{NaiveDate, Utc, Duration};

const FILENAME: &str = "history.txt";
const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn append_to_history(data: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILENAME)?;

    writeln!(file, "\n{} {}", Utc::now().format(DATE_FORMAT), data)?;

    // Remove lines with dates older than one month
    let cutoff_date = (Utc::now().naive_utc() - Duration::days(30)).date();
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(|line| line.ok());

    // Create a new file writer
    let temp_file = Path::new(FILENAME).with_extension("temp");
    let mut writer = BufWriter::new(File::create(&temp_file)?);

    // Write all lines that have a date newer than the cutoff date
    while let Some(line) = lines.next() {
        if line.starts_with("\n") { continue; }
        if line.is_empty() { continue }
        // println!("Parsing {}", line);
        let date_str = line.splitn(2, ' ').next().unwrap_or("");
        let date = NaiveDate::parse_from_str(date_str, DATE_FORMAT).expect("No date able to be parsed");
        if date >= cutoff_date {
            writeln!(writer, "{}", line)?;
        }
    }

    // Replace the old file with the new file
    std::fs::rename(temp_file, FILENAME)?;

    Ok(())
}