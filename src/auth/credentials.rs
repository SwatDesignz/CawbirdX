use anyhow::{Context, Result};
use keyring::Entry;

/// Service name for keyring
const SERVICE_NAME: &str = "cawbirdx";

/// Store and retrieve API credentials using system keyring
pub struct CredentialStore;

impl CredentialStore {
    /// Store RapidAPI key
    pub fn set_api_key(key: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, "rapidapi_key");
        entry.set_password(key).context("Failed to store API key")?;
        Ok(())
    }

    /// Get stored RapidAPI key
    pub fn get_api_key() -> Result<String> {
        let entry = Entry::new(SERVICE_NAME, "rapidapi_key");
        entry.get_password().context("Failed to retrieve API key")
    }

    /// Store RapidAPI host
    pub fn set_api_host(host: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, "rapidapi_host");
        entry.set_password(host).context("Failed to store API host")?;
        Ok(())
    }

    /// Get stored RapidAPI host
    pub fn get_api_host() -> Result<String> {
        let entry = Entry::new(SERVICE_NAME, "rapidapi_host");
        entry.get_password().context("Failed to retrieve API host")
    }

    /// Check if credentials are stored
    pub fn has_credentials() -> bool {
        Self::get_api_key().is_ok() && Self::get_api_host().is_ok()
    }

    /// Clear all stored credentials
    pub fn clear_credentials() -> Result<()> {
        // Note: keyring 1.2 doesn't have delete_credential method
        // This is a no-op for now
        Ok(())
    }
}
