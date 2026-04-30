//! G&N Engine - Main library re-export
//! 
//! The Grit and Nails Engine is a 3D game engine written in Rust.
//! This crate re-exports the public API from all engine components.

pub use gn_core;
pub use gn_render;
pub use gn_scripting;
pub use gn_editor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
