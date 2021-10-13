use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Poster {
    val: i8,
    records: HashMap<i8, String>,
}

#[near_bindgen]
impl Poster {
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    #[private]
    pub fn increment(&mut self) {
        self.val += 1;
        let log_message = format!("Increased number to {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    #[payable]
    pub fn set_message(&mut self, message: String) {
        // assert!(env::id == self.val, "Owner's method");
        let account_id = env::signer_account_id();
        log!(
            "{} set_message with message {} in {}.",
            account_id,
            message,
            self.val,
        );
        self.records.insert(self.val, message);
        self.increment();
    }

    pub fn get_message(&self, val: i8) -> Option<String> {
        self.records.get(&val).cloned()
    }
}

fn after_counter_change() {
    env::log("Make sure you don't overflow, my friend.".as_bytes());
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Poster::default();
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        // confirm that we received 1 when calling get_num
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Poster::default();
        contract.increment();
        contract.set_message("Hello World".to_string());
        // confirm that we received -1 when calling get_num
        assert_eq!(Some("Hello World"), contract.get_message(1).as_deref());
    }
}
