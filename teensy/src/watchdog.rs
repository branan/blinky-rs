use core;

#[repr(C,packed)]
pub struct Wdog {
    stctrlh: u16,
    stctrll: u16,
    tovalh: u16,
    tovall: u16,
    winh: u16,
    winl: u16,
    refresh: u16,
    unlock: u16,
    tmrouth: u16,
    tmroutl: u16,
    rstcnt: u16,
    presc: u16
}

impl Wdog {
    pub unsafe fn reg() -> &'static mut Wdog {
        &mut *(0x40052000 as *mut Wdog)
    }

    pub unsafe fn unlock(&mut self) {
        core::intrinsics::volatile_store(&mut self.unlock, 0xC520);
        core::intrinsics::volatile_store(&mut self.unlock, 0xD928);
    }

    pub unsafe fn disable(&mut self) {
        self.unlock();
        let mut ctrl = core::intrinsics::volatile_load(&mut self.stctrlh);
        ctrl &= !(0x1);
        core::intrinsics::volatile_store(&mut self.stctrlh, ctrl);
    }
}
