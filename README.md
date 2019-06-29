# Two Party Hash
[![Build Status](https://travis-ci.org/elichai/two-party-hash.svg?branch=master)](https://travis-ci.org/elichai/two-party-hash)
[![Latest version](https://img.shields.io/crates/v/two-party-hash.svg)](https://crates.io/crates/two-party-hash)
![License](https://img.shields.io/crates/l/two-party-hash.svg)
[![dependency status](https://deps.rs/repo/github/elichai/two-party-hash/status.svg)](https://deps.rs/repo/github/elichai/two-party-hash)

A Rust binary to demonstrate how 2 parties can come up with a hash when each one knows just part of the preimage.


## Installation

### From Sources
With Rust's package manager cargo, you can install it via:

```sh
cargo install --force two-party-hash
```


# Usage

First Party: 
`two-party-hash first`

Second Party: 
`two-party-hash second <first-party-hash>` 

Hash them together with regular concatination
`two-party-hash verify <first-preimage> <second-preimage>` 

# Example
```sh
$ two-party-hash first
Preimage: 0x5904b7c57d0ae7afcb13a0c75e156c4f01d7fae4e530d12d896673885f9b38f3b3d58f3912e9d07d425e9792949895fd9f391231185df39487bd5f22ce017627
Non finalized hashed: 0xfcf89320f4b984683b93ef1e3ebbb33ea4a880b8dd1a685f142af9ab2fef5c46

$ two-party-hash second 0xfcf89320f4b984683b93ef1e3ebbb33ea4a880b8dd1a685f142af9ab2fef5c46
Preimage: 0x23e1c69b26401525173be914bdd718b9bfd74a655e9330e0c3cfe8f6bf1ed80e
Final hashed: 0x54c4289d02d9e4ae99e04bad638911c0236226c0ee20ba820fa856e6f5802911

$ two-party-hash verify 0x5904b7c57d0ae7afcb13a0c75e156c4f01d7fae4e530d12d896673885f9b38f3b3d58f3912e9d07d425e9792949895fd9f391231185df39487bd5f22ce017627 0x23e1c69b26401525173be914bdd718b9bfd74a655e9330e0c3cfe8f6bf1ed80e
Regular hash of both preimages: 0x54c4289d02d9e4ae99e04bad638911c0236226c0ee20ba820fa856e6f5802911

```
