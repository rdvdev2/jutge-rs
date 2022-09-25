use crate::Result;

/// A `Client` to interact with <https://jutge.org>.
///
/// The Client can be configured at construction time using [`Client::builder()`].
#[derive(Debug)]
pub struct Client {
    handle: reqwest::Client,
}

impl Client {
    /// Creates a `Client` with default configuration.
    ///
    /// # Errors
    /// This method builds a [`reqwest::Client`], and thus can return a [`reqwest::Error`]
    /// wrapped within an [`Error::ReqwestError`](crate::Error::ReqwestError)
    pub fn new() -> Result<Self> {
        ClientBuilder::new().build()
    }

    /// Creates a `ClientBuilder` to configure a `Client`.
    ///
    /// This is the same as [`ClientBuilder::new()`].
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

/// A `ClientBuilder` can be used to create a [`Client`] with custom
/// configuration.
#[derive(Debug)]
pub struct ClientBuilder {
    handle_builder: reqwest::ClientBuilder,
}

impl ClientBuilder {
    /// Creates a `ClientBuilder` to configure a `Client`.
    ///
    /// This is the same as [`Client::builder()`].
    #[must_use]
    pub fn new() -> Self {
        const APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        let handle_builder = reqwest::ClientBuilder::new().user_agent(APP_USER_AGENT);

        Self { handle_builder }
    }

    /// Builds a `Client` from this builder.
    ///
    /// # Errors
    /// This method builds a [`reqwest::Client`], and thus can return a [`reqwest::Error`]
    /// wrapped within an [`Error::ReqwestError`](crate::Error::ReqwestError)
    pub fn build(self) -> Result<Client> {
        let handle = self.handle_builder.build()?;

        Ok(Client { handle })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
