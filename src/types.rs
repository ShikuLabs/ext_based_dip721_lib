use crate::token_identifier;
use ic_cdk::export::candid::{CandidType, Deserialize, Int, Nat};
use ic_cdk::export::Principal;
use serde::Serialize;
use std::collections::HashSet;

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub name: Option<String>,
    pub logo: Option<String>,
    pub symbol: Option<String>,
    pub custodians: Option<HashSet<Principal>>,
    pub cap: Option<Principal>,
}


#[derive(CandidType, Default, Deserialize, Debug, Clone)]
pub struct MetaData {
    pub name: Option<String>,
    pub logo: Option<String>,
    pub symbol: Option<String>,
    pub custodians: HashSet<Principal>,
    pub created_at: u64,
    pub upgraded_at: u64,
}


#[derive(Debug, Deserialize, CandidType)]
pub enum SetMinterResponse {
    #[allow(non_camel_case_types)]
    ok,
    #[allow(non_camel_case_types)]
    err(CommonError),
}

use std::cell::RefCell;
thread_local! {
    //minted token
    static MINTEDID: RefCell<Vec<Nat>> = RefCell::new(Vec::new());

    //minter
    static MINTER: RefCell<Principal> = RefCell::new(ic_cdk::api::caller());

}

pub fn read_minter_state<T, F: FnOnce(&Principal) -> T>(f: F) -> T {
    MINTER.with(|minter: &RefCell<Principal>| f(&minter.borrow()))
}

pub fn change_minter_state<T, F: FnOnce(&mut Principal) -> T>(f: F) -> T {
    MINTER.with(|minter| f(&mut minter.borrow_mut()))
}

pub fn read_minted_state<T, F: FnOnce(&Vec<Nat>) ->T>(f: F) -> T {
    MINTEDID.with(|minted| f(&minted.borrow()))
}

