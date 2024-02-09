# QuantaSwap

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Overview](#overview)
  - [External Components](#external-components)
  - [Liquidity Receipt](#liquidity-receipt)
  - [Ticks](#ticks)
  - [Bin](#bin)
  - [Tick Index](#tick-index)
- [Instantiation](#instantiation)
- [Methods](#methods)
  - [Mint Liquidity Receipt](#mint-liquidity-receipt)
  - [Burn Liquidity Receipt](#burn-liquidity-receipt)
  - [Add Liquidity](#add-liquidity)
  - [Add Liquidity To Receipt](#add-liquidity-to-receipt)
  - [Remove Liquidity](#remove-liquidity)
  - [Remove Specific Liquidity](#remove-specific-liquidity)
  - [Swap](#swap)
  - [Get Methods](#get-methods)
- [Events](#events)
- [Permissions](#permissions)
  - [Owner Permissions](#owner-permissions)
  - [User Permissions](#user-permissions)

## Introduction

This package implements the blueprint for a concentrated liquidity automated market maker. Liquidity is added into discrete pre-defined bins. Swaps then occur within the active bin. Liquidity provided to the pool is represented as liquidity receipt which can hold up to 200 different positions. This allows for a more efficient use of liquidity, automatic compounding of fees, and easily customized liquidity provisioning strategies.

## Getting Started

### Docs

Rust docs are provided. To generate them, from the quantaswap directory run:

```bash
cargo doc --no-deps --open
```

### Testing

A full set of tests are provided that use the scrypto test runner. At times there is a quirk with the test runner where a race condition causes the code to not compile before the test runner tries to run it. This can randomly cause tests to fail. The test runner is also very slow. For these reasons, instead of using the default command `scrypto test` it is preferable to use Nextest which can be downloaded from here: <https://nexte.st>. To run the full set of tests, from the quantaswap directory run:

```bash
cargo nextest run -r --retries 3
```

## Overview

Quantaswap uses several sub-structures to operate.

### External Components

A quantaswap pool component depends on two external components: `FeeController` and `FeeVaults`. These components are used to manage fees for swaps. The `FeeController` component provides a method to get the protocol fee percentage, and the `FeeVaults` component is used collect the fees. The addresses of these components is hard coded into the quantaswap blueprint.

### Liquidity Receipt

A `LiquidityReceipt` stores up to 200 different liquidity positions. This is a map from tick to liquidity share of the bin.

### Ticks

A `Tick` is a discrete point in price logarithmic space. The distance between each `Tick` is 10 basis points. The `Tick` type uses `u32` as the underlying representation. External methods use `u32` instead of `Tick` to allow for easier integration. The range of allowed prices is `0.000000000001892254` to `528470197086.935858253558842035`.

### Bin

A `Bin` hold liquidity for a discrete part of price logarithmic space. A `Bin`'s position is set by the `Tick` of it's lower bound. The number of `Tick`s a `Bin` covers, and therefore it's upper bound is set by the `bin_span` parameter of the pool. The pool always has one active `Bin` with will contain some ratio of tokens x and y. `Bin`s at a higher `Tick` than the active `Bin` will contain only tokens x, and `Bin`s at a lower `Tick` will contain only tokens y. Swaps occur within the active `Bin`. When the bound of the active `Bin` is reached, the active `Bin` make inactive and the next `Bin` in the direction of the swap is made active.

### Tick Index

The `TickIndex` stores the `Tick`s of all active bins. It uses a key-value store (KVS) as the underlying structure. On top of this KVS is built a trie for the limited universe of possible ticks. The `TickIndex` is used to efficiently find the best available `Bin` when the bound of the active `Bin` is reached.

## Instantiation

A new quantaswap pool is created using the `new` function. This function takes the following arguments:

- `owner_rule: AccessRule` - Access rule for the `owner` role.
- `user_rule: AccessRule` - Access rule for the `user` role.
- `token_x_address: ResourceAddress` - Address of the token x for the pool.
- `token_y_address: ResourceAddress` - Address of the token y for the pool.
- `bin_span: u32` - The span of ticks a bin covers.
- `reservation` - Optional address reservation for the pool.

## Methods

### Mint Liquidity Receipt

Mint a liquidity receipt. This is necessary in order to add liquidity to the pool. There is a limit of 200 positions per liquidity receipt but no limit on the number of liquidity receipts a user can mint.

### Burn Liquidity Receipt

Burn a liquidity receipt. The liquidity receipt must have no claims in order to burn it.

### Add Liquidity

Add liquidity at the specified positions to the pool and store in a liquidity receipt.

### Add Liquidity To Receipt

Add liquidity at the specified positions to an existing liquidity receipt.

### Remove Liquidity

Remove all liquidity from a liquidity receipt and burn the receipt.

### Remove Specific Liquidity

Remove liquidity at the specified claims from the pool using a liquidity receipt.

### Swap

Swap either token x or y for the opposite token. A percentage protocol fee and liquidity fee is subtracted from the input tokens. These fees are controlled by the fee controller. The protocol fee is sent to the fee vaults and the liquidity fee is added to the active `Bin`.

### Get Methods

The following methods are available to get information about the pool:

- `get_fee_controller_address`
- `get_fee_vaults_address`
- `get_token_x_address`
- `get_token_y_address`
- `get_liquidity_receipt_address`
- `get_bin_span`
- `get_liquidity_claims`
- `get_amount_x`
- `get_amount_y`
- `get_active_tick`
- `get_price`
- `get_active_bin_price_range`
- `get_active_amounts`
- `get_bins_above`
- `get_bins_below`
- `get_redemption_value`
- `get_redemption_bin_values`

## Events

The pool emits events for the following actions:

- `NewPoolEvent` - A new pool has been created.
- `MintLiquidityReceiptEvent` - A new liquidity receipt has been minted.
- `BurnLiquidityReceiptEvent` - A liquidity receipt has been burned.
- `AddLiquidityEvent` - Liquidity has been added to the pool.
- `RemoveLiquidityEvent` - Liquidity has been removed from the pool.
- `SwapEvent` - A swap has occurred.
- `ProtocolFeeEvent` - A protocol fee has been collected.
- `LiquidityFeeEvent` - A liquidity fee has been collected.

## Permissions

### Owner Permissions

The `owner` role can take following actions:

- Update the `owner` role access rule.
- Update the `user` role access rule.
- Update metadata for the pool.
- Update metadata for the liquidity receipts.

### User Permissions

The `user` role can take the following actions:

- Mint a liquidity receipt.
- Add liquidity.
- Swap tokens.

Note, removing liquidity can not be restricted.
