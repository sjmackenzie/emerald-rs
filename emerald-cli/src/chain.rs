use emerald::addressbook::Addressbook;
use emerald::keystore::{self, KdfDepthLevel, KeyFile};
use emerald::storage::{ChainStorage, Storages, default_keystore_path};

use serde_json::{self, Map, Value};
use std::path::PathBuf;

use std::str::FromStr;
use std::sync::Arc;

#[derive(Serialize)]
pub struct Account {
    name: String,
    address: String,
}

pub struct Chain {
    security_level: KdfDepthLevel,
    keystore_path: PathBuf,
}

impl Chain {
    pub fn new(base_path_str: &str, sec_level_str: &str) -> Chain {
        let sec_level = match KdfDepthLevel::from_str(sec_level_str) {
            Ok(sec) => sec,
            Err(e) => {
                error!("{}", e.to_string());
                KdfDepthLevel::default()
            }
        };
        let base_path = if !base_path_str.is_empty() {
            Some(PathBuf::from(&base_path_str))
        } else {
            None
        };
        let storage = match base_path {
            Some(p) => Storages::new(p),
            None => Storages::default(),
        };
        if storage.init().is_err() {
            panic!("Unable to initialize storage");
        }
        let chain_storage = ChainStorage::new(&storage, "default".to_string());
        if chain_storage.init().is_err() {
            panic!("Unable to initialize chain");
        }

        Chain {
            security_level: sec_level,
            keystore_path: default_keystore_path(&chain_storage.id),
        }

    }

    pub fn current_version(&self) {
        unimplemented!();
    }

    pub fn list_accounts(&self) {
        unimplemented!();
    }

    pub fn new_account(&self) {
        unimplemented!();
    }

    pub fn hide_account(&self) {
        unimplemented!();
    }

    pub fn shake_account(&self) {
        unimplemented!();
    }

    pub fn update_account(&self) {
        unimplemented!();
    }

    pub fn import_account(&self) {
        unimplemented!();
    }

    pub fn export_account(&self) {
        unimplemented!();
    }

    pub fn sign_transaction(&self) {
        unimplemented!();
    }
}
