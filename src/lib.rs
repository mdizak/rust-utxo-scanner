//
// Usage:
//
// ! ```rust,ignore
//
// use utxo_scanner
//
// //Scan for all UTXOs
// let stats = utxo_scanner::scan("/path/to/.bitcoin", true, Some("/path/to/desired.csv"));
//
// println!("Total Txs: {}", stats.count);
// println!("Total Amount: {}", stats.amount);
// println!("Total Secs: {}", stats.total_secs);
//
// Remove RocksDB and start fresh
// utxo_scanner::reset_rocksdb
//
#![crate_type = "lib"]

pub mod database;
pub mod keys;
pub mod scanner;
#[allow(missing_docs)]
//pub mod utxo_scanner;
pub mod utils;
pub mod utxo;

use lazy_static::lazy_static; // 1.4.0
use std::fs;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref BITCOIN_DATADIR: Mutex<String> = Mutex::new("".to_string());
}
/// Scan Bitcoin Core chainstate LevelDB and extract all UTXOs to
//// RocksDB, a CSV file or both.
///
/// Returns a struct that contains the total number of transactions, amount and seconds the process took.
pub fn scan(bitcoin_datadir: &str, create_rocksdb: bool, csv_file: Option<&str>) {
    *BITCOIN_DATADIR.lock().unwrap() = bitcoin_datadir
        .to_string()
        .trim_end_matches("/")
        .to_string();

    scanner::scan(create_rocksdb, csv_file);
}

/// Reset the RocksDB and start fresh when scanning.
pub fn reset_rocksdb(bitcoin_datadir: &str) {
    // Get directory
    let dirname = format!("{}/chainstate", bitcoin_datadir.trim_end_matches("/"));
    if !Path::new(&dirname).exists() {
        return;
    }

    // Remove
    match fs::remove_dir_all(&dirname) {
        Ok(_dir) => {}
        Err(e) => panic!("Unable to remove directory at {}, error: {}.", dirname, e),
    };
}
