#[derive(Debug, Clone)]
pub enum AddressType {
    NonStandard,
    P2pkh,
    P2sh,
    Bech32
}

#[derive(Clone, Debug)]
pub struct Utxo {
    pub txid: String,
    pub vout: usize,
    pub coinbase: u64,
    pub height: u64,
    pub amount: u64,
    pub address: String,
    pub address_type: AddressType,
    pub sigscript: String,
}

impl Utxo {
    pub fn new(
        txid: String,
        vout: usize,
        coinbase: u64,
        height: u64,
        amount: u64,
        address_type: AddressType,
        address: String,
        sigscript: String,
    ) -> Self {
        Self {
            txid,
            vout,
            coinbase,
            height,
            amount,
            address_type,
            address,
            sigscript,
        }
    }

    pub fn get_rocksdb_line(&self) -> String {
        let line = vec![
            format!("{}", self.txid),
            format!("{}", self.vout),
            format!("{}", self.amount),
            format!("{}", self.address),
            format!("{}", self.sigscript),
            format!("{}", self.height),
            format!("{}", self.coinbase),
        ]
        .join(",");

        line
    }

    pub fn get_csv_line(&self) -> String {
        let line = vec![
            format!("{}", self.txid),
            format!("{}", self.vout),
            format!("{}", self.amount),
            format!("{}", self.address),
            format!("{}", self.height),
            format!("{}", self.coinbase),
            format!("{}", self.sigscript),
        ]
        .join(",");

    format!("{}\n", line)
    }
}
