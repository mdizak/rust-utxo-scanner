use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rust_base58::ToBase58;
use secp256k1::PublicKey;
use bitcoin_bech32::WitnessProgram;
use bitcoin_bech32::constants::Network;

pub fn decompress(public_key_bytes: &Vec<u8>) -> Option<Vec<u8>> {
    // Load public key
    let public_key = match PublicKey::from_slice(public_key_bytes) {
        Ok(r) => r,
        Err(_) => return None,
    };

    // Decompress
    let decompressed: Vec<u8> = public_key.serialize_uncompressed().to_vec();
    Some(decompressed)
}

pub fn standard_address(sigscript: &Vec<u8>, is_multisig: bool, testnet: &bool) -> String {



    // Initialize
    let mut address = Vec::new();
    if is_multisig && *testnet {
        address.extend(vec![0xc4]);
    } else if is_multisig && !*testnet {
        address.extend(vec![0x05]);
    } else if *testnet && !is_multisig {
        address.extend(vec![0x6F]);
    } else {
            address.extend(vec![0x00]);
    }



    address.extend(sigscript);

    // Get double sha256 hash
    let checksum = double_sha256(&address);
    address.extend(checksum[..4].iter().cloned());

    // Base 58
    address.to_base58()
}

pub fn bech32_address(sigscript: &Vec<u8>, testnet: &bool) -> String {

    // Get network
    let network = if !testnet { Network::Bitcoin } else { Network::Testnet };

    let witness = match WitnessProgram::from_scriptpubkey(&sigscript, network) {
        Ok(r) => r,
        Err(e) => return "".to_string()
    };

    witness.to_address()
}

fn double_sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let mut hash = vec![0; hasher.output_bytes()];
    hasher.input(&bytes);
    hasher.result(&mut hash);
    hasher.reset();
    hasher.input(&hash);
    hasher.result(&mut hash);
    return hash;
}
