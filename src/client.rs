use super::errors::Result;
use super::*;
use futures::Future;
use jsonrpc_core_client::transports::ws;
use metadata::RuntimeMetadataPrefixed;
use sr_rpc::{
    chain::ChainClient,
    state::StateClient,
};
use tokio;

use parity_codec::Decode;
use std::cell::RefCell;

pub struct SubstrateClient<Number, Hash, Header, SignedBlock> {
    runtime: tokio::runtime::Runtime,
    chain_client: RefCell<ChainClient<Number, Hash, Header, SignedBlock>>,
    state_client: RefCell<StateClient<Hash>>,
}

impl<'a, Number, Hash, Header, SignedBlock> SubstrateClient<Number, Hash, Header, SignedBlock>
where
    Hash: serde::ser::Serialize
        + std::marker::Send
        + std::marker::Sync
        + serde::de::DeserializeOwned
        + 'static,
    Header: std::marker::Send + std::marker::Sync + serde::de::DeserializeOwned +'static,
    Number: serde::ser::Serialize + std::marker::Send + std::marker::Sync + serde::de::DeserializeOwned + 'static,
    SignedBlock: std::marker::Send + std::marker::Sync + serde::de::DeserializeOwned + 'static,
{
    pub fn new(uri: &str) -> Result<Self> {
        let mut runtime = tokio::runtime::Runtime::new()?;
        let chain_client = runtime
            .block_on(
                ws::connect::<ChainClient<Number, Hash, Header, SignedBlock>>(uri)
                    .map_err(|e| <errors::Error as From<_>>::from(e))?,
            )
            .map_err(|e| <errors::Error as From<_>>::from(e))?;

        let state_client = runtime
            .block_on(
                ws::connect::<StateClient<Hash>>(uri)
                    .map_err(|e| <errors::Error as From<_>>::from(e))?,
            )
            .map_err(|e| <errors::Error as From<_>>::from(e))?;

        Ok(SubstrateClient {
            runtime: runtime,
            chain_client: RefCell::new(chain_client),
            state_client: RefCell::new(state_client),
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

    /// Returns the runtime metadata as an opaque blob.
    pub fn metadata(&self, hash: Option<Hash>) -> Result<RuntimeMetadataPrefixed> {
        self.state_client
            .borrow_mut()
            .metadata(hash)
            .wait()
            .and_then(|b| Ok(RuntimeMetadataPrefixed::decode(&mut &(*b)).expect("can not decoded")))
            .map_err(Into::into)
    }
}

impl<Number, Hash, Header, SignedBlock> SubstrateClient<Number, Hash, Header, SignedBlock> {
    pub fn shutdown(self) -> std::result::Result<(), ()> {
        self.runtime.shutdown_now().wait()
    }
}
