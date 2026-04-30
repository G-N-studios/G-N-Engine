//! G&N Engine Editor
//! 
//! Iced-based editor and tools for the game engine

pub mod launcher;
pub mod scene_tree;
pub mod viewport;
pub mod properties;
pub mod asset_browser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
