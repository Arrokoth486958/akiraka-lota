// 顶点着色器

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
    @location(2) tex_coords: vec2f,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) tex_coords: vec2f,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4f(model.position, 1.0);
    return out;
}

// 片元着色器

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    var color = textureSample(t_diffuse, s_diffuse, vec2f(in.tex_coords.x, 1.0 - in.tex_coords.y));
    return color;
}