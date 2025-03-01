use std::cell::OnceCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::RwLock;

use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use web_sys::js_sys;
use web_sys::MouseEvent;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

macro_rules! warn {
    ( $( $t:tt )* ) => {
        web_sys::console::warn_1(&format!( $( $t )* ).into())
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
    document()
        .body()
        .expect("a document should always have a body")
}

fn canvas() -> HtmlCanvasElement {
    document()
        .get_element_by_id("game_canvas")
        .expect("game_canvas should exist")
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}

fn context() -> web_sys::CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap()
}

fn timestamp() -> f64 {
    js_sys::Date::now()
}

macro_rules! add_listener {
    ($target:expr, $event:literal, $closure_type:ty, $($closure:tt)*) => {
        {
            let closure = Closure::<$closure_type>::new($($closure)*);
            $target
                .add_event_listener_with_callback($event, closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    };
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    let data = 1;
    let recursive_closure: RecursiveClosure = Rc::new(OnceCell::new());
    let second_ref = recursive_closure.clone();
    recursive_closure
        .set(Closure::new(move || draw(second_ref.clone(), data)))
        .expect("Cell should not have been set");
    request_animation_frame(recursive_closure.get().unwrap());

    // add_listener!(canvas(), "mousedown", dyn FnMut(_), |event: web_sys::MouseEvent| {
    //     let mut state = STATE.write().unwrap();
    //     state
    //         .mouse_events
    //         .push(MouseEvent::new(MouseEventType::MouseDown, event));
    // });
    // add_listener!(canvas(), "mousemove", dyn FnMut(_), |event: web_sys::MouseEvent| {
    //     let mut state = STATE.write().unwrap();
    //     state
    //         .mouse_events
    //         .push(MouseEvent::new(MouseEventType::MouseMove, event));
    // });
    // add_listener!(canvas(), "mouseup", dyn FnMut(_), |event: web_sys::MouseEvent| {
    //     let mut state = STATE.write().unwrap();
    //     state.mouse_events.push(MouseEvent::new(MouseEventType::MouseUp, event));
    // });

    // add_listener!(document(), "keydown", dyn FnMut(_), |event: web_sys::KeyboardEvent| {
    //     let mut state = STATE.write().unwrap();
    //     state.keyboard_events.push(KeyboardEvent::new(KeyboardEventType::KeyDown, event));
    // });
    // add_listener!(document(), "keyup", dyn FnMut(_), |event: web_sys::KeyboardEvent| {
    //     let mut state = STATE.write().unwrap();
    //     state.keyboard_events.push(KeyboardEvent::new(KeyboardEventType::KeyUp, event));
    // });

    // add_listener!(canvas(), "touchstart", dyn FnMut(_), |event: web_sys::TouchEvent| {
    //     let mut state = STATE.write().unwrap();
    //     event.touches();
    //     // state.touch_events.push(TouchEvent::new());
    // });
}

// struct App {
//     last_tick
// }

// fn main_loop(recursive_closure: RecursiveClosure, app: &mut App) {
//     request_animation_frame(recursive_closure.get().unwrap());
// }

// #[apply(js_closure)]
fn draw(recursive_closure: RecursiveClosure, data: u8) {
    let canvas = canvas();
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;
    canvas.set_width(width);
    canvas.set_height(height);
    let ctx = context();
    ctx.set_fill_style_str("rgb(200 0 0)");
    ctx.fill_rect(10.0, 10.0, 50.0, 50.0);
    ctx.set_fill_style_str("rgb(0 0 200 / 50%)");
    ctx.fill_rect(30.0, 30.0, 50.0, 50.0);
    ctx.set_fill_style_str("rgb(255 255 255)");
    ctx.fill_text(&format!("{width} {height}"), 10.0, 10.0)
        .unwrap();
    // let mut state = STATE.write().unwrap();
    // state.process_mouse_events();
    // state.process_keyboard_events();
    // if state.mouse_is_down {
    //     ctx.fill_rect(
    //         state.last_mouse_pos.x.into(),
    //         state.last_mouse_pos.y.into(),
    //         10.0,
    //         10.0,
    //     );
    // }
    request_animation_frame(recursive_closure.get().unwrap());
}

type RecursiveClosure = Rc<OnceCell<Closure<dyn FnMut()>>>;

struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    const ZERO: Self = Self { x: 0, y: 0 };

    const fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

struct Pointer {
    primary_down: bool,
    auxillary_down: bool,
    secondary_down: bool,
    fourth_down: bool,
    fifth_down: bool,
    pos: Vec2,
}

impl Pointer {
    const START: Self = Self {
        primary_down: false,
        auxillary_down: false,
        secondary_down: false,
        fourth_down: false,
        fifth_down: false,
        pos: Vec2::ZERO,
    };
}

struct Keyboard {

}

struct Touch {

}

struct Backend {
    pointer: Pointer,
    keyboard: Keyboard,
    touch: Touch,
}

impl Backend {
    const START: Self = Self {
        pointer: Pointer::START,
        keyboard: Keyboard {},
        touch: Touch {},
    };

    fn mouse_down(&mut self, event: MouseEvent) {
        let pointer = &mut self.pointer;
        match event.button() {
            0 => pointer.primary_down = true,
            1 => pointer.primary_down = true,
            2 => pointer.primary_down = true,
            3 => pointer.primary_down = true,
            4 => pointer.primary_down = true,
            x => warn!("Unknown mouse event button id {x}"),
        };
        pointer.pos = Vec2::new(event.x(), event.y());
    }
}

struct State {
    backend: Backend,
}

static STATE: RwLock<State> = RwLock::new(State {
    backend: Backend::START,
});

// struct State {
//     counter: u8,
//     mouse_events: Vec<MouseEvent>,
//     keyboard_events: Vec<KeyboardEvent>,
//     mouse_is_down: bool,
//     last_mouse_pos: Vec2,
//     touch_events: LazyLock<Arc<Mutex<Vec<(TouchEventType, web_sys::TouchEvent)>>>>,
// }

// impl State {
//     /// Should be called once per frame
//     fn process_mouse_events(&mut self) {
//         let mouse_events = std::mem::take(&mut self.mouse_events);
//         // println!("{:?}", mouse_events.clone());
//         for event in mouse_events {
//             self.last_mouse_pos = event.pos;
//             if matches!(event.ty, MouseEventType::MouseDown) {
//                 self.mouse_is_down = true;
//             }
//             if matches!(event.ty, MouseEventType::MouseUp) {
//                 self.mouse_is_down = false;
//             }
//         }
//     }

//     /// Should be called once per frame
//     fn process_keyboard_events(&mut self) {
//         let keyboard_events = std::mem::take(&mut self.keyboard_events);
//         // println!("{:?}", keyboard_events);
//     }
// }

// static STATE: RwLock<State> = RwLock::new(State {
//     counter: 0,
//     mouse_events: vec![],
//     last_mouse_pos: Vec2::new(0, 0),
//     mouse_is_down: false,
//     keyboard_events: vec![],
//     touch_events: LazyLock::new(|| Arc::new(Mutex::new(vec![]))),
// });
