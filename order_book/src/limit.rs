use scrypto::prelude::*;

use crate::order_receipt::*;

/// Holds a sum of all limit orders of one side of the order book for a given price.
/// 
/// # Responsible for
/// 
/// * Track token amounts.
/// * Allocate tokens to order receipts.
/// * Maintain head and tail of order receipt linked list used for FIFO ordering of limit orders.
/// 
/// # Not responsible for
/// 
/// * Minting, burning, or updating order receipts.
/// * Moving tokens.
/// * Calculating between tokens x and tokens y.
/// 
#[derive(ScryptoSbor, Clone, Copy)]
pub struct Limit {
    /// Sum of tokens calculated in tokens x available.
    amount_x: Decimal,
    /// Sum of tokens calculated in tokens x that have been sold and not allocated to an order.
    amount_x_unallocated: Decimal,
    /// Head of order receipts linked list, the first order to be filled.
    head_id: u64,
    /// Tail of order receipts linked list, the last order to be filled.
    tail_id: u64,
}

impl Limit {
    /// Create a new limit.
    /// 
    /// # Returns
    /// 
    /// * `Self` - The new limit.
    /// 
    pub fn new() -> Self {
        // Create limit
        Self { 
            amount_x: Decimal::zero(), 
            amount_x_unallocated: Decimal::zero(), 
            head_id: 0,
            tail_id: 0,
        }
    }

    /// Get if limit is empty.
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if limit is empty.
    /// 
    pub fn is_empty(&self) -> bool {
        self.amount_x == Decimal::zero()
    }

    /// Get amount of tokens calculated in tokens x.
    /// 
    /// # Returns
    /// 
    /// * `Decimal` - Amount of tokens calculated in tokens x.
    /// 
    pub fn get_amount_x(&self) -> Decimal {
        self.amount_x
    }

    /// Get amount of tokens calculated in tokens x that have been filled and not unallocated.
    /// 
    /// # Returns
    /// 
    /// * `Decimal` - Amount of tokens calculated in tokens x.
    /// 
    pub fn get_amount_x_unallocated(&self) -> Decimal {
        self.amount_x_unallocated
    }

    /// Get id of the order receipt that is the first order in the linked list.
    /// 
    /// # Returns
    /// 
    /// * `u64` - Id of the order receipt.
    /// 
    pub fn get_head_id(&self) -> u64 {
        self.head_id
    }

    /// Get id of the order receipt that is the last order in the linked list.
    /// 
    /// # Returns
    /// 
    /// * `u64` - Id of the order receipt.
    /// 
    pub fn get_tail_id(&self) -> u64 {
        self.tail_id
    }

    /// Add a limit order to the limit.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - Id of the order receipt.
    /// * `amount_x` - Amount of tokens calculated in tokens x.
    /// 
    /// # Returns
    /// 
    /// * `u64` - Id of the order receipt that was previously the last order in the linked list.
    /// 
    /// # Requires
    /// 
    /// * `order_id` - Id of the order receipt must be greater than 0.
    /// * `order_id` - Id of the order receipt must be greater than the previous order receipt.
    /// * `order_id` - Id must be for a valid order receipt.
    /// * `amount_x` - Amount of tokens must match the amount in the order receipt.
    /// 
    pub fn add_order(&mut self, order_id: u64, amount_x: &Decimal) -> u64 {
        let tail_id = self.tail_id;
        self.tail_id = order_id;
        if self.head_id == 0 {
            self.head_id = order_id;
        }
        self.amount_x += *amount_x;
        
        tail_id
    }
    
    /// Fill the limit by some amount.
    /// 
    /// # Arguments
    /// 
    /// * `amount_x` - Amount of tokens calculated in tokens x.
    /// * `order_receipt_manager` - Order receipt manager.
    /// 
    pub fn fill(&mut self, amount_x: &Decimal, order_receipt_manager: &ResourceManager) {
        self.amount_x -= *amount_x;
        self.amount_x_unallocated += *amount_x;

        self.allocate(order_receipt_manager);
    }

    /// Fully fill the limit.
    pub fn fully_fill(&mut self) {
        self.amount_x = Decimal::zero();
        self.amount_x_unallocated = Decimal::zero();
        self.head_id = 0;
        self.tail_id = 0;
    }

