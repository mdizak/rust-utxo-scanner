
# UTXO Scanner

Scans the chainstate LevelDB from Bitcoin Core, extracts all UTXOs, and 
places them in RocksDB, a CSV file or both.

## Usage

~~~
use utxo_scanner

//Scan for all UTXOs
let stats = utxo_scanner::scan("/path/to/.bitcoin", true, Some("/path/to/desired.csv"));

println!("Total Txs: {}", stats.count);
println!("Total Amount: {}", stats.amount);
println!("Total Secs: {}", stats.total_secs);

// Remove RocksDB and start fresh
utxo_scanner::reset_rocksdb


https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata
~~~

