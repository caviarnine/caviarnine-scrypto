# QuantaSwap Factory

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Overview](#overview)
  - [External Components](#external-components)
  - [QuantaSwap Pool](#quantaswap-pool)
  - [Pool Maps](#pool-maps)
- [Instantiation](#instantiation)
- [Methods](#methods)
  - [New Pool](#new-pool)
  - [Get Methods](#get-methods)
  - [Set Methods](#set-methods)
- [Events](#events)
- [Permissions](#permissions)
  - [Owner Permissions](#owner-permissions)
  - [User Permissions](#user-permissions)

## Introduction

This package implements the blueprint for an quantaswap factory that creates quantaswap pools for two validated fungible tokens. The quantaswap factory is responsible for creating pool, storing their addresses, and for setting owner and user permissions for the pools it creates.

## Getting Started

### Docs

Rust docs are provided. To generate them, from the quantaswap factory directory run:

```bash
cargo doc --no-deps --open
```

### Testing

A full set of tests are provided that use the scrypto test runner. At times there is a quirk with the test runner where a race condition causes the code to not compile before the test runner tries to run it. This can randomly cause tests to fail. The test runner is also very slow. For these reasons, instead of using the default command `scrypto test` it is preferable to use Nextest which can be downloaded from here: <https://nexte.st>. To run the full set of tests, from the quantaswap factory directory run:

```bash
cargo nextest run -r --retries 3
```

## Overview

### External Components

The quantaswap factory depends on the `TokenValidator` component to validate the tokens that are used to create pool. The address of this component can be set by the owner of the quantaswap factory and uses a generic interface to allow for possible future changes to the component.

### Quantaswap Pool

A quantaswap pool is concentrated liquidity automated market maker for two fungible tokens. Liquidity is added into discrete pre-defined bins. Swaps then occur within the active bin. Liquidity provided to the pool is represented as liquidity receipt which can hold up to 50 different positions. This allows for a more efficient use of liquidity, automatic compounding of fees, and easily customized liquidity provisioning strategies.

### Pool Maps

The quantaswap factory stores a map of pool addresses to the token pair as well as a map of token pairs to a list of pool. These maps can be used to easily determine which pools have been created by the factory and which pools are available for a given token pair.

## Instantiation

The quantaswap factory is instantiated using the function `new` with the following parameters:

- `admin_badge_address: ResourceAddress` - The address of the admin badge that will be set as the owner of the quantaswap factory.
- `token_validator_address: ComponentAddress` - The address of the token validator component that will be used to validate tokens.

## Methods

### New Pool

The quantaswap factory has a method `new_pool` that can be used to create a new pool given a token pair. The tokens are first validated using the `TokenValidator` component. The pool is then instantiated with the default owner and user rules and the address stored in the quantaswap factory. A reserved global address can optionally be provided.

### Get Methods

- `get_owner_rule_default`
- `get_user_rule_default`
- `get_fee_vaults_address`
- `get_fee_controller_address`
- `get_token_validator_address`
- `get_pool_count`
- `get_pools`
- `get_pool_pair`
- `get_pools_by_pair`

### Set Methods

The following methods can only be called by the owner of the quantaswap factory.

- `set_owner_rule_default`
- `set_user_rule_default`
- `set_token_validator`

## Events

The quantaswap factory emits events for the following actions:

- `SetOwnerRuleDefaultEvent` - Emitted when the owner rule default is set.
- `SetUserRuleDefaultEvent` - Emitted when the user rule default is set.
- `SetTokenValidatorEvent` - Emitted when the token validator is set.
- `NewPoolEvent` - Emitted when a new pool is created.

## Permissions

### Owner Permissions

The `owner` role can take following actions:

- Update the `owner` role access rule.
- Update the `user` role access rule.
- Update metadata for the quantaswap factory.
- Set the default owner role.
- Set the default user role.
- Set the token validator component.

### User Permissions

The `user` role can take the following actions:

- Create a new quantaswap pool.
