use wasm_bindgen::{prelude::{wasm_bindgen}};

macro_rules! println {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

// macro_rules! function {
//     ($name:ident, $( $t:tt )* ) => {
//         #[allow(non_upper_case_globals)]
//         static mut $name: std::cell::LazyCell<Function> = std::cell::LazyCell::new(|| {Closure::wrap(Box::new( $( $t )* ) as Box<dyn Fn()>).as_ref().unchecked_ref::<Function>().clone()});
//     };
// }

// fn test_callback() {

// }

// function!(test_callback, || {});
// static mut test_callback: std::cell::LazyCell<Function> = std::cell::LazyCell::new(|| {Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>).as_ref().unchecked_ref::<Function>().clone()});

// macro_rules! callback {
//     ($method:expr, $($args:tt)*, @$($callback:tt)*) => {

//     }
// }

#[wasm_bindgen(inline_js=r#"
export function get_canvas_context() {
    return null;
    const canvas = document.querySelector("body > canvas");
    if (canvas.getContext) {
        return canvas.getContext("2d");
    } else {
        return null;
    }
}
"#)]
extern "C" {
    fn get_canvas_context() -> Option<web_sys::CanvasRenderingContext2d>;
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    // let context = get_canvas_context();
    println!("{:?}", get_canvas_context());
    println!("{:?}", get_canvas_context());
    println!("{:?}", get_canvas_context());
    // get_canvas_context().unwrap().begin_path();
}
