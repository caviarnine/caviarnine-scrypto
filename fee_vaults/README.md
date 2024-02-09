# Fee Vaults

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Overview](#overview)
  - [Treasury vaults](#treasury-vaults)
  - [Swap vaults](#swap-vaults)
  - [Reserve](#reserve)
- [Instantiation](#instantiation)
- [Methods](#methods)
  - [Swap](#swap)
  - [Set Methods](#set-methods)
  - [Get Methods](#get-methods)
  - [Withdraw Methods](#withdraw-methods)
  - [Deposit Methods](#deposit-methods)
- [Events](#events)
- [Permissions](#permissions)
  - [Owner Permissions](#owner-permissions)
  - [Treasury Manager Permissions](#treasury-manager-permissions)
  - [Reserve Manager Permissions](#reserve-manager-permissions)
  - [User Permissions](#user-permissions)

## Introduction

This package implements the blueprint for fee vaults which is used to collect protocol fees. The fee vaults is designed to be used by another component such as a DEX for collect protocol fees from a trade. Collected fees are split between the treasury and swap vaults that are auctioned off for the ecosystem token. Ecosystem tokens are then split between the reserve and being burned.

## Getting Started

### Docs

Rust docs are provided. To generate them, from the fee vaults directory run:

```bash
cargo doc --no-deps --open
```

### Testing

A full set of tests are provided that use the scrypto test runner. At times there is a quirk with the test runner where a race condition causes the code to not compile before the test runner tries to run it. This can randomly cause tests to fail. The test runner is also very slow. For these reasons, instead of using the default command `scrypto test` it is preferable to use Nextest which can be downloaded from here: <https://nexte.st>. To run the full set of tests, from the fee vaults directory run:

```bash
cargo nextest run -r --retries 3
```

## Overview

### Treasury vaults

A percentage of the deposited tokens according to `treasury_percentage` are sent to the treasury vaults. The treasury vaults can be withdrawn from by the `treasury_manager`.

### Swap vaults

Fees that are not sent to the treasury are sent to the swap vaults. The swap vaults are auctioned off for the swap token. The price of each swap vault is determined by the base `swap_amount` and the number of epochs since the last swap. The decrease linearly at a rate such that the swap price will be zero at `max_epochs`. It is a assumed the swap vault will be swapped near fair value due to arbitrage. A percentage of swap tokens from a swap are burned according to `burn_percentage`.

### Reserve

Swap tokens that are not burned are sent to the reserve. The reserve can be withdrawn from by the `reserve_manager`.

## Instantiation

A new global fee vaults is created using the `new` function. This function takes the following arguments:

- `admin_badge_address` - The address of the admin badge that will be used to set the `owner`, `treasury_manager`, and `reserve_manager` roles.
- `swap_token_address` - The address of the swap token that swap vaults will be auctioned off for.
- `swap_amount` - The base cost of a swap vault.

A new owned fee vaults is created using the `new_local` function. This function takes the following arguments:

- `swap_token_address` - The address of the swap token that swap vaults will be auctioned off for.
- `swap_amount` - The base cost of a swap vault.

## Methods

### Swap

Swap the swap token for the contents of a swap vault as the current swap price of that swap vault. The swap price slowly decreases over time. The swap price is reset when a swap vault is swapped.

### Set Methods

The following can only be called by the `owner`.

- `set_treasury_percentage`
- `set_burn_percentage`
- `set_swap_amount`
- `set_max_epochs`

### Get Methods

- `get_treasury_percentage`
- `get_burn_percentage`
- `get_swap_amount`
- `get_max_epochs`
- `get_last_swapped_epoch`
- `get_swap_vault_amount`
- `get_reserve_amount`
- `get_treasury_vault_amount`
- `get_swap_price`

### Withdraw Methods

- `treasury_withdraw` - Can only be called by the `treasury_manager`.
- `reserve_withdraw` - Can only be called by the `reserve_manager`.

### Deposit Methods

- `treasury_deposit`
- `swap_vault_deposit`
- `reserve_deposit`
- `deposit`
- `deposit_batch`

## Events

- `SetTreasuryPercentageEvent` - Emitted when the `treasury_percentage` is set.
- `SetBurnPercentageEvent` - Emitted when the `burn_percentage` is set.
- `SetSwapAmountEvent` - Emitted when the `swap_amount` is set.
- `SetMaxEpochsEvent` - Emitted when the `max_epochs` is set.
- `TreasuryWithdrawEvent` - Emitted when tokens are withdrawn from a treasury vault.
- `ReserveWithdrawEvent` - Emitted when tokens are withdrawn from the reserve.
- `TreasuryDepositEvent` - Emitted when tokens are deposited into a treasury vault.
- `SwapVaultDepositEvent` - Emitted when tokens are deposited into a swap vault.
- `ReserveDepositEvent` - Emitted when tokens are deposited into the reserve.
- `BurnEvent` - Emitted when swap tokens are burned.
- `SwapEvent` - Emitted when swap tokens are swapped for the contents of a swap vault.

## Permissions

### Owner Permissions

- Update the `owner` role access rule.
- Update the `treasury_manager` role access rule.
- Update the `reserve_manager` role access rule.
- Update the `user` role access rule.
- Update metadata for the fee vaults.
- Update various fee vaults parameters.

### Treasury Manager Permissions

- Withdraw tokens from the treasury vaults.

### Reserve Manager Permissions

- Withdraw tokens from the reserve vaults.

### User Permissions

- Swap the swap token for the contents of a swap vault.
