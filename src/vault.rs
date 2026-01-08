use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use colored::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultEntry {
    pub network_version: String,
    pub circuit_name: String,
    pub verification_key: String,
}

pub struct Vault {
    pub entries: HashMap<String, VaultEntry>,
}

impl Vault {
    /// Initializes the Vault and seeds it with current network version information.
    pub fn new() -> Self {
        let mut entries = HashMap::new();
        
        // Seeding the vault with the latest Devnet 3.0.0 information
        let devnet_entry = VaultEntry {
            network_version: "3.0.0-devnet.20251212".to_string(),
            circuit_name: "RootRollup".to_string(),
            verification_key: "0xdeadbeef...7720761".to_string(),
        };
        
        entries.insert(devnet_entry.network_version.clone(), devnet_entry);
        
        Self { entries }
    }

    /// Displays the full registry of verification keys in the terminal.
    pub fn list_entries(&self) {
        println!("{}", "Historical Verification Key Registry:".bold().cyan());
        println!("{}", "--------------------------------------".cyan());
        for entry in self.entries.values() {
            println!(
                "Version: {} | Circuit: {} | VK: {}",
                entry.network_version.green(),
                entry.circuit_name.yellow(),
                entry.verification_key
            );
        }
    }

    /// Retrieves a specific verification key entry based on the network version.
    #[allow(dead_code)]
    pub fn get_vk_for_version(&self, version: &str) -> Option<&VaultEntry> {
        self.entries.get(version)
    }
}
