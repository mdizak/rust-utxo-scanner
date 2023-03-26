extern crate rocksdb;

use crate::BITCOIN_DATADIR;
use rocksdb::{DBCompressionType, DB, DBCompactionStyle};
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub trait Database {
    fn new() -> Self;
    fn put(&self, header: &String, sequence: &String);
    fn get(&self, header: &String) -> Option<String>;
    fn delete(&self, header: &String) -> bool;
}

#[derive(Clone)]
pub struct RocksDB {
    pub db: Arc<DB>,
}

impl Database for RocksDB {
    fn new() -> Self {
        // Create directory, if needed
        let rocksdb_path = format!("{}/{}", BITCOIN_DATADIR.lock().unwrap(), "rocksdb");
        if !Path::new(&rocksdb_path).exists() {
            match fs::create_dir_all(&rocksdb_path) {
                Ok(_) => {}
                Err(_error) => panic!("Unable to create directory at {}.", rocksdb_path),
            };
        }

        // Set options
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(DBCompressionType::None);
        //opts.set_compaction_style(DBCompactionStyle::Universal);
        opts.increase_parallelism(4);

        // Connect to database
        let database = match DB::open(&opts, rocksdb_path.as_str()) {
            Ok(r) => r,
            Err(e) => panic!("Unable to open RocksDB at {}, error: {}", rocksdb_path, e),
        };

        // Return
        RocksDB {
            db: Arc::new(database),
        }
    }

    fn put(&self, header: &String, sequence: &String) {
        self.db.put(header.as_bytes(), sequence.as_bytes()).unwrap();
    }

    fn get(&self, header: &String) -> Option<String> {
        let sequence = match self.db.get(header.as_bytes()) {
            Ok(Some(r)) => String::from_utf8(r).unwrap(),
            Ok(None) => return None,
            Err(e) => panic!(
                "Received database error when trying to retrieve sequence, error: {}",
                e
            ),
        };

        Some(sequence)
    }

    fn delete(&self, header: &String) -> bool {
        self.db.delete(header.as_bytes()).is_ok()
    }
}
