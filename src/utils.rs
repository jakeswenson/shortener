pub fn set_panic_hook() {
  // https://github.com/rustwasm/console_error_panic_hook#readme

  #[cfg(feature = "debug")]
  {
    pub use console_error_panic_hook::set_once;
    set_once();
  }
}
