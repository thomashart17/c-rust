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
            sea::print_macros::NullWriter
        );

        use cprint as print;
        use cprintln as println;
        use ceprint as eprint;
        use ceprintln as eprintln;
    };
}
