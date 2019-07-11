use cgmath::Deg;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

use camera::Camera;

use crate::console_log;

pub mod camera;
pub mod renderer;
pub mod model;

pub struct GraphicContext {
    pub gl: WebGl2RenderingContext,
    pub canvas: HtmlCanvasElement,
    program: WebGlProgram,
    position_loc: i32,
    world_to_screen_loc: WebGlUniformLocation,  // view * proj
    model_to_world_loc: WebGlUniformLocation,  // model
    camera: Camera,
}

const VERTEX_SHADER_SRC: &str = r#"#version 300 es
in vec3 position;

uniform mat4 world_to_screen;
uniform mat4 model_to_world;

out vec4 color;

void main() {
    // Multiply the position by the matrix.
    gl_Position = world_to_screen * model_to_world * vec4(position, 1);

    // Convert from clipspace to colorspace.
    // Clipspace goes -1.0 to +1.0
    // Colorspace goes from 0.0 to 1.0
    color = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"#version 300 es
precision mediump float;

in vec4 color;

out vec4 outColor;

void main() {
    outColor = color;
}
"#;


impl GraphicContext {
    pub fn from_canvas(id: &str) -> Result<GraphicContext, JsValue> {
        let document = web_sys::window().unwrap().document().expect("failed to find document");
        let canvas = document.get_element_by_id(id).expect("failed to find canvas");
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;


        let gl = canvas
            .get_context("webgl2")?
            .expect("Cannot find webgl2")
            .dyn_into::<WebGl2RenderingContext>()?;


        let vert_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            VERTEX_SHADER_SRC)?;

        let frag_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER_SRC)?;

        let program = link_program(&gl, &vert_shader, &frag_shader)?;
        gl.use_program(Some(&program));

        let position_loc = gl.get_attrib_location(&program, "position");
        let world_to_screen_loc = gl.get_uniform_location(&program, "world_to_screen").expect("Cannot find uniform world_to_screen");
        let model_to_world_loc = gl.get_uniform_location(&program, "model_to_world").expect("Cannot find uniform model_to_world");

        let w = canvas.width();
        let h = canvas.height();

        let mut g = GraphicContext {
            gl,
            canvas,
            program,
            position_loc,
            world_to_screen_loc,
            model_to_world_loc,
            camera: Camera::new(
                Deg(45.0),
                w as f32 / h as f32,
            )
        };

        //g.on_resize(g.canvas.width(), g.canvas.height());

        Ok(g)
    }

    pub fn on_resize(&mut self, width: u32, height: u32) {
        console_log!("Resized! {} {}", width, height);

        self.canvas.set_width(width);
        self.canvas.set_height(height);

        self.camera.aspect_ratio = width as f32 / height as f32;
        self.camera.rebuild_projection();

        self.gl.viewport(0, 0, width as i32, height as i32);
    }
}

pub fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .expect("Unable to create shader");
        //.ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) {
        Ok(shader)
    } else {
        let description = gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("unknown error"));
        Err(format!(
            "cannot compile {} shader, {}",
            if shader_type == WebGl2RenderingContext::VERTEX_SHADER { "vertex" } else { "fragment" },
            description
        ))
    }
}

pub fn link_program(
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
        .unwrap_or(false) {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn setup() -> Result<GraphicContext, JsValue> {
    let ctx = GraphicContext::from_canvas("canvas")?;

    Ok(ctx)
}