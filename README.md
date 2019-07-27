# infuse
[![crates.io version](https://img.shields.io/crates/v/infuse.svg)](https://crates.io/crates/infuse)
[![Build status](https://travis-ci.org/shockham/infuse.svg?branch=master)](https://travis-ci.org/shockham/infuse)
[![Documentation](https://docs.rs/infuse/badge.svg)](https://docs.rs/infuse)

Minimalist wasm based webgl renderer.

### Example usage:
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

### Project creation:

This can be done easily using [cargo-generate](https://github.com/ashleygwilliams/cargo-generate).
- Install cargo-generate with:
```
cargo install cargo-generate
```
- Create a new project with:
```
cargo generate --git https://github.com/shockham/infuse-template.git
```
- Install the deps:
```
yarn install
```
- Serve the app:
```
yarn run serve
```

[License](https://github.com/shockham/infuse/blob/master/LICENSE.md)
