use super::errors::Result;
use super::*;
use futures::Future;
use jsonrpc_core_client::transports::ws;
use jsonrpc_core_client::{RpcChannel, RpcError, TypedClient};
use jsonrpc_pubsub::{typed::Subscriber, SubscriptionId};
use parity_codec::Decode;
use sr_rpc::chain::*;
use tokio;

use std::cell::RefCell;

pub struct SubstrateClient<Number, Hash, Header, SignedBlock> {
    runtime: tokio::runtime::Runtime,
    chain_client: RefCell<ChainClient<Number, Hash, Header, SignedBlock>>,
}

impl<Number, Hash, Header, SignedBlock> SubstrateClient<Number, Hash, Header, SignedBlock>
where
    Hash: serde::de::DeserializeOwned
        + serde::ser::Serialize
        + std::marker::Send
        + std::marker::Sync
        + serde::de::DeserializeOwned,
    Header: std::marker::Send + std::marker::Sync + serde::de::DeserializeOwned,
    Number: serde::ser::Serialize + std::marker::Send + std::marker::Sync,
    SignedBlock: std::marker::Send + std::marker::Sync + serde::de::DeserializeOwned,
{
    pub fn new(uri: &str) -> Result<Self> {
        let mut runtime = tokio::runtime::Runtime::new()?;
        let chain_client = runtime
            .block_on(
                ws::connect::<ChainClient<Number, Hash, Header, SignedBlock>>(uri)
                    .map_err(|e| <errors::Error as From<_>>::from(e))?,
            )
            .map_err(|e| <errors::Error as From<_>>::from(e))?;

        Ok(SubstrateClient {
            runtime: runtime,
            chain_client: RefCell::new(chain_client),
        })
    }

    /// Get hash of the n-th block in the canon chain.
    ///
    /// By default returns latest block hash.
    pub fn latest_block_hash(&self) -> Result<Option<Hash>> {
        self.chain_client
            .borrow_mut()
            .block_hash(None)
            .wait()
            .map_err(Into::into)
    }
}

impl<Number, Hash, Header, SignedBlock> SubstrateClient<Number, Hash, Header, SignedBlock> {
    pub fn shutdown(self) -> std::result::Result<(), ()> {
        self.runtime.shutdown_now().wait()
    }
}
