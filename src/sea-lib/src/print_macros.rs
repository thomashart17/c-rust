use core::fmt::{self, Arguments};

pub struct NullWriter;

impl NullWriter {
    pub fn write_fmt(&mut self, _: Arguments<'_>) -> fmt::Result { Ok(()) }
}

custom_print::define_macros!(
    #[macro_export]
    {print, println, eprint, eprintln},
    sea::print_macros::NullWriter
);
