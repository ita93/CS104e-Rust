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

pub fn blink(repeat: u8, interval: u64) {
    use pi::timer::spin_sleep_ms;

    let mut gpio16 = pi::gpio::Gpio::new(16).into_output();
    for _ in 0..repeat {
       gpio16.set();
       spin_sleep_ms(interval);
       gpio16.clear();
       spin_sleep_ms(interval);
    }
}

pub fn echo() {
    loop {
        blink(1,10);
        /*use std::fmt::Write;
        let mut console = console::CONSOLE.lock();
        console.write_fmt(format_args!("Hello world\n")).unwrap();
        */
        /*console::CONSOLE.lock().write_byte(b'>');
        loop {
            let byte = console::CONSOLE.lock().read_byte();
            console::CONSOLE.lock().write_byte(byte);
            if byte == b'\n' || byte == b'\r' {
                console::CONSOLE.lock().write_byte(b'\r');
                console::CONSOLE.lock().write_byte(b'\n');
                break;
            }
            blink(1, 10);
        }*/
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    shell::shell(">");
}

