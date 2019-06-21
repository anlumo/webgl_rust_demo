use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{WebGl2RenderingContext as WebGL};
use std::{rc::Rc, cell::RefCell};

use crate::cube::Cube;

pub fn init_renderer(gl: WebGL) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let outer_f = f.clone();

    let cube = Cube::new(&gl).expect("Failed loading cube");

    let window = web_sys::window().unwrap();
    if let Some(perf) = window.performance() {
        let start_time = perf.now();
        *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(WebGL::COLOR_BUFFER_BIT | WebGL::DEPTH_BUFFER_BIT);
            cube.render(perf.now() - start_time);

            window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("failed requesting animation frame");
        }) as Box<dyn FnMut()>));

        let window = web_sys::window().unwrap();
        window.request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }
}
