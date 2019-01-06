/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align != 0 && ( align & (align - 1) == 0) {
        //align is power of 2
        let mut res:usize = addr / align;
        res * align
    } else {
        panic!("align must be power of 2");
    }
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    if align != 0 && ( align & (align - 1) == 0) {
        //align is power of 2
        let mut res:usize = addr / align;
        let remainer = addr % align;
        if remainer > 0 {
            (res + 1) * align
        } else {
            res * align
        }
    } else {
        panic!("align must be power of 2");
    }
}
