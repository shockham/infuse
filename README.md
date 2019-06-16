# infuse

Minimalist wasm based webgl renderer.

Example usage:
```rust
use wasm_bindgen::prelude::*;

mod renderer;
use renderer::{RenderItem, Renderer};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let renderer = Renderer::new()?;

    let render_item = RenderItem::new(
        vec![-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0],
        "default".into(),
    );

    let render_items = vec![render_item];

    renderer.draw(render_items)?;

    Ok(())
}
```
