// Basic shader for spinning cube demo

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let world_pos = uniforms.model * vec4<f32>(input.position, 1.0);
    let clip_pos = uniforms.view_proj * world_pos;
    
    // Simple color based on position for visual interest
    let color = (input.position + vec3<f32>(1.0)) * 0.5;
    
    return VertexOutput(
        clip_pos,
        color,
    );
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
