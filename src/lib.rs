// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{ alloy_primitives::{ U8, Address }, prelude::* };
use stylus_sdk::{ msg };
// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Voters {
        mapping(address => bool) voted;
        uint8 votes;
        mapping(uint8 => Cax) cax_y;
        uint8 state;
    }

    pub struct Cax{
        address cax;
        uint8 total_vote;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Voters {
    pub fn signer(&mut self) -> Result<Vec<u8>, Vec<u8>> {
        if self.state.get() > U8::from(2) {
            return Err(vec![0]);
        }
        let state = self.state.get();
        let mut data = self.cax_y.setter(state);
        data.cax.set(msg::sender());

        self.state.set(state + U8::from(1));
        return Ok(vec![1]);
    }

    pub fn vote(&mut self, canx: u8) {
        let user_vote = U8::from(canx);
        if self.votes.get() > U8::from(9) {
            return;
        }
        if self.voted.get(msg::sender()) {
            return;
        }
        if user_vote > self.state.get() {
            return;
        }

        let mut candidate = self.cax_y.setter(user_vote);
        let mut cax_state_vote = candidate.total_vote.get();
        candidate.total_vote.set(cax_state_vote + U8::from(1));

        let votes = self.votes.get();
        self.votes.set(votes + U8::from(1));

        let mut voter_state = self.voted.setter(msg::sender());
        voter_state.set(true);
    }
}
