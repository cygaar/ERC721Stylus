## ERC721 Stylus

Implementation of ERC721 in Rust for Arbitrum's [Stylus](https://docs.arbitrum.io/stylus/stylus-gentle-introduction).
Most of the code is based off of the example [here](https://github.com/OffchainLabs/stylus-workshop-nft/blob/main/src/erc712.rs), however there are several differences:
- Fixed balances bug in the transfer function
- mint(address) has been changed to mint(address, tokenId). Same goes for safeMint.
- totalSupply was removed from the base implementation
- supportsInterface returns true for IERC721Metadata
- tokenUri() has been renamed to tokenUR() to match the ERC721 spec
- Changed internal burn() function to only require a tokenId