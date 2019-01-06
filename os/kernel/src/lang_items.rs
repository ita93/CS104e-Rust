#[no_mangle]
#[cfg(not(test))]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: ::std::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
    // FIXME: Print `fmt`, `file`, and `line` to the console.
    use console::kprint;
    kprint!("PANIC \n");
    kprint!("ALERT \n");
    kprint!("{}:{}\n", file, line);
    kprint!("{}\n", fmt);
    loop { unsafe { asm!("wfe") } }
}

#[cfg(not(test))] #[lang = "eh_personality"] pub extern fn eh_personality() {}
