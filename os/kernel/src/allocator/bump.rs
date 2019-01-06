use alloc::heap::{AllocErr, Layout};

use allocator::util::*;

/// A "bump" allocator: allocates memory by bumping a pointer; never frees.
#[derive(Debug)]
pub struct Allocator {
    current: usize,
    end: usize,
}

impl Allocator {
    /// Creates a new bump allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        Allocator{
            current: start,
            end
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
        //Check align and size
        if layout.size() < 0 || layout.align() & (layout.align() - 1) !=0 || layout.align() == 0 {
            return Err(AllocErr::Unsupported{details: "Align and size are invalid"});
        }
        let act_size = align_up(layout.size(), layout.align());
        if (self.current + act_size) > self.end {
            return Err(AllocErr::Exhausted{request: layout});
        }
        let return_addr = align_up(self.current, layout.align());
        self.current = return_addr + act_size;
        Ok(return_addr as *mut u8)
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
    pub fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        if align_up(_ptr as usize, _layout.align()) != (_ptr as usize) {
            panic!("The align was not match");
        }

        if _ptr as usize > self.current || _ptr as usize > self.end {
            panic!("Out of bound");
        }
    }
}
