use scrypto::prelude::*;

use crate::price::Price;
use crate::order_book::order_book::OrderBookKeyValueStore;

/// Index of prices for all active limit orders in the order book.
/// 
/// # Responsible for
/// 
/// * Tracking current ask and bid price.
/// * Finding next ask or bid price.
/// 
#[derive(ScryptoSbor)]
pub struct PriceIndex {
    /// Key value store of index nodes.
    kvs: KeyValueStore<u32, IndexNode>,
    /// Current ask price. This is the lowest ask price.
    current_ask: Option<Price>,
    /// Current bid price. This is the highest bid price.
    current_bid: Option<Price>,
}

/// Index node. This is a small bit map representing the existence of a range of prices.
/// The key for the node encodes the range of prices for which the index node is responsible.
/// 
/// # Responsible for
/// 
/// * Tracking the existence of prices in the price range.
/// * Finding the next or previous price in the price range if it exists.
///
#[derive(ScryptoSbor, Clone, Copy)]
pub struct IndexNode {
    index: u64,
}

impl IndexNode {
    /// Size of index in index node.
    pub const INDEX_SIZE: u32 = u64::BITS;
    /// Mask for getting factor from value.
    pub const FACTOR_MASK: u32 = Self::INDEX_SIZE - 1;

    /// Create a new index node.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to insert into the index node.
    /// 
    /// # Returns
    /// 
    /// * `Self` - The new index node.
    /// 
    fn new(value: u32) -> Self {
        // Calculate mask of bit to set
        let factor: u32 = value & Self::FACTOR_MASK;
        let mask: u64 = 1 << factor;
        // Create index node
        IndexNode {
            index: mask,
        }
    }

    /// Get the first value in the index node.
    /// 
    /// # Returns
    /// 
    /// * `Option<u32>` - The first value in the index node. None if there is no value.
    /// 
    fn first(&self) -> Option<u32> {
        // Check if index node is empty
        if self.index > 0 {
            Some(self.index.trailing_zeros())
        } else {
            None
        }
    }

    /// Get the last value in the index node.
    /// 
    /// # Returns
    /// 
    /// * `Option<u32>` - The last value in the index node. None if there is no value.
    /// 
    fn last(&self) -> Option<u32> {
        // Check if index node is empty
        if self.index > 0 {
            Some(Self::INDEX_SIZE - 1 - self.index.leading_zeros())
        } else {
            None
        }
    }

    /// Get the next value in the index node.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to get the next value after.
    /// 
    /// # Returns
    /// 
    /// * `Option<u32>` - The next value in the index node. None if there is no value.
    /// 
    fn next(&self, value: u32) -> Option<u32> {
        // Bit position of value
        let factor: u32 = value & Self::FACTOR_MASK;

        // Bit position to start looking after
        let pos: u32 = factor + 1;

        // Check if at end of index node
        if pos == Self::INDEX_SIZE {
            return None;
        }

        // Check if next value exists
        let index_after: u64 = self.index >> pos;
        if index_after > 0 {
            Some(index_after.trailing_zeros() + pos)
        } else {
            None
        }
    }

    /// Get the previous value in the index node.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to get the previous value before.
    /// 
    /// # Returns
    /// 
    /// * `Option<u32>` - The previous value in the index node. None if there is no value.
    /// 
    fn prev(&self, value: u32) -> Option<u32> {
        // Bit position of value
        let factor: u32 = value & Self::FACTOR_MASK;

        // Check if at start of index node
        if factor == 0 {
            return None;
        }
        
        // Bit position to start looking before
        let pos: u32 = factor - 1;
        
        // Check if previous value exists
        let index_after: u64 = self.index << (Self::INDEX_SIZE - factor);
        if index_after > 0 {
            Some(pos - index_after.leading_zeros())
        } else {
            None
        }
    }

    /// Insert a value into the index node.
    /// Returns true if the value was not already in the index node.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to insert into the index node.
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the value was not already in the index node.
    /// 
    fn insert(&mut self, value: u32) -> bool {
        // Calculate mask of bit to set
        let factor: u32 = value & Self::FACTOR_MASK;
        let mask: u64 = 1 << factor;

        // Check if value already in index node
        let contains: bool = self.index & mask == mask;

        // Set bit
        self.index |= mask;
        
        // Return if value was not already in index node
        !contains
    }

    /// Remove a value from the index node.
    /// Returns true if the value was the only value in the index node.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to remove from the index node.
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the value was the only value in the index node.
    /// 
    fn remove(&mut self, value: u32) -> bool {
        // Calculate mask of bit to unset
        let factor: u32 = value & Self::FACTOR_MASK;
        let mask: u64 = 1 << factor;

        // Check if value is only value in index node
        let only: bool = self.index == mask;

        // Unset bit
        self.index &= !mask;

        // Return if value was the only value in the index node
        only
    }
}

