use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;

#[allow(dead_code)]
pub fn gen_uid() -> String {
    Uuid::new_v4().to_string()
}

pub fn gen_lamport_id() -> String {
    "0".to_string()
}

pub fn gen_address() -> String {
    //eth format address :20 bytes hex string with 0x prefix
    format!("0x{}", Uuid::new_v4().to_string().replace("-", "").as_str())
}


pub fn gen_invite_code(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
