use scrypto::prelude::*;

use crate::tick::*;
use crate::quantaswap::quantaswap::QuantaSwapKeyValueStore;

/// Index of ticks for all active liquidity positions.
/// 
/// # Responsible for
/// 
/// * Tracking current tick.
/// * Finding next tick up or down.
/// 
#[derive(ScryptoSbor)]
pub struct TickIndex {
    /// Key value store of index nodes.
    kvs: KeyValueStore<u32, IndexNode>,
    /// Current price tick.
    current: Option<Tick>,
}

/// Index node. This is a small bit map representing the existence of a range of ticks.
/// The key for the node encodes the range of ticks for which the index node is responsible.
/// 
/// # Responsible for
/// 
/// * Tracking the existence of ticks in the tick range.
/// * Finding the next or previous tick in the tick range if it exists.
///
#[derive(ScryptoSbor, Clone, Copy)]
pub struct IndexNode {
    index: u64,
}

impl IndexNode {
    /// Size of index in index node.
    pub const INDEX_SIZE: u32 = 64;
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

impl TickIndex {
    /// Shift for getting next node key.
    pub const FACTOR_SHIFT: u32 = IndexNode::INDEX_SIZE.ilog2();
    /// Number of used tick bits. Must be multiple of FACTOR_SHIFT.
    pub const TICK_BITS_USED: u32 = (18 + Self::FACTOR_SHIFT - 1) / Self::FACTOR_SHIFT * Self::FACTOR_SHIFT;
    /// Flag for getting key from value, used to make keys unique.
    pub const KEY_FLAG: u32 = 1 << Self::TICK_BITS_USED;

    /// Create a new tick index.
    /// 
    /// # Returns
    /// 
    /// * `Self` - The new price index.
    /// 
    pub fn new() -> Self {
        TickIndex {
            kvs: KeyValueStore::new_with_registered_type(),
            current: None,
        }
    }

    /// Get the current tick.
    /// 
    /// # Returns
    /// 
    /// * `Option<Tick>` - The current tick. None if there is no current tick.
    /// 
    pub fn current(&self) -> Option<Tick> {
        self.current
    }

    /// Get the next higher value in the tick index.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to get the next higher tick after.
    /// 
    /// # Returns
    /// 
    /// * `Option<Price>` - The next higher tick. None if there are no more values.
    /// 
    pub fn next_up(&self, value: Tick) -> Option<Tick> {
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
                                    return Some(Tick(parent_node_key & !Self::KEY_FLAG));
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

    /// Get the next lower value in the tick index.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to get the next lower tick before.
    /// 
    /// # Returns
    /// 
    /// * `Option<Tick>` - The next lower tick. None if there are no more values.
    /// 
    pub fn next_down(&self, value: Tick) -> Option<Tick> {
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
                                    return Some(Tick(parent_node_key & !Self::KEY_FLAG));
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

    /// Insert a value into the tick index.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to insert into the tick index. 
    /// 
    pub fn insert(&mut self, value: Tick) {
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
                }
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

        // Make current if no current
        if self.current.is_none() {
            self.current = Some(value);
        }
    }

    /// Remove a value from the tick index.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to remove from the price index.
    /// 
    /// # Requires
    /// 
    /// * `value` - Is in the price index.
    /// 
    pub fn remove(&mut self, value: Tick) {
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
                // If removed value was current then get closest value
                if self.current == Some(value) {
                    if let Some(next) = parent_node.next(node_key) {
                        let mut first = next;
                        // Iterate down the tree to reconstruct next value
                        loop {
                            // Calculate next key
                            parent_node_key = (parent_node_key << Self::FACTOR_SHIFT) + first;

                            // Check if done
                            if parent_node_key & Self::KEY_FLAG == Self::KEY_FLAG {
                                self.current = Some(Tick(parent_node_key & !Self::KEY_FLAG));
                                return;
                            }

                            // Get factor to reconstruct value
                            first = self.kvs.get(&parent_node_key).unwrap().first().unwrap();
                        }
                    } else if let Some(prev) = parent_node.prev(node_key) {
                        let mut last = prev;
                        // Iterate down the tree to reconstruct prev value
                        loop {
                            // Calculate next key
                            parent_node_key = (parent_node_key << Self::FACTOR_SHIFT) + last;

                            // Check if done
                            if parent_node_key & Self::KEY_FLAG == Self::KEY_FLAG {
                                self.current = Some(Tick(parent_node_key & !Self::KEY_FLAG));
                                return;
                            }

                            // Get factor to reconstruct value
                            last = self.kvs.get(&parent_node_key).unwrap().last().unwrap();
                        }
                    } else {
                        // No more prices
                        self.current = None;
                    }
                } else {
                    // No need to update current
                    return;
                }
            }
        }
    }

    /// Move the current tick to the next higher value in the tick index.
    /// 
    /// # Returns
    /// 
    /// * `Option<Tick>` - The new current tick. None if there are no higher values.
    /// 
    pub fn move_up(&mut self) -> Option<Tick> {
        match self.current {
            // Get current price
            Some(current) => {
                // Get next higher value
                match self.next_up(current) {
                    Some(next) => {
                        self.current = Some(next);
                        Some(next)
                    }
                    None => None
                }
            }
            // No current price
            None => None
        }
    }

    /// Move the current tick to the next lower value in the tick index.
    /// 
    /// # Returns
    /// 
    /// * `Option<Tick>` - The new current tick. None if there are no lower values.
    /// 
    pub fn move_down(&mut self) -> Option<Tick> {
        match self.current {
            Some(current) => {
                match self.next_down(current) {
                    Some(next) => {
                        self.current = Some(next);
                        Some(next)
                    }
                    None => None
                }
            }
            None => None
        }
    }
}
