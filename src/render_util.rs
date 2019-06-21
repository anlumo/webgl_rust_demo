use wasm_bindgen::{prelude::*};
use web_sys::{WebGl2RenderingContext as WebGL, WebGlShader, WebGlProgram, WebGlUniformLocation};
use std::collections::HashMap;

enum ShaderSource<'a> {
    VertexShader(&'a str),
    FragmentShader(&'a str),
}

impl<'a> ShaderSource<'a> {
    pub fn compile(&'a self, gl: &WebGL) -> Result<WebGlShader, JsValue> {
        let shader = gl.create_shader(match self {
            ShaderSource::VertexShader(_) => WebGL::VERTEX_SHADER,
            ShaderSource::FragmentShader(_) => WebGL::FRAGMENT_SHADER,
        }).ok_or_else(|| js_sys::Error::new("Cannot create shader"))?;

        gl.shader_source(&shader, match self {
            ShaderSource::VertexShader(s) => &s,
            ShaderSource::FragmentShader(s) => &s,
        });

        gl.compile_shader(&shader);

        match gl.get_shader_parameter(&shader, WebGL::COMPILE_STATUS).as_bool() {
            Some(true) => Ok(shader),
            _ => {
                if let Some(text) = gl.get_shader_info_log(&shader) {
                    Err(js_sys::Error::new(&format!("Failed compiling {} shader: {}", match self {
                        ShaderSource::VertexShader(_) => "vertex",
                        ShaderSource::FragmentShader(_) => "fragment",
                    }, text)).into())
                } else {
                    Err(js_sys::Error::new(&format!("Failed compiling {} shader, but no reason given.", match self {
                        ShaderSource::VertexShader(_) => "vertex",
                        ShaderSource::FragmentShader(_) => "fragment",
                    })).into())
                }
            },
        }
    }
}

pub struct ShaderProgram {
    gl: WebGL,
    program: WebGlProgram,
    attributes: HashMap<String, u32>,
    uniforms: HashMap<String, WebGlUniformLocation>,
}

impl ShaderProgram {
    pub fn new(gl: &WebGL, vertex_shader: &str, fragment_shader: &str, attribute_names: &[&str], uniform_names: &[&str]) -> Result<Self, JsValue> {
        let vertex_shader = ShaderSource::VertexShader(vertex_shader).compile(gl)?;
        let fragment_shader = ShaderSource::FragmentShader(fragment_shader).compile(gl).map_err(|err| {
            gl.delete_shader(Some(&vertex_shader));
            err
        })?;
        let program = gl.create_program().ok_or_else(|| js_sys::Error::new("Cannot create shader program")).map_err(|err| {
            gl.delete_shader(Some(&vertex_shader));
            gl.delete_shader(Some(&fragment_shader));
            err
        })?;

        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);

        gl.link_program(&program);
        gl.delete_shader(Some(&vertex_shader));
        gl.delete_shader(Some(&fragment_shader));

        match gl.get_program_parameter(&program, WebGL::LINK_STATUS).as_bool() {
            Some(true) => {
                let mut attributes = HashMap::new();
                let mut uniforms = HashMap::new();
                for attrib in attribute_names {
                    let location = gl.get_attrib_location(&program, &attrib);
                    if location >= 0 {
                        attributes.insert(String::from(*attrib), location as u32);
                    }
                }
                for uniform in uniform_names {
                    if let Some(location) = gl.get_uniform_location(&program, &uniform) {
                        uniforms.insert(String::from(*uniform), location.clone());
                    }
                }

                Ok(ShaderProgram {
                    gl: gl.clone(),
                    program,
                    attributes,
                    uniforms,
                })
            },
            _ => {
                let err = if let Some(text) = gl.get_program_info_log(&program) {
                    format!("Failed linking shader program: {}", text)
                } else {
                    "Failed linking shader program: No reason given.".to_string()
                };
                gl.delete_program(Some(&program));
                Err(js_sys::Error::new(&err).into())
            },
        }
    }

    pub fn use_program(&self) {
        self.gl.use_program(Some(&self.program));
    }

    pub fn uniform(&self, name: &str) -> Option<&WebGlUniformLocation> {
        Some(self.uniforms.get(name).expect(&format!("Unknown uniform '{}' in shader program", name)))
    }

    pub fn attribute(&self, name: &str) -> Option<u32> {
        self.attributes.get(name).cloned()
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}
