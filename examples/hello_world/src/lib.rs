use wasm_bindgen::prelude::*;
use infuse::{RenderItem, Renderer};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let renderer = Renderer::new()?;

    let render_item = RenderItem::new(
        vec![
            -1.0, -1.0, 0.0,
            1.0, -1.0, 0.0,
            1.0, 1.0, 0.0,
            -1.0, 1.0, 0.0,
            -1.0, -1.0, 0.0,
            1.0, 1.0, 0.0,
        ],
        "default".into(),
    );

    let render_items = vec![render_item];

    renderer.draw(render_items)?;

    Ok(())
}
