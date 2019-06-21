use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as WebGL};

mod renderer;
mod shader_program;
mod cube;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if let Some(canvas) = document.get_element_by_id("wasm") {
        let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
        if let Some(context) = canvas.get_context("webgl2")? {
            let context: WebGL = context.dyn_into().unwrap();
            // context.clear_color(0.0, 0.0, 0.0, 1.0);
            // context.clear(WebGL::COLOR_BUFFER_BIT);
            renderer::init_renderer(context);
        }
    }

    Ok(())
}
