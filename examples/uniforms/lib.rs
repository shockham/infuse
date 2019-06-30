use std::collections::HashMap;

use infuse::{RenderItem, Renderer};
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
    uniforms.insert("colour".to_string(), (0.5f32, 0.5f32, 0.5f32, 1f32));

    let render_item = RenderItem::new(
        vec![
            -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0,
            1.0, 0.0,
        ],
        "colour".into(),
        Some(uniforms),
    );

    let render_items = vec![render_item];

    renderer.draw(render_items)?;

    Ok(())
}
