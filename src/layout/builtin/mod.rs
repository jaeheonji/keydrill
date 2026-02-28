mod colemak;

pub use colemak::builtin_colemak;

use super::types::Layout;

pub fn all() -> Vec<Layout> {
    vec![builtin_colemak()]
}
