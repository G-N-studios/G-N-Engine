//! G&N Engine Editor
//!
//! Iced-based editor and tools for the game engine

pub mod asset_browser;
pub mod launcher;
pub mod properties;
pub mod scene_tree;
pub mod viewport;
pub mod viewport_renderer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
