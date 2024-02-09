# Fee Controller

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Overview](#overview)
  - [Protocol Fees](#protocol-fees)
  - [Liquidity Fees](#liquidity-fees)
  - [Fee Storage](#fee-storage)
- [Instantiation](#instantiation)
- [Methods](#methods)
  - [Set Methods](#set-methods)
  - [Get Methods](#get-methods)
- [Events](#events)
- [Permissions](#permissions)
  - [Owner Permissions](#owner-permissions)
  - [Fee manager Permissions](#fee-manager-permissions)

## Introduction

This package implements the blueprint for a fee controller which is used to set percentage protocol and liquidity fees. The fee controller is designed to be used by another component such as a DEX for determining the fees to charge on a trade. If the specified protocol or liquidity position is not listed, the fee controller will return the default fee. Default fees and specific fees are set by the `fee_manager`.

## Getting Started

### Docs

Rust docs are provided. To generate them, from the fee controller directory run:

```bash
cargo doc --no-deps --open
```

### Testing

A full set of tests are provided that use the scrypto test runner. At times there is a quirk with the test runner where a race condition causes the code to not compile before the test runner tries to run it. This can randomly cause tests to fail. The test runner is also very slow. For these reasons, instead of using the default command `scrypto test` it is preferable to use Nextest which can be downloaded from here: <https://nexte.st>. To run the full set of tests, from the fee controller directory run:

```bash
cargo nextest run -r --retries 3
```

## Overview

### Protocol Fees

Protocol fees are saved in a map of a protocol's `PackageAddress` to fee. If the protocol's `PackageAddress` is not found in the map, the default protocol fee is returned. Protocol fees are set by the `fee_manager` and must be between 0% and 1%.

### Liquidity Fees

Liquidity fees are saved in a map of a liquidity position's `ResourcesKey` to fee. If the liquidity position's `ResourcesKey` is not found in the map, the liquidity protocol fee is returned. The `ResourceKey` is the XOR of all unique `ResourceAddress` for the liquidity position. Liquidity fees are set by the `fee_manager` and must be between 0% and 5%.

### Fee Storage

Key values stores are used for the maps of protocol and liquidity fees for scalability. Fees are stored as a `u16` which represents basis point hundredths. For example, a fee of 0.0001% would be stored as 1.

## Instantiation

The fee controller can be instantiated as either a owned component by using `new_local()` or as a global component by using `new(admin_badge_address: ResourceAddress)`. The `admin_badge_address` will be set as the `owner` and `fee_manager` of the fee controller.

## Methods

### Set Methods

Setting fees is only allowed by the `fee_manager`.

- `set_default_protocol_fee`
- `set_default_liquidity_fee`
- `set_protocol_fee`
- `set_liquidity_fee`

### Get Methods

- `get_default_protocol_fee`
- `get_default_liquidity_fee`
- `get_protocol_fee`
- `get_liquidity_fee`
- `get_fees`

## Events

- `SetProtocolFeeDefaultEvent` - The default protocol fee has been set.
- `SetLiquidityFeeDefaultEvent` - The default liquidity fee has been set.
- `SetProtocolFeeEvent` - A protocol fee has been set.
- `SetLiquidityFeeEvent` - A liquidity fee has been set.

## Permissions

### Owner Permissions

- Update the `owner` role access rule.
- Update the `fee_manager` role access rule.
- Update metadata for the fee controller.

### Fee Manager Permissions

- Set the default protocol fee.
- Set the default liquidity fee.
- Set a protocol fee.
- Set a liquidity fee.