impl PriceIndex {
    /// Shift for getting next node key.
    pub const FACTOR_SHIFT: u32 = IndexNode::INDEX_SIZE.ilog2();
    /// Number of used price bits. Must be multiple of FACTOR_SHIFT.
    pub const PRICE_BITS_USED: u32 = (Price::SIG_BITS + Price::EXP_BITS + Self::FACTOR_SHIFT - 1) / Self::FACTOR_SHIFT * Self::FACTOR_SHIFT;  
    /// Flag for getting key from value, used to make keys unique.
    pub const KEY_FLAG: u32 = 1 << Self::PRICE_BITS_USED;

    /// Create a new price index.
    /// 
    /// # Returns
    /// 
    /// * `Self` - The new price index.
    /// 
    pub fn new() -> Self {
        PriceIndex {
            kvs: KeyValueStore::new_with_registered_type(),
            current_ask: None,
            current_bid: None,
        }
    }

    /// Get the current ask price.
    /// 
    /// # Returns
    /// 
    /// * `Option<Price>` - The current ask price. None if there are no ask prices.
    /// 
    pub fn current_ask(&self) -> Option<Price> {
        self.current_ask
    }

    /// Get the current bid price.
    /// 
    /// # Returns
    /// 
    /// * `Option<Price>` - The current bid price. None if there are no bid prices.
    /// 
    pub fn current_bid(&self) -> Option<Price> {
        self.current_bid
    }

