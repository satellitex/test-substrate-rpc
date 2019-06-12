use jsonrpc_core_client::transports::ws;
use jsonrpc_core_client::{RpcError, TypedClient, RpcChannel};
use metadata::RuntimeMetadataPrefixed;
use parity_codec::Decode;
use sr_primitives::{H256, Bytes};
use std::time::Duration;
use futures::Future;
use tokio;

#[derive(Clone)]
struct SubstrateClient(TypedClient);

impl From<RpcChannel> for SubstrateClient {
    fn from(channel: RpcChannel) -> Self {
        SubstrateClient(channel.into())
    }
}

impl SubstrateClient {
    fn get_block_hash(&self) -> impl Future<Item = H256, Error = RpcError> {
        self.0.call_method("chain_getBlockHash", "H256", ())
    }
    fn get_metadata(&self, msg: H256) -> impl Future<Item = Bytes, Error = RpcError> {
        self.0.call_method("state_getMetadata", "Bytes", (msg,))
    }
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    // Connect to the url and call the closure
    let call = ws::connect("ws://127.0.0.1:9944")
        .expect("expected client")
        .and_then(|client: SubstrateClient| {
            client.get_block_hash().then(move |res| {
                let _ = tx.send(res);
                Ok(())
            })
        })
        .map_err(|e| panic!("RPC Client error: {:?}", e));

    tokio::run(call);

    // then
    let res = rx.recv_timeout(Duration::from_secs(3)).unwrap();

    if let Err(RpcError::Other(err)) = res {
        panic!("Expected a tokio::Error {:?}", err)
    } else {
        println!("Expected JsonRpcError. Received {:?}", res)
    }

    let (tx, rx) = std::sync::mpsc::channel();
    let call = ws::connect("ws://127.0.0.1:9944")
        .expect("expected client")
        .and_then(|client: SubstrateClient| {
            client.get_metadata(res.unwrap()).then(move |res| {
                let _ = tx.send(res);
                Ok(())
            })
        })
        .map_err(|e| panic!("RPC Client error: {:?}", e));

    tokio::run(call);

    // then
    let res = rx.recv_timeout(Duration::from_secs(3)).unwrap();

    if let Ok(binary) = res {
        let metadata = RuntimeMetadataPrefixed::decode(&mut &(*binary)).expect("unk3");
        println!("Expected JsonRpcError. Received Metadata: {:?}.", metadata);
    } else {
        panic!("Expected a Error {:?}", res)
    }
}
