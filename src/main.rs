#![no_main]
#![no_std]

extern crate teensy;

use teensy::ports::*;

#[no_mangle]
pub extern fn main() {
    unsafe {
        let port = Port::reg(PortName::C);
        port.set_pcr(5, 1<<8);
        let gpio = Gpio::reg(PortName::C);
        gpio.set_pddr(1<<5);
        loop {
            for _ in 0..100000 {
                gpio.set_psor(1<<5);
            }
            for _ in 0..100000 {
                gpio.set_pcor(1<<5);
            }
        }
    }
}
