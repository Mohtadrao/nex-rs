//! Byte stream settings analogous to Go's ByteStreamSettings.
#[derive(Debug, Clone)]
pub struct ByteStreamSettings {
    pub string_length_size: usize,
    pub pid_size: usize,
    pub use_structure_header: bool,
}

impl Default for ByteStreamSettings {
    fn default() -> Self {
        Self {
            string_length_size: 2,
            pid_size: 4,
            use_structure_header: false,
        }
    }
}
