//! Lua script engine integration
//!
//! Embeds Lua for entity scripting and behavior

use mlua::{Lua, Value as LuaValue};
use std::fs;
use std::path::Path;

/// Lua script engine for game logic
pub struct ScriptEngine {
    lua: Lua,
}

impl ScriptEngine {
    /// Create a new Lua script engine
    pub fn new() -> mlua::Result<Self> {
        let lua = Lua::new();
        Ok(ScriptEngine { lua })
    }

    /// Load and execute a Lua script from a file
    pub fn load_script_file(&self, path: &Path) -> mlua::Result<()> {
        let source = fs::read_to_string(path)
            .map_err(|e| mlua::Error::external(format!("Failed to read script: {}", e)))?;
        self.lua.load(&source).eval::<()>()?;
        Ok(())
    }

    /// Load and execute Lua code from a string
    pub fn load_script(&self, code: &str) -> mlua::Result<()> {
        self.lua.load(code).eval::<()>()?;
        Ok(())
    }

    /// Call a Lua function by name with no arguments
    pub fn call_function(&self, name: &str) -> mlua::Result<()> {
        let globals = self.lua.globals();
        let func: mlua::Function = globals.get(name)?;
        func.call::<_, ()>(())?;
        Ok(())
    }

    /// Get a global variable from Lua
    pub fn get_global(&self, name: &str) -> mlua::Result<LuaValue> {
        let globals = self.lua.globals();
        globals.get(name)
    }

    /// Set a global integer
    pub fn set_global_int(&self, name: &str, value: i64) -> mlua::Result<()> {
        let globals = self.lua.globals();
        globals.set(name, value)?;
        Ok(())
    }

    /// Set a global string
    pub fn set_global_string(&self, name: &str, value: &str) -> mlua::Result<()> {
        let globals = self.lua.globals();
        globals.set(name, value)?;
        Ok(())
    }

    /// Create a Lua table
    pub fn create_table(&self) -> mlua::Result<mlua::Table<'_>> {
        self.lua.create_table()
    }

    /// Get the Lua instance for advanced operations
    pub fn lua(&self) -> &Lua {
        &self.lua
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create Lua context")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_engine_creation() {
        let engine = ScriptEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_load_script() {
        let engine = ScriptEngine::new().unwrap();
        let result = engine.load_script("x = 42");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_global() {
        let engine = ScriptEngine::new().unwrap();
        engine.load_script("test_var = 123").unwrap();
        let result = engine.get_global("test_var").unwrap();

        match result {
            LuaValue::Integer(i) => assert_eq!(i, 123),
            _ => panic!("Expected integer"),
        }
    }

    #[test]
    fn test_call_function() {
        let engine = ScriptEngine::new().unwrap();
        engine
            .load_script("function greet() return 'hello' end")
            .unwrap();

        // Call function
        let result = engine.call_function("greet");
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_global_int() {
        let engine = ScriptEngine::new().unwrap();
        let result = engine.set_global_int("my_value", 99);
        assert!(result.is_ok());

        let retrieved = engine.get_global("my_value").unwrap();
        match retrieved {
            LuaValue::Integer(i) => assert_eq!(i, 99),
            _ => panic!("Expected integer"),
        }
    }

    #[test]
    fn test_set_global_string() {
        let engine = ScriptEngine::new().unwrap();
        let result = engine.set_global_string("greeting", "hello");
        assert!(result.is_ok());

        let retrieved = engine.get_global("greeting").unwrap();
        match retrieved {
            LuaValue::String(s) => {
                let s_str = s.to_str().unwrap();
                assert_eq!(s_str, "hello");
            }
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_create_table() {
        let engine = ScriptEngine::new().unwrap();
        let table = engine.create_table();
        assert!(table.is_ok());
    }
}
