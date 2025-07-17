use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Result};
fn main() -> Result<()> {
    //If there is a file (or line) reading error,
    //the ? at the end of the fs::File::open("output.log"?; call automatically propagates the error and terminates the program
    // key : INFO and value :3 how many info
    //INFO:3 ERROR:3
    let file = fs::File::open("output.log")?;
    let reader = BufReader::new(file); //read line by line
    let mut total_line = 0;
    let mut level_counts_map = HashMap::new();
    let log_level = vec!["ERROR", "INFO", "WARNING"];
    for line in reader.lines() {
        let line = line?;
        total_line += 1;
        for level in &log_level {
            if line.contains(level) {
                //if this entry exist then modifiy it and_modify
                //if dosent exit then pass default value 1
                //deref *value
                //`v` are`&mut i32
                //The += operator is implemented for i32, not for &mut i32
                //Rust does not enforce automatic dereferencing on arithmetic operators.
                level_counts_map
                    .entry(level)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
            }
        }
    }
    println!("Total line : {}", total_line);

    println!("\n Log level counts : ");
    for (key, value) in &level_counts_map {
        println!(" {} : {} ", key, value);
    }
    // key mean anonyomus functuion
    if let Some((key, value)) = level_counts_map.iter().max_by_key(|entry| entry.1) {
        println!("\n Most common log level :{} ({} times", key, value);
    } // retrun enrty with the max values

    Ok(())
}
