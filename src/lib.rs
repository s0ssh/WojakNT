#![no_std]

use crate::{include::MmIsAddressValid, string::new_unicode};
use core::panic::PanicInfo;

pub mod include;
pub mod log;
pub mod string;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log!("Panicked: %s", info.message());
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {

    let sanity_check = unsafe { MmIsAddressValid(0 as _) };
    log!("Sanity Check: MmIsAddressValid(0) returned %i", sanity_check as u64);

    let string = new_unicode(obfstr::wide!("wojak in the NT kernel!\0"));
    log!("Description: UNICODE_STRING: %ws", string.Buffer);

    log!("\n\
        ⠀⠀⠀⠀⢠⣤⣤⣤⢠⣤⣤⣤⣤⣄⢀⣠⣤⣤⣄⠀⠀⠀⢀⣠⣤⣤⣄⠀⣤⣤⠀⠀⣠⣤⣤⣤⣤⣤⡄⢠⣤⣤⣤⣄⠀⠀\n\
        ⠀⠀⠀⠀⠈⢹⣿⠉⠈⠉⣿⣿⠉⠉⢾⣿⣉⣉⠙⠀⠀⢀⣾⡟⠉⠉⣿⣧⢸⣿⡄⢠⣿⠏⣿⣿⣉⣉⡁⢸⣿⡏⢉⣿⡷⠀\n\
        ⠀⠀⠀⠀⠀⢸⣿⠀⠀⠀⣿⣿⠀⠀⠈⠿⠿⣿⣿⡀⠀⠸⣿⡇⠀⠀⣾⣿⠀⢿⣿⣸⡿⠀⣿⣿⠿⠿⠇⢸⣿⣿⣿⣿⠀⠀\n\
        ⠀⠀⠀⠀⢠⣼⣿⣤⠀⠀⣿⣿⠀⠀⢷⣦⣤⣼⡿⠁⠀⠀⠹⣿⣤⣴⡿⠋⠀⠘⣿⣿⠃⠀⣿⣿⣤⣤⡄⢸⣿⡇⠙⢿⣦⡀\n\
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⠀⠀⠀⠀⠀⠀⢀⣰⣶⣶⣶⣿⣿⣿⣿⣷⣶⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⠿⠛⠛⠻⢿⣿⣿⣿⣿⣿⣿⣿⣶⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⠀⠀⠀⢀⢾⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠈⠉⠉⠉⠻⢿⢿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⠀⠀⢠⠏⢸⣿⣿⡿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠿⢻⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⠀⢀⠇⠀⠈⠿⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⢀⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⡸⠀⠀⠀⠀⠀⠀⠀⠀⡼⠛⠳⣄⡀⠀⠐⢿⣦⡀⠀⠀⠀⢠⠃⠀⣸⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⢠⠇⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠉⣳⠟⠒⠻⣿⣦⡀⠀⡘⠀⢰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⢀⠞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠃⢠⣄⡀⠈⠙⢿⡌⠁⠀⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣄⣈⢻⡿⠃⢰⠟⠲⣼⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⠀⡰⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⡶⢴⠋⠀⠀⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⡴⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠞⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⡴⢟⠒⠀⠀⠀⠀⢰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠏⠀⠀⠈⠉⣿⠇⠀⢀⡎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠠⣤⣤⣀⢰⠏⠉⠙⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣠⠴⠢⠦⠽⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⣿⣿⣿⣷⡄⣀⡀⠈⠉⠋⢹⠋⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ⠿⠿⠿⠿⠿⠦⠈⠀⠀⠀⠸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n\
        ");

    0 /* STATUS_SUCCESS */
}
