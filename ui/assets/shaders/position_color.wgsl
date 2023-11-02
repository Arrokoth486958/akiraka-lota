// 顶点着色器

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
    @location(2) uv: vec3f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec3f,
};

@group(0) @binding(0)
var<uniform> surface_size: vec3f;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let flipped_y = 1.0 - model.position.y;
    let x = (model.position.x + 1.0) * 0.5;
    let y = (flipped_y + 1.0) * 0.5;
    out.color = model.color;
    out.clip_position = (vec4f(model.position, 1.0) / vec4f(surface_size, 1.0) * vec4f(1.0, -1.0, 1.0, 1.0) - vec4f(0.5, -0.5, 0.0, 0.0)) / vec4f(0.5, 0.5, 1.0, 1.0);
    return out;
}

// 片元着色器

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(in.color, 1.0);
}