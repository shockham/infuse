use std::collections::HashMap;

use infuse::{RenderItem, Renderer, Uniform};
use wasm_bindgen::prelude::*;

const VERT: &str = include_str!("./shaders/vert.glsl");
const FRAG: &str = include_str!("./shaders/frag.glsl");

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut renderer = Renderer::new()?;

    // add a shader that will use the uniform
    renderer.add_shader(
        "colour".into(),
        VERT.into(),
        FRAG.into(),
    )?;

    // create the uniforms for the render item
    let mut uniforms = HashMap::new();
    uniforms.insert("colour".to_string(), Uniform::Vec4(0.5f32, 0.5f32, 0.5f32, 1f32));

    let render_item = RenderItem::new(
        vec![
            -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0,
            1.0, 0.0,
        ],
        "colour".into(),
        Some(uniforms),
    );

    let mut render_items = vec![render_item];

    render_items[0].set_uniform("colour".to_string(), Uniform::Vec4(0.6f32, 0.6f32, 0.6f32, 1f32));

    renderer.draw(&render_items)?;

    Ok(())
}
