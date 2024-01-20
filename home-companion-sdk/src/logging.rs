#[link(wasm_import_module = "logging")]
extern "C" {
    /// Log message with informational level.
    pub fn info(message: &str);
}
