use core::arch::asm;

#[naked]
#[link_section = ".fw_fdt"]
pub unsafe extern "C" fn raw_fdt() {
    asm!(
        concat!(".incbin \"", env!("PROTOTYPER_FDT"), "\""),
        options(noreturn)
    );
}

#[naked]
#[link_section = ".fw_payload"]
pub unsafe extern "C" fn fw_payload_image() {
    asm!(
        concat!(".incbin \"", env!("PROTOTYPER_IMAGE"), "\""),
        options(noreturn)
    );
}

pub fn get_fdt_address() -> usize {
    let addr: usize;
    unsafe {
        asm!("la {}, {fdt}", out(reg) addr, fdt = sym raw_fdt, options(nomem));
    }
    addr
}

pub fn get_image_address() -> usize {
    let addr: usize;
    unsafe {
        asm!("la {}, {image}", out(reg) addr, image = sym fw_payload_image, options(nomem));
    }
    addr
}
