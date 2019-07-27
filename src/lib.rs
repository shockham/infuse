use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlContextAttributes};

use std::collections::HashMap;

pub mod macros;

pub struct RenderItem {
    vertices: Vec<f32>,
    shader_name: String,
    uniforms: Option<HashMap<String, (f32, f32, f32, f32)>>,
}

impl RenderItem {
    pub fn new(
        vertices: Vec<f32>,
        shader_name: String,
        uniforms: Option<HashMap<String, (f32, f32, f32, f32)>>,
    ) -> RenderItem {
        RenderItem {
            vertices,
            shader_name,
            uniforms,
        }
    }
}

impl RenderItem {
    pub fn set_uniform(&mut self, name: String, value: (f32, f32, f32, f32)) {
        if let Some(uniforms) = self.uniforms.as_mut() {
            uniforms.insert(name, value);
        }
    }
}

pub struct Renderer {
    context: WebGl2RenderingContext,
    shaders: HashMap<String, WebGlProgram>,
}

impl Renderer {
    pub fn new() -> Result<Renderer, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = canvas
            .get_context_with_context_options(
                "webgl2",
                WebGlContextAttributes::new()
                    .antialias(true)
                    .preserve_drawing_buffer(true)
                    .as_ref()
            )?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        let shaders = HashMap::<String, WebGlProgram>::new();

        let mut renderer = Renderer { context, shaders };

        renderer.add_shader(
            "default".into(),
            "
            attribute vec4 position;
            void main() {
                gl_Position = position;
            }
            "
            .into(),
            "
            void main() {
                gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
            }
            "
            .into(),
        )?;

        Ok(renderer)
    }

    pub fn draw(&self, render_items: &Vec<RenderItem>) -> Result<(), JsValue> {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);

        for render_item in render_items {
            let shader_name = &render_item.shader_name;
            let program = self.shaders.get(shader_name.as_str()).unwrap();
            self.context.use_program(Some(&program));

            if let Some(uni) = &render_item.uniforms {
                for (key, value) in uni.iter() {
                    let location = self.context.get_uniform_location(program, key.as_str());
                    self.context
                        .uniform4f(location.as_ref(), value.0, value.1, value.2, value.3);
                }
            }

            let vertices = &render_item.vertices;
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()?
                .buffer();
            let vertices_location = vertices.as_ptr() as u32 / 4;
            let vert_array = js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices.len() as u32);

            let buffer = self
                .context
                .create_buffer()
                .ok_or("failed to create buffer")?;
            self.context
                .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
            self.context.vertex_attrib_pointer_with_i32(
                0,
                3,
                WebGl2RenderingContext::FLOAT,
                false,
                0,
                0,
            );
            self.context.enable_vertex_attrib_array(0);

            self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

            self.context.draw_arrays(
                WebGl2RenderingContext::TRIANGLES,
                0,
                (vertices.len() / 3) as i32,
            );
        }

        Ok(())
    }

    pub fn add_shader(
        &mut self,
        name: String,
        vertex_shader: String,
        fragment_shader: String,
    ) -> Result<(), String> {
        let vert_shader = Self::compile_shader(
            &self.context,
            WebGl2RenderingContext::VERTEX_SHADER,
            vertex_shader.as_str(),
        )?;
        let frag_shader = Self::compile_shader(
            &self.context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            fragment_shader.as_str(),
        )?;
        let program = Self::link_program(&self.context, &vert_shader, &frag_shader)?;

        self.shaders.insert(name.into(), program);

        Ok(())
    }

    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(
        context: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
