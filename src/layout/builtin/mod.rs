mod colemak;
mod colemak_dh;

pub use colemak::builtin_colemak;
pub use colemak_dh::builtin_colemak_dh;

use super::types::Layout;

pub fn all() -> Vec<Layout> {
    vec![builtin_colemak(), builtin_colemak_dh()]
}
