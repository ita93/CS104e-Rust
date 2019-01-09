use std::fmt;
use alloc::heap::{AllocErr, Layout};

use allocator::util::*;
use allocator::linked_list::LinkedList;

const MIN_ORDER: usize = 3; //minimum allocation: 8bytes;
const MIN_SIZE: usize = 8;

/// A simple allocator that allocates based on size classes.
pub struct Allocator {
    // FIXME: Add the necessary fields.
    free_list: [LinkedList; 32],
    allocated: usize,
    total: usize,
    highest_order: usize,
}

//return order  and decimal presentation
fn lower_power_of_two(mut number: usize) -> (usize, usize) {
    let mut count:usize = 0;
    while (number >> 1) > 0 {
        count += 1;
        number = number >> 1;
    }
    let decimal = (2 as usize).pow(count as u32);
    (count, decimal)
}

fn upper_power_of_two(mut number: usize) -> (usize, usize) {
    let mut count:usize = 0;
    while (number >> 1) > 0 {
        count += 1;
        number = number >> 1;
    }
    count += 1;
    let decimal = (2 as usize).pow(count as u32);
    (count, decimal)
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        let mut total = 0;
        let mut highest_order = 0;
        let mut free_list = [LinkedList::new(); 32];
        let mut lower_bound: usize = start;
        while lower_bound + MIN_SIZE <= end {
            let (order, block_size) = lower_power_of_two(end - lower_bound);
            unsafe{
                free_list[order].push(lower_bound as *mut usize);
            }
            total += block_size;
            lower_bound += block_size;
            if highest_order == 0 {
                highest_order = order;
            }
        }
        Allocator {
            free_list,
            allocated: 0,
            total,
            highest_order,
        }
    }

    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning `Err` indicates that either memory is exhausted
    /// (`AllocError::Exhausted`) or `layout` does not meet this allocator's
    /// size or alignment constraints (`AllocError::Unsupported`).
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        if layout.size() < 0 || layout.align() & (layout.align() - 1) !=0 || layout.align() == 0 {
            return Err(AllocErr::Unsupported{details: "Align and size are invalid"});
        }

        let act_size = align_up(layout.size(), layout.align());
        let (order, block_size) = upper_power_of_two(act_size);

        //check in free list
        if !self.free_list[order].is_empty() {
            self.allocated += block_size;
            unsafe{
                return Ok(self.free_list[order].pop().unwrap() as *mut u8);
            }
        } 

        if order > self.highest_order {
            return Err(AllocErr::Exhausted{request: layout});
        }        

        let mut idx = order;

        //find the nearest order that is available
        while self.free_list[idx].is_empty() {
            idx += 1;
            
        }

        //split from available order 
        while idx > order {
            let first_block = self.free_list[idx].pop();
            idx -= 1;
            unsafe{
                self.free_list[idx].push(first_block.unwrap());
                let second_block = *first_block.unwrap() + (2 as usize).pow(idx as u32);
                self.free_list[idx].push(second_block as *mut usize);
            }        
        }
        if !self.free_list[order].is_empty() {
            self.allocated += block_size;
            unsafe{
                return Ok(self.free_list[order].pop().unwrap() as *mut u8);
            }
        } else {
            Err(AllocErr::Exhausted{request: layout})
        }
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        unimplemented!("bin deallocation")
    }
}
//
// FIXME: Implement `Debug` for `Allocator`.
impl fmt::Debug for Allocator {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BinAllocator")
            .field("allocated", &self.allocated)
            .field("total", &self.total)
            .finish()
    }
}