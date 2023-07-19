use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use ic_cdk::api::{time};
use ic_cdk::export::candid::{CandidType, Deserialize, Nat};
use ic_cdk::export::Principal;

use crate::types::*;


thread_local! {
    static LEDGER: RefCell<Ledger> = RefCell::new(Ledger::default());
}

pub fn with<T, F: FnOnce(&Ledger) -> T>(f: F) -> T {
    LEDGER.with(|ledger| f(&ledger.borrow()))
}

pub fn with_mut<T, F: FnOnce(&mut Ledger) -> T>(f: F) -> T {
    LEDGER.with(|ledger| f(&mut ledger.borrow_mut()))
}


#[derive(CandidType, Default, Deserialize)]
pub struct Ledger {
    pub metadata: MetaData,
    pub tokens: HashMap<TokenId, TokenMetadata>,
    pub owners: HashMap<AccountIdentifier_shiku, HashSet<TokenId>>,
    pub operators: HashMap<AccountIdentifier_shiku, HashSet<TokenId>>,
    pub custodians: HashSet<AccountIdentifier_shiku>,
    pub idx2aid: HashMap<TokenIndex, AccountIdentifier__1>,
    pub tx_count: Nat,
}

impl Ledger {
    // pub fn init_metadata(&mut self, default_custodian: Principal, args: Option<InitArgs>) {
    //     let metadata = self.metadata_mut();
    //     metadata.custodians.insert(default_custodian);
    //     if let Some(args) = args {
    //         metadata.name = args.name;
    //         metadata.logo = args.logo;
    //         metadata.symbol = args.symbol;
    //         if let Some(custodians) = args.custodians {
    //             for custodian in custodians {
    //                  metadata.custodians.insert(custodian);
    //             }
    //         }

    //         handshake(1_000_000_000_000, args.cap);
    //     } else {
    //         handshake(1_000_000_000_000, None);
    //     }
    //     metadata.created_at = time();
    //     metadata.upgraded_at = time();
    // }

    pub fn get_registry(&self) -> HashMap<TokenIndex, AccountIdentifier__1> {
        self.idx2aid.clone()
    }

    pub fn metadata(&self) -> &MetaData {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut MetaData {
        &mut self.metadata
    }

    pub fn tokens_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_token_existed(&self, token_identifier: &TokenId) -> bool {
        self.tokens.contains_key(token_identifier)
    }

    pub fn owner_token_identifiers(
        &self,
        owner: &AccountIdentifier_shiku,
    ) -> Result<&HashSet<TokenId>, NftError> {
        self.owners.get(owner).ok_or(NftError::OwnerNotFound)
    }

    pub fn token_metadata(
        &self,
        token_identifier: &TokenId,
    ) -> Result<&TokenMetadata, NftError> {
        self.tokens
            .get(token_identifier)
            .ok_or(NftError::TokenNotFound)
    }

    pub fn add_token_metadata(
        &mut self,
        token_identifier: &TokenId,
        token_metadata: TokenMetadata,
    ) {
        self.tokens.insert(token_identifier.to_owned(), token_metadata);
    }

    pub fn owners_count(&self) -> usize {
        self.owners.len()
    }

    pub fn owner_of(
        &self,
        token_identifier: &TokenId
    ) -> Result<Option<AccountIdentifier_shiku>, NftError> {
        self.token_metadata(token_identifier)
            .map(|token_metadata| token_metadata.owner)
    }

    pub fn update_owner_cache(
        &mut self,
        token_identifier: &TokenId,
        old_owner: Option<AccountIdentifier_shiku>,
        new_owner: Option<AccountIdentifier_shiku>,
    ) {
        if let Some(old_owner) = old_owner {
            let old_owner_token_identifier = self
                .owners
                .get_mut(&old_owner)
                .expect("couldn't find owner");
            
            old_owner_token_identifier.remove(token_identifier);
            if old_owner_token_identifier.is_empty() {
                self.owners.remove(&old_owner);
            }
        }
        if let Some(new_owner) = new_owner {
            self.owners
                .entry(new_owner)
                .or_insert_with(HashSet::new)
                .insert(token_identifier.clone());
        }
    }

    pub fn operator_token_identifier(
        &self,
        operator: &AccountIdentifier_shiku,
    ) -> Result<&HashSet<TokenId>, NftError> {
        self.operators
            .get(operator)
            .ok_or(NftError::OperatorNotFound)
    }

    pub fn operator_of(
        &self,
        token_identifier: &TokenId,
    ) -> Result<Option<AccountIdentifier_shiku>, NftError> {
        self.token_metadata(token_identifier)
            .map(|token_metadata| token_metadata.operator)
    }

    pub fn update_operator_cache(
        &mut self,
        token_identifier: &TokenId,
        old_operator: Option<AccountIdentifier_shiku>,
        new_operator: Option<AccountIdentifier_shiku>,
    ) {
        if let Some(old_operator) = old_operator {
            let old_operator_token_identifiers = self
                .operators
                .get_mut(&old_operator)
                .expect("couldn't find operator");
            old_operator_token_identifiers.remove(token_identifier);
            if old_operator_token_identifiers.is_empty() {
                self.operators.remove(&old_operator);
            }
        }
        if let Some(operator) = new_operator {
            self.operators
                .entry(operator)
                .or_insert_with(HashSet::new)
                .insert(token_identifier.clone());
        }
    }

    pub fn approve(
        &mut self,
        approved_by: AccountIdentifier_shiku,
        token_identifier: &TokenId,
        new_operator: Option<AccountIdentifier_shiku>,
    ) {
        let token_metadata = self
            .tokens
            .get_mut(token_identifier)
            .expect("couldn't find token metadata");
        token_metadata.operator = new_operator;
        token_metadata.approved_by = Some(approved_by);
        token_metadata.approved_at = Some(time());
    }

    pub fn transfer(
        &mut self,
        transferred_by: AccountIdentifier_shiku,
        token_identifier: &TokenId,
        new_owner: Option<AccountIdentifier_shiku>,
    ) {
        let token_metadata = self
            .tokens
            .get_mut(token_identifier)
            .expect("could not find token metadata");
        token_metadata.owner = new_owner;
        token_metadata.transferred_by = Some(transferred_by);
        token_metadata.transferred_at = Some(time());
        token_metadata.operator = new_owner;
    }

    pub fn burn(&mut self,
        burned_by: AccountIdentifier_shiku, 
        token_identifier: &TokenId) {
            let token_metadata = self.
                tokens
                .get_mut(token_identifier)
                .expect("could not find token metadata");
            token_metadata.owner = None;
            token_metadata.operator = None;
            token_metadata.is_burned = true;
            token_metadata.burned_by = Some(burned_by);
            token_metadata.burned_at = Some(time());
        }

    pub fn inc_tx(&mut self) -> Nat {
        self.tx_count += 1;
        self.tx_count.clone()
    }
}