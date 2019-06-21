use lazy_static::lazy_static;
use euclid::{Transform3D, Vector3D, Angle};
use wasm_bindgen::{JsValue, JsCast};
use js_sys::WebAssembly;
use web_sys::{WebGl2RenderingContext as WebGL, WebGlBuffer};

use crate::shader_program::ShaderProgram;

pub struct Cube {
    gl: WebGL,
    vertex_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    program: ShaderProgram,
}

lazy_static! {
    static ref VERTEX_VEC: Vec<f32> = vec![
        // front
        -1.0, -1.0,  1.0,
         1.0, -1.0,  1.0,
         1.0,  1.0,  1.0,
        -1.0,  1.0,  1.0,
        // back
        -1.0, -1.0, -1.0,
         1.0, -1.0, -1.0,
         1.0,  1.0, -1.0,
        -1.0,  1.0, -1.0,
    ];
    static ref INDEX_VEC: Vec<u16> = vec![
        // front
        0, 1, 2,
        2, 3, 0,
        // right
        1, 5, 6,
        6, 2, 1,
        // back
        7, 6, 5,
        5, 4, 7,
        // left
        4, 0, 3,
        3, 7, 4,
        // bottom
        4, 5, 1,
        1, 0, 4,
        // top
        3, 2, 6,
        6, 7, 3,
    ];
}

impl Cube {
    pub fn new(gl: &WebGL) -> Result<Self, JsValue> {
        let vertex_buffer = gl.create_buffer().ok_or_else(|| js_sys::Error::new("WebGL is out of memory"))?;
        let index_buffer = gl.create_buffer().ok_or_else(|| js_sys::Error::new("WebGL is out of memory"))?;
        gl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.bind_buffer(WebGL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        let memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>()?.buffer();
        let vertices_location = VERTEX_VEC.as_ptr() as u32 / 4;
        let vertex_array = js_sys::Float32Array::new(&memory_buffer).subarray(vertices_location, vertices_location + VERTEX_VEC.len() as u32);
        gl.buffer_data_with_array_buffer_view(WebGL::ARRAY_BUFFER, &vertex_array, WebGL::STATIC_DRAW);

        let indices_location = INDEX_VEC.as_ptr() as u32 / 2;
        let index_array = js_sys::Uint16Array::new(&memory_buffer).subarray(indices_location, indices_location + INDEX_VEC.len() as u32);
        gl.buffer_data_with_array_buffer_view(WebGL::ELEMENT_ARRAY_BUFFER, &index_array, WebGL::STATIC_DRAW);

        Ok(Self {
            gl: gl.clone(),
            vertex_buffer,
            index_buffer,
            program: ShaderProgram::new(&gl, include_str!("shaders/cube.vs"), include_str!("shaders/cube.fs"), &["vertexPosition"], &["vertexTransform"])?,
        })
    }

    pub fn render(&self, time: f64) {
        let gl = &self.gl;
        gl.enable(WebGL::DEPTH_TEST);
        gl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.bind_buffer(WebGL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        self.program.use_program();

        let vertex_attribute_index = self.program.attribute("vertexPosition").unwrap();
        gl.vertex_attrib_pointer_with_i32(vertex_attribute_index, 3, WebGL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(vertex_attribute_index);

        let transform = Transform3D::create_perspective(50.0).pre_translate(Vector3D::new(0.0, 0.0, 1.0)).pre_scale(0.5, 0.5, 0.5).pre_rotate(1.0, 0.0, 0.0, Angle::radians((time / 1000.0) as f32)).pre_rotate(0.0, 1.0, 0.0, Angle::radians((time / 1000.0) as f32)).pre_rotate(0.0, 0.0, 1.0, Angle::radians((time / 1000.0) as f32));

        gl.uniform_matrix4fv_with_f32_array(self.program.uniform("vertexTransform"), false, &transform.to_column_major_array());

        gl.draw_elements_with_i32(WebGL::TRIANGLES, INDEX_VEC.len() as i32, WebGL::UNSIGNED_SHORT, 0);
    }
}

impl Drop for Cube {
    fn drop(&mut self) {
        self.gl.delete_buffer(Some(&self.vertex_buffer));
        self.gl.delete_buffer(Some(&self.index_buffer));
    }
}