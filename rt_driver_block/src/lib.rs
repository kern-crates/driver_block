#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    unimplemented!("");
}

pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
