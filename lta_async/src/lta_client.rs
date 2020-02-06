//! Client for interacting with LTA API
use lta_utils_commons::reqwest::{
    Client as AsyncClient, RequestBuilder as AsyncReqBuilder, Response,
};

use crate::utils::Client as ExperimentalClient;
use async_trait::async_trait;
use lta_utils_commons::serde::Serialize;
use lta_utils_commons::{Client, LTAError};
/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
/// There are some instance where you might need to customise your client due to certain limitations.
///
/// The `Client` trait has a general constructor method and you should use the `reqwest` re-export
/// to build you own customised client from the ground up.
///
/// Take a look at the reqwest documentation on how to build your own client
///
/// ## Example
/// ```rust
/// use lta_utils_commons::reqwest::ClientBuilder;
/// use lta_utils_commons::Client;
/// use std::time::Duration;
/// use lta_async::lta_client::LTAClient;
///
/// fn my_custom_client() -> LTAClient {
///     let client = ClientBuilder::new()
///         .no_gzip()
///         .connect_timeout(Duration::new(420, 0))
///         .build()
///         .unwrap();
///
///     LTAClient::new(Some("api_key".to_string()), client)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct LTAClient {
    api_key: Option<String>,
    client: AsyncClient,
}

impl Client<AsyncClient, AsyncReqBuilder> for LTAClient {
    fn new(api_key: Option<String>, client: AsyncClient) -> LTAClient {
        LTAClient { api_key, client }
    }

    fn with_api_key<S>(api_key: S) -> LTAClient
    where
        S: Into<String>,
    {
        let api_key = api_key.into();
        let api_opt = if api_key.is_empty() {
            None
        } else {
            Some(api_key)
        };

        let client = AsyncClient::new();

        LTAClient {
            api_key: api_opt,
            client,
        }
    }

    fn get_req_builder(&self, url: &str) -> AsyncReqBuilder {
        let api_key = self.api_key.as_ref().expect("Empty API KEY!");
        self.client.get(url).header("AccountKey", api_key.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct AsyncLTAClient {
    api_key: String,
    client: AsyncClient,
}

impl AsyncLTAClient {
    pub fn new(api_key: String) -> Self {
        AsyncLTAClient { api_key, client: AsyncClient::new() }
    }
}


impl ExperimentalClient<AsyncReqBuilder> for AsyncLTAClient {
    fn get_req_builder(&self, url: &str) -> AsyncReqBuilder {
        self.client
            .get(url)
            .header("AccountKey", self.api_key.as_str())
    }
}
