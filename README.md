# ERC721 Stylus

Implementation of ERC721 in Rust for Arbitrum's [Stylus](https://docs.arbitrum.io/stylus/stylus-gentle-introduction).
Most of the code is based off of the example [here](https://github.com/OffchainLabs/stylus-workshop-nft/blob/main/src/erc712.rs), however there are several differences:
- Fixed balances bug in the transfer function
- Fixed receiver check for safeMint and safeTransfer
- Optimized mint function to be more gas efficient
- Optimized safeMint function to be more gas efficient
- mint(address) has been changed to mint(address, tokenId). Same goes for safeMint.
- totalSupply was removed from the base implementation
- supportsInterface returns true for IERC721Metadata
- tokenUri() has been renamed to tokenUR() to match the ERC721 spec
- Changed internal burn() function to only require a tokenId
- Refactored code to match the latest Stylus SDK

## Running the code

To get the contract abi, you can run:

```bash
cargo stylus export-abi
```

To deploy the code you can run (this will use the default Stylus RPC):

```bash
cargo stylus deploy --private-key=<PRIVATE_KEY>
```

You can interact with Stylus contracts as you would any other contract using the exported ABI.

More info on Stylus can be found [here](https://docs.arbitrum.io/stylus/stylus-quickstart).
