use substrate_primitives::H256;

mod client;
mod errors;

const LOCAL_URL: &str = "ws://127.0.0.1:9944";

fn main() {
    let client =
        client::SubstrateClient::<u64, H256 , (), ()>::new(
            LOCAL_URL,
        )
        .unwrap();
    let hash = client.latest_block_hash();
    println!("Get Hash: {:?}", hash);

    let metadata = client.metadata(hash.unwrap());
    println!("Get Metadata: {:?}", metadata);

    client.shutdown();
}