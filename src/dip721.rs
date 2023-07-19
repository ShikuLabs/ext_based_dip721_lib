use crate::ledger;
use crate::{
    NftError,
    TokenId, 
    TokenMetadata,
};
use ic_cdk::api::time;
use ic_cdk::export::candid::Nat;
use ic_cdk::export::Principal;
use std::cell::RefCell;
use std::ops::Not;
use std::sync::atomic::AtomicU32;
use std::collections::{HashSet, HashMap};

use super::types::{AccountIdentifier_shiku};
use super::pid2aid;
use super::types::{change_minted_state, TokenIndex, AccountIdentifier__1};
thread_local! {
    static TID: RefCell<AtomicU32> = RefCell::new(AtomicU32::new(1));
}

pub fn new_token_id() -> u32 {
    TID.with(|tid| {
        let token = tid.borrow_mut();
        let new_id = token.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        new_id
    })
}

pub fn tid_info() -> u32 {
    TID.with(|tid| {
        tid.borrow_mut().fetch_add(0, std::sync::atomic::Ordering::SeqCst)
    })
}

pub fn restore_tid_info(token_index: u32) {
    TID.with(|tid|{
        tid.borrow_mut().fetch_add(token_index, std::sync::atomic::Ordering::SeqCst);
    })
}

pub fn restore_minted_info(minted_id: Vec<Nat>) {
    change_minted_state(|minted|
        {
            let mut mint = minted.clone();
            for id in minted_id.iter() {
                mint.push(id.to_owned())
            }
        }
    );
}

// pub fn dip721_init(args: Option<InitArgs>) {
//     ledger::with_mut(|ledger| ledger.init_metadata(ic_cdk::api::caller(), args));
// }

pub fn dip721_total_supply() -> Nat {
    ledger::with(|ledger| Nat::from(ledger.tokens_count()))
}

pub fn dip721_balance_of(owner: AccountIdentifier_shiku) -> Result<Nat, NftError> {
    ledger::with(|ledger| {
        ledger
            .owner_token_identifiers(&owner)
            .map(|token_identifier| Nat::from(token_identifier.len()))
    })
}

pub fn dip721_transfer_from(
    owner: AccountIdentifier_shiku,
    to: AccountIdentifier_shiku,
    token_identifier: TokenId,
) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = ic_cdk::api::caller();
        if owner.eq(&to) {
            // insert_sync(IndefiniteEvent {
            //     caller: ic_cdk::api::caller(),
            //     operation: "verify owner".into(),
            //     details: vec![
            //         ("owner".into(), serde_json::to_string(&owner)),
            //         ("to".into(), DetailValue::from(to.clone()))
            //         ],
            // });
            return Err(NftError::UnauthorizedOwner);
        }
        let old_owner = match ledger.owner_of(&token_identifier).ok() {
            Some(owner) => owner,
            None => return Err(NftError::OwnerNotFound),
        };
        let old_operator = match ledger.operator_of(&token_identifier).ok() {
            Some(operator) => operator,
            None => return Err(NftError::OperatorNotFound),
        };
        
        old_owner
            .eq(&Some(owner))
            .then_some(())
            .ok_or(NftError::UnauthorizedOwner)?;
        old_operator
            .eq(&Some(owner))
            .then_some(())
            .ok_or(NftError::UnauthorizedOperator)?;
        ledger.update_owner_cache(&token_identifier, old_owner, Some(to));
        ledger.update_operator_cache(&token_identifier, old_operator, Some(to));
        ledger.transfer(owner, &token_identifier, Some(to));

        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "transferFrom".into(),
        //     details: vec![
        //         ("owner".into(), DetailValue::from(owner)),
        //         ("to".into(), DetailValue::from(to)),
        //         (
        //             "token_identifier".into(),
        //             DetailValue::from(token_identifier.to_string()),
        //         ),
        //     ],
        // });

        Ok(Nat::from(ledger.inc_tx() - 1))
    })
}

pub fn dip721_custodians() -> HashSet<Principal> {
    ledger::with(|ledger| ledger.metadata().custodians.clone())
}

