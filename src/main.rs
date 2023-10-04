#![cfg_attr(not(feature = "export-abi"), no_main,no_std)]
extern crate alloc;

mod erc721;

use crate::erc721::{ERC721, ERC721Params};
use alloc::{string::String, vec::Vec, format};
use stylus_sdk::{alloy_primitives::U256, call, msg, prelude::*};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct SampleParams;

/// Immutable definitions
impl ERC721Params for SampleParams {
    const NAME: &'static str = "Sample ERC721";
    const SYMBOL: &'static str = "SAMPLE";
    
    fn token_uri(token_id: U256) -> String {
        format!("ipfs://QmZcH4YvBVVRJtdn4RdbaqgspFU8gH6P9vomDpBVpAL3u4/{}", token_id)
    }
}

// The contract
sol_storage! {
    #[entrypoint] // Makes Sample the entrypoint
    struct Sample {
        #[borrow] // Allows erc721 to access Sample's storage and make calls
        ERC721<SampleParams> erc721;
        uint256 total_supply;
    }
}

#[external]
#[inherit(ERC721<SampleParams>)]
impl Sample {
    pub fn mint(&mut self, token_id: U256) -> Result<(), Vec<u8>> {
        self.erc721.mint(msg::sender(), token_id)?;
        Ok(())
    }

    pub fn mint_loop(&mut self, qty: U256) -> Result<(), Vec<u8>> {
        let supply = self.total_supply.get();
        for i in 0..qty.try_into().unwrap() {
            self.erc721.mint(msg::sender(), supply + U256::from(i))?;
        }
        self.total_supply.set(supply + qty);
        Ok(())
    }

    pub fn burn(&mut self, token_id: U256) -> Result<(), Vec<u8>> {
        self.erc721.burn(token_id)?;
        Ok(())
    }
}
