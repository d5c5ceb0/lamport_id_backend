use alloy_primitives::{utils::keccak256, types::{Address, U256}};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
struct EIP712Domain {
    name: String,
    version: String,
    chain_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    contents: String,
    nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MsgParams {
    types: serde_json::Value,
    primaryType: String,
    domain: EIP712Domain,
    message: Request,
}


fn get_domain_separator(domain: &EIP712Domain) -> Vec<u8> {
    let domain_hash = keccak256(serde_json::to_string(domain).unwrap().as_bytes());
    domain_hash.to_vec()
}

fn get_message_hash(message: &Request) -> Vec<u8> {
    let message_hash = keccak256(serde_json::to_string(message).unwrap().as_bytes());
    message_hash.to_vec()
}


pub fn verify_eip712_signature(signature: &str, person: &Request, domain: &EIP712Domain, expected_address: &str) -> bool {
    let domain_separator = get_domain_separator(domain);
    let message_hash = get_message_hash(person);

    let final_hash = keccak256(&[domain_separator, message_hash].concat());

    let recovered_address = alloy_primitives::utils::recover_from_signature(&final_hash, signature).unwrap();

    recovered_address == Address::from_str(expected_address).unwrap()
}

#[tokio::main]
async fn main() {
    let message = MsgParams {
        types: json!({
            "EIP712Domain": [
                { "name": "name", "type": "string" },
                { "name": "version", "type": "string" },
                { "name": "chainId", "type": "uint256" }
            ],
            "Request": [
                { "name": "contents", "type": "string" },
                { "name": "nonce", "type": "uint256" }
            ]
        }),
        primaryType: "Request".to_string(),
        domain: EIP712Domain {
            name: "Request".to_string(),
            version: "1".to_string(),
            chain_id: 1,  // 主网
        },
        message: Request {
            contents: "This is a test request".to_string(),
            nonce: "0".to_string(),
        },
    };

    let signature = "0x...";
    let expected_address = "0x1234567890abcdef1234567890abcdef12345678"; // 替换为实际地址

    let is_valid = verify_eip712_signature(&signature, &message.message, &message.domain, expected_address);

    if is_valid {
        println!("Signature is valid!");
    } else {
        println!("Invalid signature!");
    }
}
