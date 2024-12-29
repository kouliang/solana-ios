use rust_client::Wallet;

use std::sync::{Mutex, OnceLock};

static INSTANCE: OnceLock<Mutex<Option<Wallet>>> = OnceLock::new();

pub fn get_instance() -> &'static Mutex<Option<Wallet>> {
    INSTANCE.get_or_init(|| {
        Mutex::new(None)
    })
}

pub fn set_instance(wallet: Wallet) {
    let mut config = get_instance().lock().unwrap();
    *config = Some(wallet);
}
