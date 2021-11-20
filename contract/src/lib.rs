use near_sdk::{near_bindgen, setup_alloc};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

const PUZZLE_NUMBER: u8 = 1;

setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    records: String,
    // records: LookupMap<String, String>,
}

#[near_bindgen]
impl Contract {
    fn default() -> Self {
        Self {
            records: "hello user".to_string()
        }
    }
    /*#[init]
    pub fn init() -> Self {
        Self {
            records: vec![
                "what a beautiful name".to_string(),
                "good weather today, isn't it?".to_string(),
                "guess my name 😀".to_string(),
                "well done!!".to_string(),
            ],
        }
    }*/

    pub fn set_greeting(&mut self, message: String) {
        // let account_id = env::signer_account_id();

        // Use env::log to record logs permanently to the blockchain!
        // log!("Saving greeting '{}'", message);

        self.records = message;
    }

    // `match` is similar to `switch` in other languages; here we use it to default to "Hello" if
    // self.records.get(&account_id) is not yet defined.
    // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
    pub fn get_greeting(&self, account_id: String) -> String {
        // match self.records.get(&account_id) {
        //     Some(greeting) => greeting,
        //     None => "Hello".to_string(),
        // }
        self.records.clone()
    }
    /*pub fn get_greeting(&self) -> String {
        "Hello".to_string()
    }*/

    // pub fn add_hello(&mut self, hello: String) {
    //     self.records.push(hello);
    // }

    /*pub fn set_greeting(&mut self, username: String) -> String {
        env::log_str("fn hello start!");
        // let rnd = block_timestamp().to_string().chars().last().unwrap_or('0').to_string();
        // let mut rnd: i32 = rnd.parse::<i32>().unwrap_or(0);
        // if rnd >= self.records.len() as i32 {
        //     rnd = rnd / 2;
        // }
        // let greetings = format!("Hello {}, {}", username, self.records[rnd as usize]);
        let greetings = format!("Hello {}, {}", username, "how are you doing?");
        env::log_str(&greetings);
        greetings
    }*/
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{testing_env, VMContext};
    use near_sdk::test_utils::VMContextBuilder;

    use super::*;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn get_message() {
        let context = get_context(false);
        testing_env!(context);
        let contract = Contract::init();
        assert_eq!(
            1,
            contract.get_number()
        );
    }

    #[test]
    fn hello_username() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Contract::init();
        let answer = contract.hello("dolly".to_string());
        println!("answer: {}", answer);
        contract.add_hello("welcome new user".to_string());
        let answer = contract.hello("molly".to_string());
        println!("answer: {}", answer);
        let answer = contract.hello("molly".to_string());
        println!("answer: {}", answer);
        let answer = contract.hello("polly".to_string());
        println!("answer: {}", answer);
        let answer = contract.hello("guest".to_string());
        println!("answer: {}", answer);
    }
}