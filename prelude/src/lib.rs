pub use prelude_2021::*;

pub fn init() {
    console_error_panic_hook::set_once();
    console_log::init().unwrap();
}
