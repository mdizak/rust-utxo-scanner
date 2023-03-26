
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::io;
use std::io::Write;

pub fn load(csv_file: &str) -> BTreeMap<String, Vec<u64>> {

    // Open the file
    let file = File::open(&csv_file).unwrap_or_else(|e| {
        panic!("Unable to open file due to error: {}", e);
    });

    // Initialize
    let reader = BufReader::new(file);
    let mut map: BTreeMap<String, Vec<u64>> = BTreeMap::new();

     // Iterate over the lines
    let mut line_num = 0;
    for line in reader.lines() {

        if line_num == 0 {
            line_num += 1;
            continue;
        }

        let line = line.unwrap();
        let parts: Vec<_> = line.split(",").collect();

        // Add to btree
        let address = format!("{}", parts[3]);
        map.entry(address).or_insert(Vec::new()).push(line_num as u64);
        line_num += 1;

        if line_num % 10000 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }
    }

    map
}


