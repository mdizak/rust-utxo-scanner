//#![allow(warnings)]
use crate::database::{Database, RocksDB};
use crate::utxo::{AddressType, Utxo};
use crate::{keys, utils, BITCOIN_DATADIR};
use lazy_static::lazy_static;
use leveldb::database::Database as LevelDatabase;
use leveldb::iterator::Iterable;
use leveldb::options::{Options, ReadOptions};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

lazy_static! {
    pub static ref ROCKSDB: RocksDB = Database::new();
}
pub struct Stats {
    pub count: u64,
    pub amount: f64,
    pub total_secs: u64,
}

pub fn scan(create_rocksdb: bool, csv_file: Option<&str>, testnet: bool) -> Stats {

    // Initialize
    let chainstate_dir = format!("{}/chainstate", BITCOIN_DATADIR.lock().unwrap());

    // Open CSV file, if needed
    let mut _csv_fh = None;
    if let Some(csv_filename) = csv_file {
        let csv_path = Path::new(&csv_filename);
        let mut fh = match File::create(&csv_path) {
            Ok(r) => r,
            Err(e) => panic!(
                "Unable to open file for writing, {}, error: {}",
                csv_filename, e
            ),
        };

        // Header line
        let header_line = "TXID,VOUT,AMOUNT,ADDRESS,HEIGHT,COINBASE,SIGSCRIPT\n";
        fh.write_all(&header_line.as_bytes());
        _csv_fh = Some(fh);
    }

    // Set options
    let mut options = Options::new();
    options.create_if_missing = true;

    // Open the database
    let path = Path::new(&chainstate_dir);
    let db: LevelDatabase<TxKey> = match LevelDatabase::open(path, options) {
        Ok(r) => r,
        Err(e) => panic!("Unable to open LevelDB, error: {}", e),
    };

    // Set variables
    let mut obfuscate_key: Vec<u8> = Vec::new();
    let mut total = 1;
    let mut total_amount = 0;
    let start_time = Instant::now();
    let read_opts = ReadOptions::new();

    // Iterate through key-value pairs
    let iter = db.iter(read_opts);
    for (k, v) in iter {

        // Check first byte -
        if k.key[0] == 14 {
            obfuscate_key = v;
            obfuscate_key.remove(0);
            continue;

        // utxo
        } else if k.key[0] == 67 {
            // Deobfuscate the leveldb value
            let value: Vec<u8> = utils::deobfuscate(&obfuscate_key, &v);
            let offset = 0;

            // Decode txid and vout
            let (txid, vout) = decode_utxo_key(&k.key);

            // Read first chunk, get blockheight and coinbase
            let (first_chunk, offset) = utils::read_chunk(&value, &offset);
            let height = first_chunk >> 1;
            let coinbase = first_chunk & 1;

            // Get second chunk and amount
            let (second_chunk, offset) = utils::read_chunk(&value, &offset);
            let amount = utils::convert_amount(second_chunk);

            // Get third chunk
            let (script_type, mut offset) = utils::read_chunk(&value, &offset);

            // Sutract 1 from offset, if needed
            if script_type > 1 && script_type < 6 {
                offset = offset - 1;
            }

            // Get remaining bytes
            let mut _sigscript = &value[offset..value.len()].to_vec();

            // Decompress public key, if needed
            if script_type == 4 || script_type == 5 {
                let _sigscript = match &keys::decompress(&_sigscript) {
                    Some(r) => r,
                    None => continue,
                };
            }

            // Get address
            let (addr_type, address) = get_address(&script_type, &_sigscript, &testnet);

            // Define utxo
            let utxo: Utxo = Utxo::new(
                utils::bin2hex(&txid),
                vout,
                coinbase,
                height,
                amount,
                addr_type,
                address,
                utils::bin2hex(&_sigscript.to_vec()),
            );

            // Add utxo
            add_utxo(&utxo, &create_rocksdb, &_csv_fh);

            // Add tot totals
            total += 1;
            total_amount += amount;

            if total % 10000 == 0 {
                print!(".");
                io::stdout().flush().unwrap();
            }
        }
    }

    // Set stats
    let final_stats = Stats {
        count: total,
        amount: total_amount as f64 / 10000000.0,
        total_secs: start_time.elapsed().as_secs(),
    };

    final_stats
}

fn get_address(script_type: &u64, sigscript: &Vec<u8>, testnet: &bool) -> (AddressType, String) {
    // Default address vars
    let mut addr_type = AddressType::NonStandard;
    let mut address = String::new();


    // Bech32
    if *script_type == 28 && sigscript[0] == 0 as u8 && sigscript[1] == 20 as u8 {
        address = keys::bech32_address(&sigscript, &testnet);
        addr_type = AddressType::Bech32;
    // p2sh
    } else if *script_type == 1 {
        address = keys::standard_address(&sigscript, true, &testnet);
        addr_type = AddressType::P2sh;

    // Standard
    } else {
        address = keys::standard_address(&sigscript, false, &testnet);
        addr_type = AddressType::P2pkh;
    }

    (addr_type, address)
}

fn decode_utxo_key(key: &Vec<u8>) -> (Vec<u8>, usize) {
    // Reverse txid
    let mut rev_txid: Vec<u8> = Vec::new();
    for x in 1..33 {
        rev_txid.insert(0, key[x]);
    }

    // Decode
    //let txid: String = utils::bin2hex(&rev_txid);
    let vout = key[33] as usize;

    (rev_txid, vout)
}

#[derive(Debug)]
struct TxKey {
    key: Vec<u8>,
}

impl db_key::Key for TxKey {
    fn from_u8(key: &[u8]) -> Self {
        TxKey {
            key: Vec::from(key),
        }
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        f(&self.key)
    }
}

fn add_utxo(utxo: &Utxo, create_rocksdb: &bool, csv_fh: &Option<File>) {

    // RocksDB
    if *create_rocksdb {
        let mut rocksdb_line = utxo.get_rocksdb_line();
        if let Some(current_line) = ROCKSDB.get(&utxo.address) {
            let rocksdb_line = format!("{}\n{}", current_line, rocksdb_line);
        }
    let rocksdb_line = "Hello".to_string();
        ROCKSDB.put(&utxo.address, &rocksdb_line);
    }

    // Check csv
    if csv_fh.is_none() {
        return;
    }

    // Add csv
    let mut fh = csv_fh.as_ref().unwrap();
    let line = utxo.get_csv_line();
    fh.write_all(&line.as_bytes())
        .expect("Unable to write to CSV file.");
}


