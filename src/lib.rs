use std::cell::LazyCell;

use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

macro_rules! println {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}


#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    let window = web_sys::window().unwrap();
    let closure: Closure<dyn FnMut() -> ()> = Closure::new(|| {draw()});
    window.request_animation_frame(closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}
// https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html

fn draw() {
    let window = web_sys::window().unwrap();
    let canvas = window.document().unwrap().get_element_by_id("game_canvas").unwrap().unchecked_into::<HtmlCanvasElement>();
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
    canvas.set_width(width);
    canvas.set_height(height);
    let ctx = canvas.get_context("2d").unwrap().unwrap().unchecked_into::<CanvasRenderingContext2d>();
    ctx.set_fill_style_str("rgb(200 0 0)");
    ctx.fill_rect(10.0, 10.0, 50.0, 50.0);
    ctx.set_fill_style_str("rgb(0 0 200 / 50%)");
    ctx.fill_rect(30.0, 30.0, 50.0, 50.0);
    ctx.set_fill_style_str("rgb(255 255 255)");
    ctx.fill_text(&format!("{width} {height}"), 10.0, 10.0).unwrap();
    // window.request_animation_frame(draw.as_ref().unchecked_ref()).unwrap();
}