pub fn change_minted_state<T, F: FnOnce(&Vec<Nat>) -> T >(f: F) -> T {
    MINTEDID.with(|minted| f(&minted.borrow_mut()))
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct Status {
    pub total_transactions: Nat,
    pub total_supply: Nat,
    pub cycles: Nat,
    pub total_unique_holders: Nat,
}
#[warn(non_camel_case_types)]
pub type TokenId = Nat;

#[derive(CandidType, Deserialize, Serialize, Clone, PartialEq)]
pub enum GeneralValue {
    BoolContent(bool),
    TextContent(String),
    BlobContent(Vec<u8>),
    Principal(Principal),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    NatContent(Nat),
    Int8Content(i8),
    Int16Content(i16),
    Int32Content(i32),
    Int64Content(i64),
    IntContent(Int),
    FloatContent(f64),
    NestedContent(Vec<(String, GeneralValue)>),
}
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TokenMetadata {
    pub token_identifier: Nat,
    pub owner: Option<AccountIdentifier_shiku>,
    pub operator: Option<AccountIdentifier_shiku>,
    pub is_burned: bool,
    pub properties: Option<MetaData>,
    pub minted_at: u64,
    pub minted_by: AccountIdentifier_shiku,
    pub transferred_at: Option<u64>,
    pub transferred_by: Option<AccountIdentifier_shiku>,
    pub approved_at: Option<u64>,
    pub approved_by: Option<AccountIdentifier_shiku>,
    pub burned_at: Option<u64>,
    pub burned_by: Option<AccountIdentifier_shiku>,
    pub status: u32,
}

#[derive(Debug, CandidType)]
pub enum NftError {
    UnauthorizedOwner,
    UnauthorizedOperator,
    OwnerNotFound,
    OperatorNotFound,
    TokenNotFound,
    ExistedNFT,
    SelfApprove,
}

/////////////// YUMI TYPES ////////////

pub type Time = Int;
pub type TokenIndex = u32;
#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct SubAccount(pub Vec<u8>);

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum User {
    #[allow(non_camel_case_types)]
    address(AccountIdentifier),
    #[allow(non_camel_case_types)]
    principal(Principal),
}

impl User {
    pub fn aid(user: User) -> AccountIdentifier {
        match user {
            Self::address(aid) => aid.clone(),
            Self::principal(pid) => pid2aid(&pid).to_hex(),
        }
    }
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct AllowanceRequest {
    pub owner: User,
    pub spender: Principal,
    pub token: TokenIdentifier__1,
}


#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct ApproveRequest {
    pub allowance: Balance,
    pub spender: Principal,
    pub subaccount: Option<SubAccount>,
    pub token: token_identifier::TokenIdentifier,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result__1_1 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(AccountIdentifier__1),
}

#[allow(non_camel_case_types)]
pub type AccountIdentifier__1 = String;
pub type AccountIdentifier = String;
#[allow(non_camel_case_types)]
pub type TokenIdentifier__1 = String;
#[allow(non_camel_case_types)]
pub type Balance = Nat;

// use ic_ledger_types::AccountIdentifier;
// #[derive(Debug, Clone, Hash, PartialEq, Eq, Copy, CandidType, Deserialize)]
// pub struct ShikuAccountIdentifier(pub ic_ledger_types::AccountIdentifier);

#[allow(non_camel_case_types)]
pub type AccountIdentifier_shiku = crate::account_identifier::ShikuAccountIdentifier;



#[derive(Debug, CandidType, Clone, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CommonError__1 {
    InvalidToken(String),
    Other(String),
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum CommonError {
    InvalidToken(String),
    Other(String),
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct MetaDataFungibleDetails {
    decimals: u8,
    metadata: Option<Vec<u8>>,
    name: String,
    symbol: String,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct MetaDataNonFungibleDetails {
    pub metadata: Option<Vec<u8>>,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum TokenMetaDataExt {
    #[allow(non_camel_case_types)]
    fungible(MetaDataFungibleDetails),
    #[allow(non_camel_case_types)]
    nonfungible(MetaDataNonFungibleDetails),
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct Listing {
    locked: Option<Time>,
    price: u64,
    seller: Principal,
}


#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct Registry(TokenIndex, AccountIdentifier__1);

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum TransferResponse {
    #[allow(non_camel_case_types)]
    err(TransferResponseDetails),
    #[allow(non_camel_case_types)]
    ok(Balance),
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum TransferResponseDetails {
    CannotNotify(AccountIdentifier),
    InsufficientBalance,
    InvalidToken(String),
    Other(String),
    Rejected,
    Unauthorized(AccountIdentifier),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct TransferRequest {
    pub amount: Balance,
    pub from: User,
    pub memo: Memo,
    pub notify: bool,
    pub subaccount: Option<SubAccount>,
    pub to: User,
    pub token: token_identifier::TokenIdentifier,
}

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct TransferRequestV1 {
//     pub amount: Balance,
//     pub from: User,
//     pub memo: Memo,
//     pub notify: bool,
//     pub subaccount: Option<SubAccount>,
//     pub to: User,
//     pub class: String,
//     pub start: usize,
//     pub end: usize,
// }

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct TransferRequestV2 {
//     pub amount: Balance,
//     pub from: User,
//     pub memo: Memo,
//     pub notify: bool,
//     pub subaccount: Option<SubAccount>,
//     pub to: User,
//     pub token_list: Vec<TokenId>,
// }

pub type Memo = Vec<u8>;

#[derive(Debug, Clone, CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result__1_2 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(Balance__1),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result__1 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(TokenMetaDataExt),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_2 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(Balance__1),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_1 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(Vec<TokenIndex>),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ResultDetail(pub TokenIndex, pub Option<Listing>, pub Option<Vec<Vec<u8>>>);

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum NFTResult {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(Vec<ResultDetail>),
}
#[allow(non_camel_case_types)]
pub type Balance__1 = Nat;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum BalanceResponse {
    #[allow(non_camel_case_types)]
    err(CommonError__1),
    #[allow(non_camel_case_types)]
    ok(Balance),
}
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct BalanceRequest {
    token: u64,
    user: User,
}

pub fn pid2aid(pid: &Principal) -> AccountIdentifier_shiku {
    let sub_acc = ic_ledger_types::Subaccount([0u8; 32]);
    let account_id = ic_ledger_types::AccountIdentifier::new(pid, &sub_acc);
     match AccountIdentifier_shiku::from_hex(&account_id.to_string()) {
         Ok(shiku) => shiku,
         Err(_) => AccountIdentifier_shiku::default(),
     }
}