pub fn dip721_mint(
    to: AccountIdentifier_shiku,
    token_identifier: &TokenId,
    // properties: CoCreateMetadata,
) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        // let caller = ic_cdk::api::caller();
        if !ledger.is_token_existed(token_identifier).not() {
            // insert_sync(IndefiniteEvent {
            //     caller: ic_cdk::api::caller(),
            //     operation: "verify token exist".into(),
            //     details: vec![(
            //         "existed token identifier".into(),
            //         DetailValue::from(token_identifier.clone()),
            //     )],
            // });
            return Err(NftError::ExistedNFT);
        }
        // let name = NAME.to_string() + token_identifier.to_string().as_str();
        // let description = DESCRIPTION.to_string();

        // let cocreate_prop = CoCreateMetadata::new(Some(name), None, Some(description));

        ledger.add_token_metadata(
            token_identifier,
            TokenMetadata {
                token_identifier: token_identifier.to_owned(),
                owner: Some(to),
                operator: Some(to),
                properties: None,
                is_burned: false,
                minted_at: time(),
                minted_by: to,
                transferred_at: None,
                transferred_by: None,
                approved_at: None,
                approved_by: None,
                burned_at: None,
                burned_by: None,
                status: 1,
            },
        );
        ledger.update_owner_cache(&token_identifier, None, Some(to));
        ledger.update_operator_cache(&token_identifier, None, Some(to));
        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "mint".into(),
        //     details: vec![
        //         ("to".into(), DetailValue::from(to)),
        //         (
        //             "token_identifier".into(),
        //             DetailValue::from(token_identifier.to_string()),
        //         ),
        //     ],
        // });

        Ok(Nat::from(ledger.inc_tx() - 1))
    })
}

pub fn dip721_burn(token_identifier: TokenId) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = pid2aid(&ic_cdk::api::caller());
        let old_owner = match ledger.owner_of(&token_identifier).ok() {
            Some(owner) => owner,
            None => return Err(NftError::OwnerNotFound),
        };
        
        if old_owner.ne(&Some(caller)) {
            // insert_sync(IndefiniteEvent {
            //     caller: ic_cdk::api::caller(),
            //     operation: "verify old owner".into(),
            //     details: vec![(
            //         "unauthozied owner".into(),
            //         DetailValue::from(caller.clone()),
            //     )],
            // });
            return Err(NftError::UnauthorizedOwner);
        }
        let old_operator = match ledger.operator_of(&token_identifier).ok() {
            Some(operator) => operator,
            None => return Err(NftError::OperatorNotFound),
        };
        ledger.update_owner_cache(&token_identifier, old_owner, None);
        ledger.update_operator_cache(&token_identifier, old_operator, None);
        ledger.burn(caller, &token_identifier);

        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "burn".into(),
        //     details: vec![(
        //         "token_identifier".into(),
        //         DetailValue::from(token_identifier.to_string()),
        //     )],
        // });

        Ok(Nat::from(ledger.inc_tx() - 1))
    })
}

pub fn dip721_approve(
    operator: AccountIdentifier_shiku,
    token_identifier: TokenId,
) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = pid2aid(&ic_cdk::api::caller());
        if operator.eq(&caller) {
            // insert_sync(IndefiniteEvent {
            //     caller: ic_cdk::api::caller(),
            //     operation: "verify caller".into(),
            //     details: vec![("operator".into(), DetailValue::from(operator.to_string()))],
            // });
            return Err(NftError::SelfApprove);
        };
        let owner = match ledger.owner_of(&token_identifier).ok() {
            Some(owner) => owner,
            None => return Err(NftError::OwnerNotFound),
        };
        if owner.ne(&Some(caller)) {
            // insert_sync(IndefiniteEvent {
            //     caller: ic_cdk::api::caller(),
            //     operation: "verify owner".into(),
            //     details: vec![(
            //         "owner".into(),
            //         DetailValue::from(owner.unwrap().to_string()),
            //     )],
            // });
            return Err(NftError::UnauthorizedOwner);
        }
        ledger.update_operator_cache(
            &token_identifier,
            ledger.operator_of(&token_identifier)?,
            Some(operator),
        );
        ledger.approve(caller, &token_identifier, Some(operator));

        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "approve".into(),
        //     details: vec![
        //         ("operator".into(), DetailValue::from(operator)),
        //         (
        //             "token_identifier".into(),
        //             DetailValue::from(token_identifier.to_string()),
        //         ),
        //     ],
        // });

        Ok(Nat::from(ledger.inc_tx() - 1))
    })
}

pub fn dip721_get_registry() -> HashMap<TokenIndex, AccountIdentifier__1> {
    ledger::with(|ledger| ledger.get_registry())
}


pub fn dip721_add_aid_idx(k: &TokenIndex, v: &AccountIdentifier__1) {
    ledger::with_mut(|ledget| {
        ledget.idx2aid.insert(*k, v.to_string());
    });
}

pub fn dip721_token_metadata(token_identifier: TokenId) -> Result<TokenMetadata, NftError> {
    ledger::with(|ledger| ledger.token_metadata(&token_identifier).cloned())
}

pub fn dip721_owner_token_identifiers(
    owner: AccountIdentifier_shiku,
) -> Result<HashSet<TokenId>, NftError> {
    ledger::with(|ledger| ledger.owner_token_identifiers(&owner).cloned())
}

pub fn dip721_operator_token_identifiers(
    token: TokenId,
) -> Result<Option<AccountIdentifier_shiku>, NftError> {
    ledger::with(|ledger| ledger.operator_of(&token))
}

pub fn dip721_owner_of(
    token: TokenId,
) -> Result<Option<AccountIdentifier_shiku>, NftError> {
    ledger::with(|ledger| ledger.owner_of(&token))
}
