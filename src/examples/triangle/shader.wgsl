struct AppState {
    color: vec4<f32>,
}

@group(0)
@binding(0)
var<uniform> app_state: AppState;

@vertex
fn vs_main(@builtin(vertex_index) v_index: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2(0.0, 0.5),
        vec2(-0.5, -0.5),
        vec2(0.5, -0.5)
    );
    return vec4<f32>(pos[v_index], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return app_state.color;
}
