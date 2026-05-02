// Blinn-Phong lighting shader for basic rendering
// Supports directional, point, and spot lights with per-fragment lighting

// ============================================================================
// Vertex Shader
// ============================================================================

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct CameraUniform {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    view_proj_matrix: mat4x4<f32>,
    position: vec3<f32>,
}

struct ObjectData {
    model_matrix: mat4x4<f32>,
    normal_matrix: mat3x3<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(0) @binding(1) var<uniform> object: ObjectData;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Transform position to world space
    let world_pos = object.model_matrix * vec4<f32>(input.position, 1.0);
    output.world_position = world_pos.xyz;
    
    // Transform normal to world space using normal matrix
    output.world_normal = normalize(object.normal_matrix * input.normal);
    
    // Project to clip space
    output.position = camera.view_proj_matrix * world_pos;
    
    // Pass through UV coordinates
    output.uv = input.uv;
    
    return output;
}

// ============================================================================
// Fragment Shader
// ============================================================================

struct Material {
    color: vec3<f32>,
    ambient: f32,
    specular: vec3<f32>,
    shininess: f32,
}

struct Light {
    position: vec3<f32>,
    _padding1: u32,
    direction: vec3<f32>,
    _padding2: u32,
    color: vec3<f32>,
    intensity: f32,
    light_type: u32,
    _padding3: u32,
}

@group(1) @binding(0) var<uniform> material: Material;

@group(2) @binding(0) var<uniform> lights_buffer: array<Light, 16>;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.world_normal);
    let view_dir = normalize(camera.position - input.world_position);
    
    // Initialize result with ambient component
    var result = material.color * material.ambient;
    
    // Process all lights
    for (var i: u32 = 0u; i < 16u; i = i + 1u) {
        let light = lights_buffer[i];
        
        // Skip inactive lights (zero intensity)
        if (light.intensity <= 0.0) {
            continue;
        }
        
        var light_dir: vec3<f32>;
        var attenuation: f32 = 1.0;
        
        if (light.light_type == 0u) {
            // Directional light - use negative direction
            light_dir = normalize(-light.direction);
        } else if (light.light_type == 1u) {
            // Point light - calculate direction from light to surface
            let light_vec = light.position - input.world_position;
            let distance = length(light_vec);
            light_dir = normalize(light_vec);
            
            // Inverse square law for attenuation
            attenuation = 1.0 / (distance * distance + 0.001);
        } else if (light.light_type == 2u) {
            // Spot light - similar to point light but with angular falloff
            let light_vec = light.position - input.world_position;
            let distance = length(light_vec);
            light_dir = normalize(light_vec);
            
            // Inverse square law for attenuation
            attenuation = 1.0 / (distance * distance + 0.001);
            
            // Simple angular falloff (would need actual cone angle for proper spot light)
            let spot_dot = dot(light_dir, normalize(-light.direction));
            if (spot_dot < 0.5) {
                attenuation = 0.0;
            }
        } else {
            continue;
        }
        
        // Blinn-Phong lighting model
        // Diffuse component
        let diff = max(dot(normal, light_dir), 0.0);
        let diffuse = diff * material.color * light.color * light.intensity * attenuation;
        
        // Specular component (Blinn-Phong uses half vector)
        let half_dir = normalize(light_dir + view_dir);
        let spec = pow(max(dot(normal, half_dir), 0.0), material.shininess);
        let specular = spec * material.specular * light.color * light.intensity * attenuation;
        
        result = result + diffuse + specular;
    }
    
    return vec4<f32>(result, 1.0);
}
