// C function signature:
// #[no_mangle]
// pub extern "C" fn _____(...) -> ... {}
// #![feature(lang_items)]

// **************************
// Global Allocator is automatically read by std
//
// When using no_std, this lib must be used to
// have an Allocator, panic handler, and error handler 
// **************************

#![no_std]


use core::fmt::{self, Arguments};

pub struct NullWriter;

impl NullWriter {
    pub fn write_fmt(&mut self, _: Arguments<'_>) -> fmt::Result { Ok(()) }
}

#[macro_export]
macro_rules! define_custom_print {
    () => {
        custom_print::define_macros!(
            {cprint, cprintln, ceprint, ceprintln},
            NullWriter
        );

        use cprint as print;
        use cprintln as println;
        use ceprint as eprint;
        use ceprintln as eprintln;
    };
}
