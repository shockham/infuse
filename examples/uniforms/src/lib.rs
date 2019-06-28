use std::collections::HashMap;

use infuse::{RenderItem, Renderer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut renderer = Renderer::new()?;

    // add a shader that will use the uniform
    renderer.add_shader(
        "colour".into(),
        "
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
        "
        .into(),
        "
        precision mediump float;

        uniform vec4 colour;

        void main() {
            gl_FragColor = colour;
        }
        "
        .into(),
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
