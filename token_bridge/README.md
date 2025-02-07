# Token Bridge

This repository contains a Scrypto (Rust) implementation of a token bridge, allowing tokens to be migrated from an older version to a new version which is burnable. The bridge can be instantiated with any fungible token's `ResourceAddress`. The newly created tokens can be minted and burned as needed. Users can bridge their old tokens to the new token using this implementation. This is a one-way bridge allowing users only to send an old resource for a new one.

## Overview

It contains a `TokenBridge` struct, which is the core data structure and has three key components:

1. `auth_minting_burning`: A vault that holds the authentication for minting and burning the new tokens.
2. `old_token_resource_vault`: A vault containing the old tokens.
3. `bridge_token_resource_address`: The address of the newly created token resource.

The module includes several methods:

- `instantiate_bridge()`: Creates a new token bridge instance and sets up the necessary resources for minting and burning the new tokens.
- `bridge_token_resource_address()`, `old_token_resource_address()`, and `old_tokens_bridged()`: Getter methods that provide information about the new token, old token, and the amount of old tokens bridged respectively.
- `bridge()`: Allows users to convert their old tokens into new ones. It ensures the tokens being bridged match the old tokens and then mints an equivalent amount of new tokens.

## Example

To create a new instance of the token bridge and bridge some tokens, you can use the following code:

```rust
use scrypto::prelude::*;

// Instantiate the token bridge
let old_token_resource_address: ResourceAddress = /* ... */;
let bridge_token_name = "New Token";
let bridge_token_symbol = "NTK";
let bridge_token_description = "A new token to replace the old one";
let token_bridge_component_address = token_bridge::TokenBridge::instantiate_bridge(
    old_token_resource_address,
    bridge_token_name.to_string(),
    bridge_token_symbol.to_string(),
    bridge_token_description.to_string(),
);

// Get a reference to the instantiated token bridge
let mut token_bridge: token_bridge::TokenBridge = token_bridge_component_address.get_mut();

// Bridge some old tokens to the new token
let old_tokens: Bucket = /* ... */;
let new_tokens = token_bridge.bridge(old_tokens);
```

This will instantiate a new token bridge, create a new token, and allow bridging old tokens to the new token by calling the `bridge` function.