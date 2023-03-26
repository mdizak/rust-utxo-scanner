#![allow(warnings)]
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
pub mod csv_loader;
pub mod utils;
pub mod utxo;
#[allow(missing_docs)]

use lazy_static::lazy_static; // 1.4.0
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use crate::scanner::Stats;
use std::collections::BTreeMap;

lazy_static! {
    static ref BITCOIN_DATADIR: Mutex<String> = Mutex::new("".to_string());
}
/// Scan Bitcoin Core chainstate LevelDB and extract all UTXOs to
//// RocksDB, a CSV file or both.
///
/// Returns a struct that contains the total number of transactions, amount and seconds the process took.
pub fn scan(bitcoin_datadir: &str, create_rocksdb: bool, csv_file: Option<&str>, testnet: bool) -> Stats {
    *BITCOIN_DATADIR.lock().unwrap() = bitcoin_datadir
        .to_string()
        .trim_end_matches("/")
        .to_string();

    let stats = scanner::scan(create_rocksdb, csv_file, testnet);
    stats
}

/// Reset the RocksDB and start fresh when scanning.
pub fn reset_rocksdb(bitcoin_datadir: &str) {
    // Get directory
    let dirname = format!("{}/rocksdb", bitcoin_datadir.trim_end_matches("/"));
    if !Path::new(&dirname).exists() {
        return;
    }

    // Remove
    match fs::remove_dir_all(&dirname) {
        Ok(_dir) => {}
        Err(e) => panic!("Unable to remove directory at {}, error: {}.", dirname, e),
    };
}


/// Load all UTXOs from a previously generated CSV file into a BTreeMap<String, Vec<u64>> 
/// where the string is the payment address and the vector is the line number(s) that match that address.
pub fn load_from_csv(csv_file: &str) -> BTreeMap<String, Vec<u64>> {
    let map = csv_loader::load(csv_file);
    map
}


