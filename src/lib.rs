use std::cell::LazyCell;
use std::cell::OnceCell;
use std::cell::RefCell;
use std::rc::Rc;

use macro_rules_attribute::apply;
use paste::paste;
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

macro_rules! println {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

type RecursiveClosure = Rc<OnceCell<Closure<dyn FnMut()>>>;

// macro_rules! js_recursive_closure {
//     (fn $fn_name:ident(recursive_closure: RecursiveClosure $($(,)? $arg:ident: $arg_ty:ty)* $(,)?) {$($body:tt)*}) => {
//         const $fn_name: LazyCell<RecursiveClosure> = LazyCell::new(|| {
//             let recursive_closure: RecursiveClosure = Rc::new(OnceCell::new());
//             let second_ref = recursive_closure.clone();
//             recursive_closure.set(Closure::new(move || paste!([<_$fn_name>])(second_ref.clone(), ))).expect("Cell should not have been set");
//             recursive_closure
//         });
//         paste!{fn [<_$fn_name>](recursive_closure: RecursiveClosure, $($arg: $arg_ty,)*) {$($body)*}}
//     }
// }

// macro_rules! js_closure {
//     ($($f:tt)* [<self>] $($e:tt)*) => {
//         {
//             let _recursive_closure: RecursiveClosure = Rc::new(OnceCell::new());
//             let _second_ref = _recursive_closure.clone();
//             $($f)* _second_ref.clone() $($e:tt)*
//     };
// }

macro_rules! js_closure {
    (fn $fn_name:ident($($arg:ident: $arg_ty:ty $(,)?)*) {$($body:tt)*}) => {
        #[allow(non_upper_case_globals)]
        const $fn_name: LazyCell<Closure<dyn FnMut($($arg_ty,)*)>> = LazyCell::new(|| {
            Closure::new(paste!([<_$fn_name>]))
        });
        paste!{fn [<_$fn_name>]($($arg: $arg_ty,)*) {$($body)*}}
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("a global window should always exist")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requestAnimationFrame registration should not fail");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("a window should always have a document")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("a document should always have a body")
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    let data = 1;
    let recursive_closure: RecursiveClosure = Rc::new(OnceCell::new());
    let second_ref = recursive_closure.clone();
    recursive_closure.set(Closure::new(move || draw(second_ref.clone(), data))).expect("Cell should not have been set");
    request_animation_frame(recursive_closure.get().unwrap());
}

// #[apply(js_closure)]
fn draw(recursive_closure: RecursiveClosure, data: u8) {
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
    request_animation_frame(recursive_closure.get().unwrap());
}

#[apply(js_closure)]
fn mouse_down(event: web_sys::MouseEvent, context: Rc<CanvasRenderingContext2d>) {

}