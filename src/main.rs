use futures::Future;
use jsonrpc_core_client::transports::ws;
use jsonrpc_core_client::{RpcChannel, RpcError, TypedClient};
use metadata::RuntimeMetadataPrefixed;
use parity_codec::Decode;
use sr_primitives::generic;
use substrate_primitives::{Bytes, H256};
use std::time::Duration;

use node_primitives::{BlockNumber, Hash};
use node_runtime::{Header, Block};

mod client;
mod errors;

const LOCAL_URL: &str = "ws://127.0.0.1:9944";


fn main() {
    let client = client::SubstrateClient::<BlockNumber, Hash, Header, generic::SignedBlock<Block>>::new(LOCAL_URL).unwrap();
    let hash = client.latest_block_hash();
    println!("Get Hash: {:?}", hash);
}
