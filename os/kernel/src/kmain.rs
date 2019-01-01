#![feature(compiler_builtins_lib, pointer_methods)]
#![no_builtins]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]

extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

extern crate compiler_builtins;

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut gpio16_output = pi::gpio::Gpio::new(16).into_output();

    // Continuously set and clear GPIO 16.
    loop {
        gpio16_output.set();
        pi::timer::spin_sleep_ms(100);
        gpio16_output.clear();
        pi::timer::spin_sleep_ms(100);
    }
}

