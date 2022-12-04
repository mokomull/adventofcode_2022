pub use prelude_2021::*;

pub extern crate log;

pub fn init() {
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    let _ = console_log::init_with_level(log::Level::Debug);

    #[cfg(not(debug_assertions))]
    let _ = console_log::init();
}