    /// Helper method to allocate tokens to orders receipts.
    /// 
    /// # Arguments
    /// 
    /// * `order_receipt_manager` - Order receipt manager.
    /// 
    fn allocate(&mut self, order_receipt_manager: &ResourceManager) {
        // Get next order
        loop {
            // Get order data
            let order_data = order_receipt_manager
                .get_non_fungible_data::<OrderReceipt>(&NonFungibleLocalId::integer(self.head_id));

            // If sold tokens_a left to allocate
            if self.amount_x_unallocated >= order_data.amount {
                // Allocate sold tokens_a to fill order
                self.amount_x_unallocated -= order_data.amount;
                self.head_id = order_data.next_id;
            } else {
                return;
            }
        }
    }

    /// Claim tokens owned by a order receipt.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - Id of the order receipt.
    /// * `order_data` - Order receipt data.
    /// 
    /// # Returns
    /// 
    /// * `Decimal` - Amount of tokens calculated in tokens x that were canceled.
    /// * `Decimal` - Amount of tokens calculated in tokens x that were filled.
    /// 
    /// # Requires
    /// 
    /// * `order_id` - Must be a valid order id.
    /// * `order_data` - Must match the order receipt id.
    /// 
    pub fn claim_order(&mut self, order_id: u64, order_data: &OrderReceipt) -> (Decimal, Decimal) {
        // Get amounts due
        let amount_order = order_data.amount;
        let (amount_canceled, amount_filled): (Decimal, Decimal) = 
            if self.head_id == 0 || order_id < self.head_id {
                self.claim_filled_order(amount_order)
            } else if order_id == self.head_id {
                self.cancel_active_order(amount_order)
            } else {
                self.cancel_order(amount_order)
            };

        // Update pointers
        if order_id == self.head_id {
            self.head_id = order_data.next_id;
        }
        if order_id == self.tail_id {
            self.tail_id = order_data.prev_id;
        }

        // Return amounts due
        (amount_canceled, amount_filled)
    }

    /// Helper method to claim tokens owned by a filled order.
    /// 
    /// # Arguments
    /// 
    /// * `amount_order` - Amount of tokens the order was for calculated in tokens x.
    /// 
    /// # Returns
    /// 
    /// * `Decimal` - Amount of tokens calculated in tokens x that were canceled. This is always zero.
    /// * `Decimal` - Amount of tokens calculated in tokens x that were filled.
    /// 
    /// # Requires
    /// 
    /// * `amount_order` - Must match the amount in the order receipt.
    /// 
    fn claim_filled_order(&mut self, amount_order: Decimal) -> (Decimal, Decimal) {
        // Calculate amounts
        let amount_canceled = Decimal::zero();
        let amount_filled = amount_order;

        // Return amounts due
        (amount_canceled, amount_filled)
    }

    /// Helper method to claim tokens owned by a limit order being canceled.
    /// 
    /// # Arguments
    /// 
    /// * `amount_order` - Amount of tokens the order was for calculated in tokens x.
    /// 
    /// # Returns
    /// 
    /// * `Decimal` - Amount of tokens calculated in tokens x that were canceled.
    /// * `Decimal` - Amount of tokens calculated in tokens x that were filled. This is always zero.
    /// 
    /// # Requires
    /// 
    /// * `amount_order` - Must match the amount in the order receipt.
    /// 
    fn cancel_order(&mut self, amount_order: Decimal) -> (Decimal, Decimal) {
        // Calculate amounts
        let amount_canceled = amount_order;
        let amount_filled = Decimal::zero();

        // Update limit
        self.amount_x -= amount_canceled;

        // Return amounts due
        (amount_canceled, amount_filled)
    }

    /// Helper method to claim tokens owned by the active limit order being canceled.
    /// 
    /// # Arguments
    /// 
    /// * `amount_order` - Amount of tokens the order was for calculated in tokens x.
    /// 
    /// # Returns
    /// 
    /// * `Decimal` - Amount of tokens calculated in tokens x that were canceled.
    /// * `Decimal` - Amount of tokens calculated in tokens x that were filled.
    /// 
    /// # Requires
    /// 
    /// * `amount_order` - Must match the amount in the order receipt.
    /// 
    fn cancel_active_order(&mut self, amount_order: Decimal) -> (Decimal, Decimal) {
        // Calculate amounts
        let amount_canceled = amount_order - self.amount_x_unallocated;
        let amount_filled = self.amount_x_unallocated;

        // Update limit
        self.amount_x -= amount_canceled;
        self.amount_x_unallocated = Decimal::zero();

        // Return amounts due
        (amount_canceled, amount_filled)
    }
}