    /// Get the next higher value in the price index.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to get the next higher price after.
    /// 
    /// # Returns
    /// 
    /// * `Option<Price>` - The next higher price. None if there are no more values.
    /// 
    pub fn next_up(&self, value: Price) -> Option<Price> {
        // Transform value into node keys
        let value: u32 = value.0;
        let mut node_key: u32 = value | Self::KEY_FLAG;
        let mut parent_node_key: u32 = node_key >> Self::FACTOR_SHIFT;

        // Iterate up the tree to find next
        loop {
            match self.kvs.get(&parent_node_key) {
                Some(parent_node) =>
                    match parent_node.next(node_key) {
                        Some(mut next) => {
                            // Iterate down the tree to reconstruct value
                            loop {
                                // Calculate next key
                                parent_node_key = (parent_node_key << Self::FACTOR_SHIFT) | next;

                                // Check if done
                                if parent_node_key & Self::KEY_FLAG == Self::KEY_FLAG {
                                    return Some(Price(parent_node_key & !Self::KEY_FLAG));
                                }

                                // Get factor to reconstruct value
                                next = self.kvs.get(&parent_node_key).unwrap().first().unwrap();
                            }
                        }
                        None => {
                            // Check if at root node, otherwise continue up the tree
                            if parent_node_key >= IndexNode::INDEX_SIZE {
                                node_key = parent_node_key;
                                parent_node_key >>= Self::FACTOR_SHIFT;
                            } else {
                                return None;
                            }
                        }
                    }
                None => {
                    // Check if at root node, otherwise continue up the tree
                    if parent_node_key >= IndexNode::INDEX_SIZE {
                        node_key = parent_node_key;
                        parent_node_key >>= Self::FACTOR_SHIFT;
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    /// Get the next lower value in the price index.
    /// Returns None if there are no more values.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to get the next lower price before.
    /// 
    /// # Returns
    /// 
    /// * `Option<Price>` - The next lower price. None if there are no more values.
    /// 
    pub fn next_down(&self, value: Price) -> Option<Price> {
        // Transform value into node keys
        let value: u32 = value.0;
        let mut node_key: u32 = value | Self::KEY_FLAG;
        let mut parent_node_key: u32 = node_key >> Self::FACTOR_SHIFT;

        // Iterate up the tree to find prev
        loop {
            match self.kvs.get(&parent_node_key) {
                Some(parent_node) =>
                    match parent_node.prev(node_key) {
                        Some(mut prev) => {
                            // Iterate down the tree to reconstruct value
                            loop {
                                // Calculate next key
                                parent_node_key = (parent_node_key << Self::FACTOR_SHIFT) | prev;

                                // Check if done
                                if parent_node_key & Self::KEY_FLAG == Self::KEY_FLAG {
                                    return Some(Price(parent_node_key & !Self::KEY_FLAG));
                                }

                                // Get factor to reconstruct value
                                prev = self.kvs.get(&parent_node_key).unwrap().last().unwrap();
                            }
                        }
                        None => {
                            // Check if at root node, otherwise continue up the tree
                            if parent_node_key >= IndexNode::INDEX_SIZE {
                                node_key = parent_node_key;
                                parent_node_key >>= Self::FACTOR_SHIFT;
                            } else {
                                return None;
                            }
                        }
                    }
                None => {
                    // Check if at root node, otherwise continue up the tree
                    if parent_node_key >= IndexNode::INDEX_SIZE {
                        node_key = parent_node_key;
                        parent_node_key >>= Self::FACTOR_SHIFT;
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    /// Insert a value into the price index.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to insert into the price index.
    /// * `is_ask` - True if the value is an ask price, false if the value is a bid price.
    /// 
    /// # Requires
    /// 
    /// * 'value` - If is ask then value must be greater than current bid price.
    /// * 'value` - If is bid then value must be less than current ask price.
    /// 
    pub fn insert(&mut self, value: Price, is_ask: bool) {
        // Transform value into node keys
        let mut node_key: u32 = value.0 | Self::KEY_FLAG;
        let mut parent_node_key: u32 = node_key >> Self::FACTOR_SHIFT;
        let mut not_set: bool = true;

        // Iterate up the tree to set index bits
        loop {
            // Set index bit
            let parent_node = match self.kvs.get(&parent_node_key) {
                Some(parent_node) => {
                    let mut parent_node = *parent_node;
                    not_set = parent_node.insert(node_key);
                    parent_node
                },
                None => {
                    IndexNode::new(node_key)
                }
            };
            self.kvs.insert(parent_node_key, parent_node);
            
            // Check if not done
            if not_set && parent_node_key >= IndexNode::INDEX_SIZE {
                node_key = parent_node_key;
                parent_node_key >>= Self::FACTOR_SHIFT;
            } else {
                break;
            }
        }

        // Update current price if necessary
        if is_ask && (self.current_ask.is_none() || value < self.current_ask.unwrap()) {
            self.current_ask = Some(value);
        } else if !is_ask && (self.current_bid.is_none() || value > self.current_bid.unwrap()) {
            self.current_bid = Some(value);
        }
    }

    /// Remove a value from the price index.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to remove from the price index.
    /// 
    /// # Requires
    /// 
    /// * `value` - Is in the price index.
    /// 
    pub fn remove(&mut self, value: Price) {
        // Transform value into node keys
        let mut node_key: u32 = value.0 | Self::KEY_FLAG;
        let mut parent_node_key: u32 = node_key >> Self::FACTOR_SHIFT;
        let mut only: bool;

        // Iterate up the tree to unset index bits
        loop {
            // Get parent node
            let mut parent_node = *self.kvs.get(&parent_node_key).unwrap();

            // Unset index bit
            only = parent_node.remove(node_key);
            self.kvs.insert(parent_node_key, parent_node);

            // Check if not done
            if only && parent_node_key >= IndexNode::INDEX_SIZE {
                node_key = parent_node_key;
                parent_node_key >>= Self::FACTOR_SHIFT;
            } else {
                // If removed value was current ask/bid then get next ask/bid
                if self.current_ask == Some(value) {
                    loop {
                        if let Some(mut first) = parent_node.next(node_key) {
                            // Iterate down the tree to reconstruct next ask value
                            loop {
                                // Calculate next key
                                parent_node_key = (parent_node_key << Self::FACTOR_SHIFT) | first;
            
                                // Check if done
                                if parent_node_key & Self::KEY_FLAG == Self::KEY_FLAG {
                                    self.current_ask = Some(Price(parent_node_key & !Self::KEY_FLAG));
                                    return;
                                }
                                
                                // Get factor to reconstruct value
                                first = self.kvs.get(&parent_node_key).unwrap().first().unwrap();
                            }
                        } else if parent_node_key >= IndexNode::INDEX_SIZE {
                            // Iterate up the tree
                            node_key = parent_node_key;
                            parent_node_key >>= Self::FACTOR_SHIFT;
                            parent_node = *self.kvs.get(&parent_node_key).unwrap()
                        } else {
                            // No more ask prices
                            self.current_ask = None;
                            return;
                        }
                    }
                } else if self.current_bid == Some(value) {
                    loop {
                        if let Some(mut last) = parent_node.prev(node_key) {
                            // Iterate down the tree to reconstruct next bid value
                            loop {
                                // Calculate next key
                                parent_node_key = (parent_node_key << Self::FACTOR_SHIFT) | last;

                                // Check if done
                                if parent_node_key & Self::KEY_FLAG == Self::KEY_FLAG {
                                    self.current_bid = Some(Price(parent_node_key & !Self::KEY_FLAG));
                                    return;
                                }
                                
                                // Get factor to reconstruct value
                                last = self.kvs.get(&parent_node_key).unwrap().last().unwrap();
                            }
                        } else if parent_node_key >= IndexNode::INDEX_SIZE {
                            // Iterate up the tree
                            node_key = parent_node_key;
                            parent_node_key >>= Self::FACTOR_SHIFT;
                            parent_node = *self.kvs.get(&parent_node_key).unwrap()
                        } else {
                            // No more bid prices
                            self.current_bid = None;
                            return;
                        }
                    }
                } else {
                    // No need to update current ask/bid
                    return;
                }
            }
        }
    }
}
