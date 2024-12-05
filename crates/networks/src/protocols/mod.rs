// TODO: Refactor this i dont want to put this here, only import all the NetworkManager crate
pub use self::http::HttpClient;

pub use crate::NetworkManager;
pub mod http;
