mod instance;

use solana_sdk::signer::keypair;
use solana_sdk::signature::Signer;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use rust_client::Wallet;
use rust_client::instruction_builder;
use rust_client::transaction_builder;

use std::io::Cursor;
use std::str::FromStr;


#[no_mangle]
pub extern "C" fn test_key_pair(content: *const libc::c_char) -> *const libc::c_char {
    let c_str = unsafe { std::ffi::CStr::from_ptr(content) };
    let content = c_str.to_str().unwrap();

    let mut reader = Cursor::new(content.as_bytes());
    let keypair = keypair::read_keypair(&mut reader);

    match keypair {
        Ok(keypair) => {
            let pubkey = keypair.pubkey();
            let pubkey = pubkey.to_string();
            return std::ffi::CString::new(format!("Address: {:?}", pubkey)).unwrap().into_raw();
        },
        Err(e) => {
            return std::ffi::CString::new(format!("Error: {:?}", e)).unwrap().into_raw();
        }
    }
}

#[no_mangle]
pub extern "C" fn test_rpc(content: *const libc::c_char) -> *const libc::c_char {
    let c_str = unsafe { std::ffi::CStr::from_ptr(content) };
    let content = c_str.to_str().unwrap();

    let lowercase = content.to_lowercase();

    let url = match lowercase.as_str() {
        "localhost" => "http://localhost:8899".to_string(),
        "testnet" => "https://api.testnet.solana.com".to_string(),
        "devnet" => "https://api.devnet.solana.com".to_string(),
        "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
        other => other.to_string()
    };

    let client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());

    let block_height = client.get_block_height();
    match block_height {
        Ok(block_height) => {
            return std::ffi::CString::new(format!("block_height: {:?}", block_height)).unwrap().into_raw();
        },
        Err(e) => {
            return std::ffi::CString::new(format!("Error: {:?}", e)).unwrap().into_raw();
        }
        
    }
}

#[no_mangle]
pub extern "C" fn save_config(rpc: *const libc::c_char, keypair: *const libc::c_char) -> *const libc::c_char {
    let c_str = unsafe { std::ffi::CStr::from_ptr(rpc) };
    let content = c_str.to_str().unwrap();
    let lowercase = content.to_lowercase();
    let url = match lowercase.as_str() {
        "localhost" => "http://localhost:8899".to_string(),
        "testnet" => "https://api.testnet.solana.com".to_string(),
        "devnet" => "https://api.devnet.solana.com".to_string(),
        "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
        other => other.to_string()
    };
    let client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());


    let c_str = unsafe { std::ffi::CStr::from_ptr(keypair) };
    let content = c_str.to_str().unwrap();
    let mut reader = Cursor::new(content.as_bytes());
    let keypair = keypair::read_keypair(&mut reader);
    if let Ok(value) = keypair {
        instance::set_instance(Wallet { client: client, payer: value});
        return std::ffi::CString::new("OK").unwrap().into_raw();
    } else {
        return std::ffi::CString::new("Error: Invalid keypair").unwrap().into_raw();
    }
}

#[no_mangle]
pub extern "C" fn balance(address: *const libc::c_char) -> *const libc::c_char {
    let c_str = unsafe { std::ffi::CStr::from_ptr(address) };
    let content = c_str.to_str().unwrap();

    let pubkey = Pubkey::from_str(content).unwrap();


    let wallet = instance::get_instance().lock().unwrap();
    if (*wallet).is_some() {
        let result = wallet.as_ref().unwrap().client.get_balance(&pubkey);

        match result {
            Ok(balance) => {
                return std::ffi::CString::new(format!("balance: {:?}", balance)).unwrap().into_raw();
            },
            Err(e) => {
                return std::ffi::CString::new(format!("Error: {:?}", e)).unwrap().into_raw();
            }
        }
    } else {
        return std::ffi::CString::new("Error: Wallet not found").unwrap().into_raw();
    }
}

#[no_mangle]
pub extern "C" fn transfer_to(address: *const libc::c_char, amount: *const libc::c_char) -> *const libc::c_char {
    let c_str = unsafe { std::ffi::CStr::from_ptr(address) };
    let content = c_str.to_str().unwrap();
    let pubkey = Pubkey::from_str(content).unwrap();

    let c_str = unsafe { std::ffi::CStr::from_ptr(amount) };
    let content = c_str.to_str().unwrap();
    let amount = content.parse::<u64>().unwrap();

    let wallet = instance::get_instance().lock().unwrap();
    if (*wallet).is_some() {

        let instruction1 = instruction_builder::transfer_to(&wallet.as_ref().unwrap(), &pubkey, amount);

        let signing_keypairs = &[&wallet.as_ref().unwrap().payer];
        let transaction = transaction_builder::signed_independent(&wallet.as_ref().unwrap(), &[instruction1], signing_keypairs);

        wallet.as_ref().unwrap().send_transaction(&transaction);

        return std::ffi::CString::new("OK").unwrap().into_raw();

    } else {
        return std::ffi::CString::new("Error: Wallet not found").unwrap().into_raw();
    }
}