use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::{
    models::BlockId, models::EventFilter, HttpTransport, JsonRpcClient,
};

use dotenv::dotenv;
use reqwest::Url;
use std::env;

/// felt!("0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9");
const TRANSFER_EVENT_KEY: FieldElement = FieldElement::from_mont([
    10370298062762752593,
    7288672513944573579,
    6148261015514870755,
    242125613396778233,
]);

/// felt!("0x182d859c0807ba9db63baf8b9d9fdbfeb885d820be6e206b9dab626d995c433");
const TRANSFER_SINGLE_EVENT_KEY: FieldElement = FieldElement::from_mont([
    1986363494579022220,
    17146673375846491535,
    6125027481420860397,
    307829215948623223,
]);

/// felt!("0x2563683c757f3abe19c4b7237e2285d8993417ddffe0b54a19eb212ea574b08");
const TRANSFER_BATCH_EVENT_KEY: FieldElement = FieldElement::from_mont([
    14114721770411318090,
    10106114908748783105,
    12894248477188639378,
    518981439849896716,
]);

pub async fn run() {
    dotenv().ok();

    let rpc_url = env::var("STARKNET_GOERLI2_RPC").expect("configure your .env file");
    let rpc = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));

    let keys: Vec<FieldElement> = Vec::from([
        TRANSFER_EVENT_KEY,
        TRANSFER_SINGLE_EVENT_KEY,
        TRANSFER_BATCH_EVENT_KEY,
    ]);

    let filter: EventFilter = EventFilter {
        from_block: Some(BlockId::Number(44788)),
        to_block: Some(BlockId::Number(44790)),
        address: None,
        keys: Some(keys),
    };

    let events = rpc.get_events(filter, None, 64).await.unwrap();

    dbg!(events);
}
