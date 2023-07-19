
use candid::{CandidType, Deserialize, Principal};

use crate::types::{CommonError};

pub const CANISTER_ID_HASH_LEN_IN_BYTES: usize = 10;
const TOKEN_ID_PREFIX: [u8; 4] = [10, 116, 105, 100]; //b"\x0Atid"

#[derive(
    Default, Deserialize, Copy, CandidType, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd,
)]
pub struct TokenIndex(pub u32);

impl TokenIndex {
    pub fn get_value(&self) -> u32 {
        self.0
    }
}
#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum ShikuError {
    #[allow(non_camel_case_types)]
    ok(String),
    #[allow(non_camel_case_types)]
    err(String),    
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TokenObj {
    pub index: TokenIndex,
    pub canister: Vec<u8>,
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct CanisterId(pub Principal);

pub type TokenIdentifier = String;

// pub fn is_valid_token_id(tid: &TokenIdentifier, p: &CanisterId) -> bool {
//     let t_parsed = decode_token_id(tid);
//     match t_parsed {
//         Ok(t) => t.canister == p.0.as_slice().to_vec(),
//         Err(_) => false,
//     }
// }

pub fn get_token_index(tid: &TokenIdentifier) -> TokenIndex {
    let tobj = decode_token_id(tid).unwrap();
    tobj.index
}
pub type NFTServiceResult<T> = anyhow::Result<T, CommonError>;


pub fn decode_token_id(tid: &TokenIdentifier) -> NFTServiceResult<TokenObj> {
    let principal_parse_res = Principal::from_text(tid);
    match principal_parse_res {
        Ok(principal) => {
            let bytes = principal.as_slice();
            if !bytes.starts_with(&TOKEN_ID_PREFIX) {
                return Err(CommonError::Other(format!("This is not TokenIdentifier")));
            }
            let canister: Vec<u8> = bytes[4..(4 + CANISTER_ID_HASH_LEN_IN_BYTES)].to_vec();
            let mut token_index: [u8; 4] = Default::default();
            token_index.copy_from_slice(&bytes[14..]);

            return Ok(TokenObj {
                index: TokenIndex(u32::from_be_bytes(token_index)),
                canister,
            });
        }
        Err(_) => return Err(CommonError::InvalidToken(tid.to_owned())),
    }
}


pub fn encode_token_id(canister_id: CanisterId, token_index: TokenIndex) -> TokenIdentifier {
    let mut blob: Vec<u8> = Vec::new();
    blob.extend_from_slice(&TOKEN_ID_PREFIX);
    blob.extend_from_slice(canister_id.0.as_slice());
    blob.extend_from_slice(&token_index.0.to_be_bytes());
    Principal::from_slice(blob.as_slice()).to_text()
}