use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};
use crate::graphics::GraphicContext;

#[derive(Clone)]
pub struct RenderModel {
    pub vao: WebGlVertexArrayObject,
    pub triangle_count: u32,
}

impl RenderModel {
    pub fn new(ctx: &GraphicContext, vertices: &[f32]) -> RenderModel {
        let vao: WebGlVertexArrayObject = ctx.gl.create_vertex_array().expect("failed to create VAO");
        ctx.gl.bind_vertex_array(Some(&vao));

        let buffer = ctx.gl.create_buffer().expect("failed to create buffer");
        ctx.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let vert_array = js_sys::Float32Array::view(vertices);

            ctx.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        ctx.gl.enable_vertex_attrib_array(ctx.position_loc as u32);
        ctx.gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);

        RenderModel {
            vao,
            triangle_count: (vertices.len() / 3) as u32
        }
    }
}
