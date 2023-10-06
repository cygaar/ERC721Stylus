#![cfg_attr(not(feature = "export-abi"), no_std)]
extern crate alloc;

mod erc721;

use crate::erc721::{ERC721, ERC721Params};
use alloc::{string::String, vec::Vec, format};
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct SampleParams;

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
    #[entrypoint] // Makes SampleNFT the entrypoint
    pub struct SampleNFT {
        #[borrow] // Allows erc721 to access SampleNFT's storage and make calls
        ERC721<SampleParams> erc721;
        uint256 total_supply;
    }
}

// Rust implementation of this SampleNFT Solidity contract:

// pragma solidity ^0.8.21;
// import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

// contract SampleNFT is ERC721 {
//     uint256 public totalSupply;

//     constructor() ERC721("Sample NFT", "SAMPLE") {}

//     function mintLoop(uint256 qty) external {
//         uint256 supply = totalSupply;
//         for (uint256 i; i < qty; ) {
//             unchecked {
//                 _mint(msg.sender, supply++);
//                 ++i;
//             }
//         }
//         totalSupply = supply;
//     }

//     function burn(uint256 tokenId) external {
//         _burn(tokenId);
//         unchecked {
//             --totalSupply;
//         }
//     }
// }

#[external]
#[inherit(ERC721<SampleParams>)]
impl SampleNFT {
    pub fn total_supply(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_supply.get())
    }

    pub fn mint_loop(&mut self, qty: U256) -> Result<(), Vec<u8>> {
        let supply = self.total_supply.get();
        let supply: u32 = supply.try_into().unwrap();
        let qty: u32 = qty.try_into().unwrap();
    
        for i in 0..qty.try_into().unwrap() {
            self.erc721.mint(msg::sender(), U256::from(supply + i))?;
        }
        self.total_supply.set(U256::from(supply + qty));
        Ok(())
    }

    pub fn burn(&mut self, token_id: U256) -> Result<(), Vec<u8>> {
        self.erc721.burn(token_id)?;
        let supply = self.total_supply.get();
        self.total_supply.set(supply - U256::from(1));
        Ok(())
    }
}
