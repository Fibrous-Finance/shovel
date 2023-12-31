#![allow(unused, clippy::pedantic)]

//this file probably doesn't belong in this module
use starknet::core::types::FieldElement;

/// felt!("0");
pub const ZERO_FELT: FieldElement = FieldElement::from_mont([0, 0, 0, 0]);

/// felt!("0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9");
pub const TRANSFER_EVENT_KEY: FieldElement = FieldElement::from_mont([
    10370298062762752593,
    7288672513944573579,
    6148261015514870755,
    242125613396778233,
]);

/// felt!("0x182d859c0807ba9db63baf8b9d9fdbfeb885d820be6e206b9dab626d995c433");
pub const TRANSFER_SINGLE_EVENT_KEY: FieldElement = FieldElement::from_mont([
    1986363494579022220,
    17146673375846491535,
    6125027481420860397,
    307829215948623223,
]);

/// felt!("0x2563683c757f3abe19c4b7237e2285d8993417ddffe0b54a19eb212ea574b08");
pub const TRANSFER_BATCH_EVENT_KEY: FieldElement = FieldElement::from_mont([
    14114721770411318090,
    10106114908748783105,
    12894248477188639378,
    518981439849896716,
]);

/// Selector for "name()"
/// felt!("0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60");
pub const NAME_SELECTOR: FieldElement = FieldElement::from_mont([
    4539611826636167848,
    2380157814635835479,
    16059280649635539212,
    204437639094763333,
]);

/// Selector for "symbol()"
/// felt!("0x216b05c387bab9ac31918a3e61672f4618601f3c598a2f3f2710f37053e1ea4");
pub const SYMBOL_SELECTOR: FieldElement = FieldElement::from_mont([
    9178143007560336762,
    17130963829830369960,
    11796451914517155703,
    179664498801601103,
]);

/// Selector for "tokenURI(token_id: Uint256)"
/// felt!("0x12a7823b0c6bee58f8c694888f32f862c6584caa8afa0242de046d298ba684d");
pub const TOKEN_URI_SELECTOR: FieldElement = FieldElement::from_mont([
    2091095801852678759,
    17231162349608376638,
    6222938823409426658,
    1993342367472756,
]);
