# G&N Engine

The Grit and Nails Engine is a 3D game engine written in Rust, featuring an integrated Iced-based editor with scene management, asset browsing, and viewport rendering.

## Running the Editor

To launch the G&N Engine editor GUI, run:

```bash
cargo run
```

This will start the editor with an interactive launcher where you can:
- Select demo scenes (rotating cube, full editor demo)
- Create new projects
- Open existing projects
- Manage your scenes and assets

The editor interface includes:
- **Scene Tree**: View and manage entities in your scene
- **Viewport**: 3D view of your scene (placeholder rendering)
- **Properties Panel**: Inspect and edit entity properties
- **Asset Browser**: Browse and manage game assets

## Project Structure

- `src/` - Main library and re-exports
- `crates/gn-core` - Core engine functionality (ECS, components)
- `crates/gn-render` - Rendering subsystem
- `crates/gn-scripting` - Scripting engine (Lua integration)
- `crates/gn-editor` - Iced-based editor GUI
