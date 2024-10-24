# Order Book Factory

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Overview](#overview)
  - [External Components](#external-components)
  - [Order Book](#order-book)
  - [Order Book Maps](#order-book-maps)
- [Instantiation](#instantiation)
- [Methods](#methods)
  - [New Order Book](#new-order-book)
  - [Get Methods](#get-methods)
  - [Set Methods](#set-methods)
- [Events](#events)
- [Permissions](#permissions)
  - [Owner Permissions](#owner-permissions)
  - [User Permissions](#user-permissions)

## Introduction

This package implements the blueprint for an order book factory that creates order books for two validated fungible tokens. The order book factory is responsible for creating order books, storing their addresses, and for setting owner and user permissions for the order books it creates.

## Getting Started

### Docs

Rust docs are provided. To generate them, from the order book factory directory run:

```bash
cargo doc --no-deps --open
```

### Testing

A full set of tests are provided that use the scrypto test runner. At times there is a quirk with the test runner where a race condition causes the code to not compile before the test runner tries to run it. This can randomly cause tests to fail. The test runner is also very slow. For these reasons, instead of using the default command `scrypto test` it is preferable to use Nextest which can be downloaded from here: <https://nexte.st>. To run the full set of tests, from the order book factory directory run:

```bash
cargo nextest run -r --retries 3
```

## Overview

### External Components

The order book factory depends on the `TokenValidator` component to validate the tokens that are used to create order books. The address of this component can be set by the owner of the order book factory and uses a generic interface to allow for possible future changes to the component.

### Order Book

An order book is a market for two fungible tokens. Limit orders are placed at a specific price and executed in a FIFO ordering at that price. There can never be overlap between ask and bid limits. Market orders execute against the best available limit orders. When a limit order is placed an order receipt is minted. The order receipt can be used to claim tokens from the limit order which consumes the order receipt.

### Order Book Maps

The order book factory stores a map of order book addresses to the token pair as well as a map of token pairs to a list of order books. These maps can be used to easily determine which order books have been created by the factory and which order books are available for a given token pair.

## Instantiation

The order book factory is instantiated using the function `new` with the following parameters:

- `admin_badge_address: ResourceAddress` - The address of the admin badge that will be set as the owner of the order book factory.
- `token_validator_address: ComponentAddress` - The address of the token validator component that will be used to validate tokens.

## Methods

### New Order Book

The order book factory has a method `new_order_book` that can be used to create a new order book given a token pair. The tokens are first validated using the `TokenValidator` component. The order book is then instantiated with the default owner and user rules and the address stored in the order book factory. A reserved global address can optionally be provided.

### Get Methods

- `get_owner_rule_default`
- `get_user_rule_default`
- `get_fee_vaults_address`
- `get_fee_controller_address`
- `get_token_validator_address`
- `get_order_book_count`
- `get_order_books`
- `get_order_book_pair`
- `get_order_books_by_pair`

### Set Methods

The following methods can only be called by the owner of the order book factory.

- `set_owner_rule_default`
- `set_user_rule_default`
- `set_token_validator`

## Events

The order book factory emits events for the following actions:

- `SetOwnerRuleDefaultEvent` - Emitted when the owner rule default is set.
- `SetUserRuleDefaultEvent` - Emitted when the user rule default is set.
- `SetTokenValidatorEvent` - Emitted when the token validator is set.
- `NewOrderBookEvent` - Emitted when a new order book is created.

## Permissions

### Owner Permissions

The `owner` role can take following actions:

- Update the `owner` role access rule.
- Update the `user` role access rule.
- Update metadata for the order book factory.
- Set the default owner role.
- Set the default user role.
- Set the token validator component.

### User Permissions

The `user` role can take the following actions:

- Create a new order book.
