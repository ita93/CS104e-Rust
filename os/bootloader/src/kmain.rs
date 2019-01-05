#![feature(asm, lang_items)]
#![feature(decl_macro)]

extern crate xmodem;
extern crate pi;

pub mod lang_items;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

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

/// Branches to the address `addr` unconditionally.
fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        asm!("br $0" : : "r"(addr as usize));
        loop { asm!("nop" :::: "volatile")  }
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    let mut led = pi::gpio::Gpio::new(16).into_output();
    let mut led_on = false;
    let mut uart = pi::uart::MiniUart::new();
    uart.set_read_timeout(750);
    let mut buf = unsafe {
        std::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE)
    };

    loop {
        if led_on { led.clear() } else { led.set() }
        led_on = !led_on;

        match xmodem::Xmodem::receive(&mut uart, &mut buf) {
            Ok(_) => {
                led.clear();
                blink(3, 300);
                break;
            }
            Err(_) => {
            }
        }
    
    }
    jump_to(BINARY_START);
}

