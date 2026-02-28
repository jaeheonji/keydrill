mod builtin;
mod types;

pub use types::{Layout, Level};

// Used by tests in other modules.
#[allow(unused_imports)]
pub use builtin::builtin_colemak;

impl Layout {
    pub fn discover_all() -> Vec<Self> {
        builtin::all()
    }
}
