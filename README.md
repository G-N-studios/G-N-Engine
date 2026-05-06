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
- **Viewport**: Embedded live scene preview that visualizes entities from the editor ECS world
- **Properties Panel**: Inspect and edit entity properties
- **Asset Browser**: Browse and manage game assets

## Graphics Backend Configuration

The G&N Engine supports multiple graphics backends: **Vulkan** and **OpenGL**. You can select which backend(s) to use during build or through the launcher UI.

### Supported Backends

- **Vulkan** (recommended): High-performance, modern graphics API. Best for supported platforms.
- **OpenGL**: Widely compatible fallback for systems without Vulkan support.

### Selecting a Backend

#### Via Launcher UI
When you launch the editor with `cargo run`, the launcher UI allows you to select your preferred graphics backend before opening a scene or project.

#### Via Build Features

Build the editor with specific backend features:

```bash
# Build with both Vulkan and OpenGL (default)
cargo run --features vulkan,opengl

# Vulkan only
cargo run --features vulkan

# OpenGL only
cargo run --features opengl

# Vulkan only, without OpenGL as fallback
cargo run --no-default-features --features vulkan
```

### Vulkan SDK Setup (Windows)

To use Vulkan on Windows, you'll need the Vulkan SDK installed:

1. **Download** the Vulkan SDK from [vulkan.lunarg.com](https://vulkan.lunarg.com/)
2. **Install** to the standard location (typically `C:\VulkanSDK`)
3. **Verify Environment Variables**:
   - `VULKAN_SDK` should point to your Vulkan SDK installation directory
   - Your system `PATH` should include `<VULKAN_SDK>\Bin`
4. **Restart your terminal/IDE** after installation for environment changes to take effect

### Troubleshooting

#### Vulkan Not Available
If Vulkan is not detected or initialization fails:
- Ensure the Vulkan SDK is properly installed and environment variables are set
- Try the OpenGL backend as a fallback: `cargo run --features opengl`
- Check system logs for detailed graphics API initialization messages

#### Checking the Active Backend
The editor logs which graphics backend is active on startup. Check the console output for messages like:
- `[GRAPHICS] Initializing Vulkan backend...`
- `[GRAPHICS] Initializing OpenGL backend...`

#### Common Issues
- **"Vulkan SDK not found"**: Reinstall the SDK and ensure `VULKAN_SDK` environment variable is set
- **"VULKAN_SDK path not in PATH"**: Add `<VULKAN_SDK>\Bin` to your system's PATH and restart your terminal
- **Fallback to OpenGL**: If Vulkan fails to initialize, the engine automatically tries OpenGL if available
- **No graphics backend available**: Ensure at least one backend is enabled via build features

## Project Structure

- `src/` - Main library and re-exports
- `crates/gn-core` - Core engine functionality (ECS, components)
- `crates/gn-render` - Rendering subsystem
- `crates/gn-scripting` - Scripting engine (Lua integration)
- `crates/gn-editor` - Iced-based editor GUI
