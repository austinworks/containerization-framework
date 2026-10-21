//! Registry clients, and the credentials they authenticate with.

mod authentication;
mod keychain_helper;
pub mod registry_client;
mod retry_options;

pub use self::authentication::Authentication;
pub use self::keychain_helper::KeychainHelper;
pub use self::registry_client::RegistryClient;
pub use self::retry_options::RetryOptions;
