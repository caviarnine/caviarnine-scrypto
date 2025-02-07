# Order Book

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Overview](#overview)
  - [External Components](#external-components)
  - [Order Receipt](#order-receipt)
  - [Limit](#limit)
  - [Price to Limit Maps](#price-to-limit-maps)
  - [Price](#price)
  - [Price Index](#price-index)
- [Instantiation](#instantiation)
- [Methods](#methods)
  - [Limit Order](#limit-order)
  - [Limit Order Batch](#limit-order-batch)
  - [Market Order](#market-order)
  - [Claim Orders](#claim-orders)
  - [Get Methods](#get-methods)
- [Events](#events)
- [Permissions](#permissions)
  - [Owner Permissions](#owner-permissions)
  - [User Permissions](#user-permissions)

## Introduction

This package implements the blueprint for an order book that creates a market for two fungible tokens. Limit orders are placed at a specific price and executed in a FIFO ordering at that price. There can never be overlap between ask and bid limits. Market orders execute against the best available limit orders. When a limit order is placed an order receipt is minted. The order receipt can be used to claim tokens from the limit order which consumes the order receipt. Protocol fees are set by the [fee controller](../fee_controller/) and collected by the [fee vaults](../fee_vaults/).

## Getting Started

### Docs

Rust docs are provided. To generate them, from the order book directory run:

```bash
cargo doc --no-deps --open
```

### Testing

A full set of tests are provided that use the scrypto test runner. At times there is a quirk with the test runner where a race condition causes the code to not compile before the test runner tries to run it. This can randomly cause tests to fail. The test runner is also very slow. For these reasons, instead of using the default command `scrypto test` it is preferable to use Nextest which can be downloaded from here: <https://nexte.st>. To run the full set of tests, from the order book directory run:

```bash
cargo nextest run -r --retries 3
```

## Overview

The order book uses several sub-structures to operate.

### External Components

An order book component depends on two external components: `FeeController` and `FeeVaults`. These components are used to manage fees for market orders. The `FeeController` component provides a method to get the protocol fee percentage, and the `FeeVaults` component is used collect the fees. The addresses of these components is hard coded into the order book blueprint.

### Order Receipt

An `OrderReceipt` stores the information of an individual limit order. This includes:

- `is_ask` - Whether the order is an ask or bid limit order.
- `price` - Calculated as `tokens_y/tokens_x`.
- `amount` - Valued in `tokens_x`.
- `next` - Next order receipt in the execution queue.
- `prev` - Previous order receipt in the the execution queue.

`OrderReceipt`'s are assigned ascending integer ids so that the position of a limit order in the execution queue in relation to the active order can be easily determined. An `OrderReceipt` represents ownership of a limit order and is used to claim tokens from that order.

### Limit

A `Limit` is a bundle of all limit orders of the same type at the same price. A `Limit` tracks the sum of tokens available at that price and maintains the head and tail of the linked list FIFO execution queue. This allows for execution of the whole bundle at once ignoring execution ordering in the case were a market order exceeds the sum of tokens.

### Price to Limit Maps

Two key-value stores are used to store limits, one for asks, one for bids. They function as maps from prices to limits. The key price could come from the order receipt or the price index depending on the action. Key-value stores are used as they are a scalable storage solution.

### Price

Decimal price values which are fixed point and contain 192 bits are convert to the `Price` type. The `Price` type is a base 10 floating point number that uses 32 bits and has 5 significant figures of precision. More precision could be allowed at this data size, but it is desirable for the number of possible prices to be reasonably limited. The conversion process truncates the decimal value to fit. The allowed range of values is `10^11` to `10^-11`.

### Price Index

The `PriceIndex` stores all active limit prices in sorted set. It uses a key-value store (KVS) as the underlying structure. On top of this KVS is built a trie for the limited universe of possible prices. The `PriceIndex` is used to efficiently find the best available limit prices when executing a market order.

## Instantiation

A new order book is created using the `new` function. This function takes the following arguments:

- `owner_rule: AccessRule` - Access rule for the `owner` role.
- `user_rule: AccessRule` - Access rule for the `user` role.
- `token_x_address: ResourceAddress` - Address of the token for the ask side of the order book.
- `token_y_address: ResourceAddress` - Address of the token for the bid side of the order book.
- `reservation: Option<GlobalAddressReservation>` - Optional global address reservation for the order book.

## Methods

The order book has three primary actions. Place a limit order, execute a market order, and claim tokens from an order receipt.

### Limit Order

Places a limit order into the order book at a specific price. The price is truncated to 5 significant figures. A non-zero positive price and a minimum order size is required to avoid spam cluttering the order book. A market order will first be executed to clear any overlap of price between the ask and bid sides of the order book. The limit order will then be created with the remaining tokens.

### Limit Order Batch

Places a batch of limit orders into the order book. This is a convenience method that allows for multiple limit orders to be placed at once. This is useful to reduce gas costs when placing multiple limit orders at once.

### Market Order

Executes a market order on the order book. This matches with best available limits. Optionally a stop price can be provided at which the market order will not execute beyond. A percentage fee is subtracted from the input tokens before execution. This fee is controlled by the fee controller and sent to the fee vaults.

### Claim Orders

Claim tokens owned by order receipts. This consumes the order receipts. If the limit order has not been filled, this means canceling the order. If the limit order has been filled, this means claiming bought tokens. If the limit order has been partially filled, this means a combination of both canceling the remaining part of the order and claiming bought tokens.

### Get Methods

Getter methods are provided to easily query the state of an order book. This includes getting basic configuration information as well as things like prices, available limits, and the current state of a limit order. The following getters methods are provided:

- `get_fee_controller_address`
- `get_fee_vaults_address`
- `get_token_x_address`
- `get_token_y_address`
- `get_order_receipt_address`
- `get_amount_x`
- `get_amount_y`
- `get_last_price`
- `get_current_ask_price`
- `get_current_bid_price`
- `get_ask_limits`
- `get_bid_limits`
- `get_order_status`
- `get_order_statuses`

## Events

The order book emits events for the following actions:

- `NewOrderBookEvent` - A new order book has been created.
- `LimitOrderEvent` - A limit order has been placed.
- `MarketOrderEvent` - A market order has been executed.
- `ClaimOrderEvent` - An order receipt has been claimed.
- `ProtocolFeeEvent` - A protocol fee has been collected.

## Permissions

### Owner Permissions

The `owner` role can take following actions:

- Update the `owner` role access rule.
- Update the `user` role access rule.
- Update metadata for the order book.
- Update metadata for the order receipts.

### User Permissions

The `user` role can take the following actions:

- Place limit orders.
- Execute market orders.

Note, claiming orders can not be restricted.
