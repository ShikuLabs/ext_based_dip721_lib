use candid::{CandidType, Principal};
use serde::{de, de::Error, Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter},
    str::FromStr,
};

use ic_ledger_types::AccountIdentifier;
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct ShikuAccountIdentifier(pub AccountIdentifier);

impl Default for ShikuAccountIdentifier {
    fn default() -> Self {
        Self(AccountIdentifier::new(&Principal::anonymous(), &ic_ledger_types::Subaccount([0u8;32])))
    }
}

impl ShikuAccountIdentifier {


    pub fn from_hex(hex_str: &str) -> Result<ShikuAccountIdentifier, String> {
        let hex: Vec<u8> = hex::decode(hex_str).map_err(|e| e.to_string())?;
        Self::from_slice(&hex[..]).map_err(|err| match err {
            // Since the input was provided in hex, return an error that is hex-friendly.
            AccountIdParseError::InvalidLength(_) => format!(
                "{} has a length of {} but we expected a length of 64 or 56",
                hex_str,
                hex_str.len()
            ),
            AccountIdParseError::InvalidChecksum(err) => err.to_string(),
        })
    }

    /// Converts a blob into an `AccountIdentifier`.
    ///
    /// The blob can be either:
    ///
    /// 1. The 32-byte canonical format (4 byte checksum + 28 byte hash).
    /// 2. The 28-byte hash.
    ///
    /// If the 32-byte canonical format is provided, the checksum is verified.
    pub fn from_slice(v: &[u8]) -> Result<ShikuAccountIdentifier, AccountIdParseError> {
        // Try parsing it as a 32-byte blob.
        match v.try_into() {
            Ok(h) => {
                // It's a 32-byte blob. Validate the checksum.
                check_sum(h).map_err(AccountIdParseError::InvalidChecksum)
            }
            Err(_) => Err(AccountIdParseError::InvalidLength(v.to_vec())), 
        }
    }

    pub fn to_hex(&self) -> String {
        let data = self.0;
        //hex::encode(data.as_ref().to_vec())
        data.to_string()
    }

    /// Converts this account identifier into a binary "address".
    /// The address is CRC32(identifier) . identifier.
    pub fn to_address(&self) -> [u8; 32] {
        let mut result = [0u8; 32];
        result[0..4].copy_from_slice(&self.generate_checksum());
        result[4..32].copy_from_slice(&[0u8]);
        result
    }

    /// Tries to parse an account identifier from a binary address.
    pub fn from_address(blob: [u8; 32]) -> Result<Self, ChecksumError> {
        check_sum(blob)
    }

    // pub fn to_vec(&self) -> Vec<u8> {
    //     let shiku_aid = AccountIdentifier::try_from([0u8;32]).unwrap();

    //     [&self.generate_checksum()[..], &ShikuAccountIdentifier(shiku_aid)[..]].concat()
    // }

    pub fn generate_checksum(&self) -> [u8; 4] {
        let mut hasher = crc32fast::Hasher::new();
        let data_hash = self.0.as_ref();

        hasher.update(&data_hash[4..32]);
        hasher.finalize().to_be_bytes()
    }

}

impl FromStr for ShikuAccountIdentifier {
    type Err = String;

    fn from_str(s: &str) -> Result<ShikuAccountIdentifier, String> {
        ShikuAccountIdentifier::from_hex(s)
    }
}

impl Serialize for ShikuAccountIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_hex().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ShikuAccountIdentifier {
    // This is the canonical way to read a this from string
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        D::Error: de::Error,
    {
        let hex: [u8; 32] = hex::serde::deserialize(deserializer)?;
        check_sum(hex).map_err(D::Error::custom)
    }
}


fn check_sum(hex: [u8; 32]) -> Result<ShikuAccountIdentifier, ChecksumError> {
    // Get the checksum provided
    let found_checksum = &hex[0..4];

    // Copy the hash into a new array
    //let mut hash = [0; 28];
    //hash.copy_from_slice(&hex[4..32]);
    let shiku_aid = AccountIdentifier::try_from(hex).unwrap();

    let account_id = ShikuAccountIdentifier(shiku_aid);

    let expected_checksum = account_id.generate_checksum();

    // Check the generated checksum matches
    if expected_checksum == found_checksum {
        Ok(account_id)
    } else {
        Err(ChecksumError {
            input: hex,
            expected_checksum,
            found_checksum: found_checksum.try_into().unwrap(),
        })
    }
}

impl CandidType for ShikuAccountIdentifier {
    // The type expected for account identifier is
    fn _ty() -> candid::types::Type {
        String::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        self.to_hex().idl_serialize(serializer)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChecksumError {
    input: [u8; 32],
    expected_checksum: [u8; 4],
    found_checksum: [u8; 4],
}

impl Display for ChecksumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Checksum failed for {}, expected check bytes {} but found {}",
            hex::encode(&self.input[..]),
            hex::encode(self.expected_checksum),
            hex::encode(self.found_checksum),
        )
    }
}

/// An error for reporting invalid Account Identifiers.
#[derive(Debug, PartialEq, Eq)]
pub enum AccountIdParseError {
    InvalidChecksum(ChecksumError),
    InvalidLength(Vec<u8>),
}



impl Display for AccountIdParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChecksum(err) => write!(f, "{}", err),
            Self::InvalidLength(input) => write!(
                f,
                "Received an invalid AccountIdentifier with length {} bytes instead of the expected 28 or 32.",
                input.len()
            ),
        }
    }
}



