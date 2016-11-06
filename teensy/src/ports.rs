use core;

pub enum PortName {
    A,
    B,
    C,
    D,
    E
}

#[repr(C,packed)]
pub struct Port {
    pcr: [u32; 32],
    gplcr: u32,
    gpchr: u32,
    reserved_0: [u8; 24],
    isfr: u32,
    reserved_1: [u8; 28],
    dfer: u32,
    dfcr: u32,
    dfwr: u32
}

impl Port {
    pub unsafe fn reg(name: PortName) -> &'static mut Port {
        &mut * match name {
            PortName::A => 0x40049000 as *mut Port,
            PortName::B => 0x4004A000 as *mut Port,
            PortName::C => 0x4004B000 as *mut Port,
            PortName::D => 0x4004C000 as *mut Port,
            PortName::E => 0x4004D000 as *mut Port
        }
    }

    pub unsafe fn set_pcr(&mut self, pin: u8, val: u32) {
        core::intrinsics::volatile_store(&mut self.pcr[pin as usize], val);
    }
}

#[repr(C,packed)]
pub struct Gpio {
    pdor: u32,
    psor: u32,
    pcor: u32,
    ptor: u32,
    pdir: u32,
    pddr: u32
}

impl Gpio {
    pub unsafe fn reg(name:PortName) -> &'static mut Gpio {
        &mut * match name {
            PortName::A => 0x400FF000 as *mut Gpio,
            PortName::B => 0x400FF040 as *mut Gpio,
            PortName::C => 0x400FF080 as *mut Gpio,
            PortName::D => 0x400FF0C0 as *mut Gpio,
            PortName::E => 0x400FF100 as *mut Gpio
        }
    }

    pub unsafe fn set_pddr(&mut self, val: u32) {
        core::intrinsics::volatile_store(&mut self.pddr, val);
    }

    pub unsafe fn set_psor(&mut self, val: u32) {
        core::intrinsics::volatile_store(&mut self.psor, val);
    }

    pub unsafe fn set_pcor(&mut self, val: u32) {
        core::intrinsics::volatile_store(&mut self.pcor, val);
    }
}
