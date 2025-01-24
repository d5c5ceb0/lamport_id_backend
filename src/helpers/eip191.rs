use alloy_primitives::{aliases::B256, keccak256, PrimitiveSignature};
use serde::{Deserialize, Serialize};
use crate::common::error::{AppResult,AppError};
use hex::FromHex;

pub fn verify_signature(msg: impl Serialize, sig: &str, expect_address: &str) -> AppResult<bool> {
    let messgae = serde_json::to_string(&msg)?;

    if sig.len() != 132 {
        return Err(AppError::CustomError("Invalid signature length!".to_string()));
    }

    let r = alloy_primitives::FixedBytes(<[u8; 32]>::from_hex(&sig[2..66]).map_err(|_| AppError::InvalidSignature)?);
    let s = alloy_primitives::FixedBytes(<[u8; 32]>::from_hex(&sig[66..130]).map_err(|_| AppError::InvalidSignature)?);
    let v = &sig[130..132] != "1b";
    
    let signature = PrimitiveSignature::from_scalars_and_parity(r, s, v);

    match signature.recover_address_from_msg(messgae) {
        Ok(recovered_address) => {
            Ok(recovered_address.to_string() == *expect_address)
        },
        Err(_) => Ok(false),
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterMessage {
    pub user_name: String,
    pub email: String,
    pub image: String,
    pub address: String,
    pub nonce: String,
}

#[allow(dead_code)]
impl RegisterMessage {
    fn new(username: &str, email: &str, avatar: &str, address: &str, nonce: &str) -> Self {
        Self {
            user_name: username.to_string(),
            email: email.to_string(),
            image: avatar.to_string(),
            address: address.to_string(),
            nonce: nonce.to_string(),
        }
    }

    fn to_signable_message(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn hash_eip191(&self) -> B256 {
        let message = self.to_signable_message();
        let prefixed_message =
            format!("\x19Ethereum Signed Message:\n{}", message.len()) + &message;
        keccak256(prefixed_message)
    }

    fn verify_signature_with_prehash(&self, signature: &PrimitiveSignature) -> bool {
        let hash = self.hash_eip191();
        match signature.recover_address_from_prehash(&hash) {
            Ok(recovered_address) => recovered_address.to_string() == self.address,
            Err(_) => false,
        }
    }

    fn verify_signature_with_msg(&self, signature: &PrimitiveSignature) -> bool {
        match signature.recover_address_from_msg(self.to_signable_message()) {
            Ok(recovered_address) => {
                println!("Recovered address: {}, Expected address: {}", recovered_address, self.address);
                recovered_address.to_string() == self.address
            },
            Err(_) => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use alloy_signer::{Signer, SignerSync};
    use alloy_signer_local::PrivateKeySigner;
    use alloy_primitives::hex;
    use hex::FromHex;
    use super::*;

    #[test]
    fn test_erc191() {
        let signer = PrivateKeySigner::from_slice(hex::decode("227dbb8586117d55284e26620bc76534dfbd2394be34cf4a09cb775d593b6f2b").unwrap().as_slice()).unwrap();
        let signer = signer.with_chain_id(Some(1));

        println!("Signer Address: {}", signer.address());

        let message= RegisterMessage {
            user_name: String::from("john_doe"),
            email: String::from("john.doe@example.com"),
            image: String::from("https://example.com/avatars/john.jpg"),
            address: signer.address().to_string(),
            nonce: String::from("9876543210"),
        };


        let signable_message = message.to_signable_message();
        println!("Signable Message:\n{}", signable_message);

        println!("Signable Message hash:\n{}", message.hash_eip191());

        let signature = signer
            .sign_message_sync(signable_message.as_bytes())
            .expect("Signing failed");

        let is_valid = message.verify_signature_with_prehash(&signature);
        println!("Signature is valid: {}", is_valid);

        let is_valid = message.verify_signature_with_msg(&signature);
        println!("Signature is valid: {},{:?}", is_valid, signature);


        let signature_hex = "0x37629319093de4b3715035ce2fbcda6a60a646f42f073089b9f9db9059f8e3037c3d2b78b0e7f75580ef3a4c6065e27be8c60b4be1a9823973698b2f35a5effa1b";

        if signature_hex.len() != 132 {
            panic!("Invalid signature length!");
        }

        let signature_hex = &signature_hex[2..];

        ////let signature_bytes = hex::decode(signature_hex).expect("Invalid hex string");
        let r = <[u8; 32]>::from_hex(&signature_hex[0..64]).unwrap();
        let s = <[u8; 32]>::from_hex(&signature_hex[64..128]).unwrap();
        let v = &signature_hex[128..130] != "1b";
        //let v = false;
        let sss = PrimitiveSignature::from_scalars_and_parity(alloy_primitives::FixedBytes(r), alloy_primitives::FixedBytes(s), v);
        ////
        println!("{:?}", sss);

        let is_valid = message.verify_signature_with_msg(&sss);
        println!("Signature is valid: {}", is_valid);
    }
}
