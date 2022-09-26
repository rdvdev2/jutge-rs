use ureq::Agent;
use ureq::AgentBuilder;

/// A `Client` to interact with <https://jutge.org>.
///
/// The Client can be configured at construction time using [`Client::builder()`].
#[derive(Debug)]
pub struct Client {
    agent: Agent,
}

impl Client {
    /// Creates a `Client` with default configuration.
    #[must_use]
    pub fn new() -> Self {
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

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// A `ClientBuilder` can be used to create a [`Client`] with custom
/// configuration.
#[derive(Debug)]
pub struct ClientBuilder {
    agent_builder: AgentBuilder,
}

impl ClientBuilder {
    /// Creates a `ClientBuilder` to configure a `Client`.
    ///
    /// This is the same as [`Client::builder()`].
    #[must_use]
    pub fn new() -> Self {
        const APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        let agent_builder = AgentBuilder::new().user_agent(APP_USER_AGENT);

        Self { agent_builder }
    }

    /// Builds a `Client` from this builder.
    #[must_use]
    pub fn build(self) -> Client {
        let agent = self.agent_builder.build();

        Client { agent }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
