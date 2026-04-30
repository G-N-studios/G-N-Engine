//! G&N Engine Scripting
//! 
//! Lua scripting integration for entity behavior and game logic

pub mod script_engine;

pub use script_engine::ScriptEngine;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
