// PBR Physically-Based Rendering Shader
// Implements full Cook-Torrance BRDF with GGX distribution and Schlick geometry

// ============================================================================
// STRUCTURES
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

struct PBRMaterial {
    albedo: vec3<f32>,
    roughness: f32,
    metallic: vec3<f32>,
    metallic_factor: f32,
    ao: f32,
}

struct Light {
    position: vec3<f32>,
    _padding1: u32,
    direction: vec3<f32>,
    _padding2: u32,
    color: vec3<f32>,
    intensity: f32,
    light_type: u32,
    _padding3: vec3<u32>,
}

// ============================================================================
// BIND GROUPS
// ============================================================================

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(0) @binding(1) var<uniform> object: ObjectData;

@group(1) @binding(0) var<uniform> material: PBRMaterial;

@group(2) @binding(0) var<uniform> lights_buffer: array<Light, 16>;

// ============================================================================
// VERTEX SHADER
// ============================================================================

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Transform to world space
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
// PBR FUNCTIONS
// ============================================================================

// Fresnel-Schlick approximation
fn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {
    return f0 + (1.0 - f0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);
}

// GGX/Trowbridge-Reitz Normal Distribution Function
fn distribution_ggx(normal: vec3<f32>, half_dir: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let ndh = max(dot(normal, half_dir), 0.0);
    let ndh2 = ndh * ndh;
    
    let numerator = a2;
    let denominator = 3.14159 * pow(ndh2 * (a2 - 1.0) + 1.0, 2.0);
    
    return numerator / denominator;
}

// Schlick-GGX Geometry Sub-Function
fn geometry_schlick_ggx(normal: vec3<f32>, view_dir: vec3<f32>, roughness: f32) -> f32 {
    let r = (roughness + 1.0) * (roughness + 1.0) / 8.0;
    let ndv = max(dot(normal, view_dir), 0.0);
    
    let numerator = ndv;
    let denominator = ndv * (1.0 - r) + r;
    
    return numerator / denominator;
}

// Smith's G Function (combines geometry for view and light directions)
fn geometry_smith(normal: vec3<f32>, view_dir: vec3<f32>, light_dir: vec3<f32>, roughness: f32) -> f32 {
    let ggx2 = geometry_schlick_ggx(normal, view_dir, roughness);
    let ggx1 = geometry_schlick_ggx(normal, light_dir, roughness);
    
    return ggx1 * ggx2;
}

// Calculate PBR contribution from a single light
fn calculate_pbr_light(
    fragment_pos: vec3<f32>,
    normal: vec3<f32>,
    view_dir: vec3<f32>,
    light_pos: vec3<f32>,
    light_color: vec3<f32>,
    light_intensity: f32,
    albedo: vec3<f32>,
    roughness: f32,
    metallic: f32,
    ao: f32,
) -> vec3<f32> {
    let light_dir = normalize(light_pos - fragment_pos);
    let half_dir = normalize(view_dir + light_dir);
    
    // Distance and attenuation for point lights
    let distance = length(light_pos - fragment_pos);
    let attenuation = 1.0 / (distance * distance + 0.001);
    let radiance = light_color * light_intensity * attenuation;
    
    // Calculate vectors
    let ndl = max(dot(normal, light_dir), 0.0);
    let ndv = max(dot(normal, view_dir), 0.0);
    
    // Fresnel (F)
    var f0 = vec3<f32>(0.04);
    f0 = mix(f0, albedo, vec3<f32>(metallic));
    let f = fresnel_schlick(max(dot(half_dir, view_dir), 0.0), f0);
    
    // Cook-Torrance BRDF
    let ndf = distribution_ggx(normal, half_dir, roughness);
    let g = geometry_smith(normal, view_dir, light_dir, roughness);
    
    // kS (specular) and kD (diffuse)
    let ks = f;
    let kd = (vec3<f32>(1.0) - ks) * (1.0 - metallic);
    
    // Specular component
    let specular = (ndf * g * f) / (4.0 * ndv * ndl + 0.001);
    
    // Combine diffuse and specular
    return (kd * albedo / 3.14159 + specular) * radiance * ndl;
}

// ============================================================================
// FRAGMENT SHADER
// ============================================================================

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.world_normal);
    let view_dir = normalize(camera.position - input.world_position);
    
    // Material properties
    let albedo = material.albedo;
    let roughness = material.roughness;
    let metallic = material.metallic_factor;
    let ao = material.ao;
    
    // Ambient lighting from AO
    let ambient = albedo * ao * 0.03;
    var result = ambient;
    
    // Process all lights
    for (var i = 0u; i < 16u; i = i + 1u) {
        let light = lights_buffer[i];
        
        if (light.light_type == 0u) {  // Directional light
            // For directional lights, calculate effective position far away
            let light_pos = input.world_position + light.direction * 1000.0;
            
            result = result + calculate_pbr_light(
                input.world_position,
                normal,
                view_dir,
                light_pos,
                light.color,
                light.intensity,
                albedo,
                roughness,
                metallic,
                ao,
            );
        } else if (light.light_type == 1u) {  // Point light
            result = result + calculate_pbr_light(
                input.world_position,
                normal,
                view_dir,
                light.position,
                light.color,
                light.intensity,
                albedo,
                roughness,
                metallic,
                ao,
            );
        } else if (light.light_type == 2u) {  // Spot light
            let light_dir = normalize(light.position - input.world_position);
            let spot_angle = dot(light_dir, normalize(-light.direction));
            let cutoff = 0.7;
            
            if (spot_angle > cutoff) {
                result = result + calculate_pbr_light(
                    input.world_position,
                    normal,
                    view_dir,
                    light.position,
                    light.color,
                    light.intensity,
                    albedo,
                    roughness,
                    metallic,
                    ao,
                );
            }
        }
    }
    
    // Gamma correction (1/2.2 ≈ 0.45454)
    result = pow(result, vec3<f32>(0.45454));
    
    return vec4<f32>(result, 1.0);
}
