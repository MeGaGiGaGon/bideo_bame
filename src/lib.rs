use std::cell::OnceCell;
use std::rc::Rc;
use std::sync::RwLock;

use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use web_sys::js_sys;
use web_sys::CanvasRenderingContext2d;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use web_sys::HtmlCanvasElement;
use web_sys::TouchEvent;

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

fn context() -> CanvasRenderingContext2d {
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

    add_listener!(canvas(), "mousedown", dyn FnMut(_), |event: MouseEvent| {
        STATE.write().unwrap().backend.mouse_down(event);
    });
    add_listener!(canvas(), "mousemove", dyn FnMut(_), |event: MouseEvent| {
        STATE.write().unwrap().backend.mouse_move(event);
    });
    add_listener!(canvas(), "mouseup", dyn FnMut(_), |event: MouseEvent| {
        STATE.write().unwrap().backend.mouse_up(event);
    });
    
    add_listener!(document(), "keydown", dyn FnMut(_), |event: KeyboardEvent| {
        STATE.write().unwrap().backend.key_down(event);
    });
    add_listener!(document(), "keyup", dyn FnMut(_), |event: KeyboardEvent| {
        STATE.write().unwrap().backend.key_up(event);
    });

    add_listener!(canvas(), "touchstart", dyn FnMut(_), |event: TouchEvent| {
        STATE.write().unwrap().backend.touch_start(event);
    });

    // add_listener!(canvas(), "touchstart", dyn FnMut(_), |event: web_sys::TouchEvent| {
    //     let mut state = STATE.write().unwrap();
    //     event.touches();
    //     // state.touch_events.push(TouchEvent::new());
    // });
}

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
    let mut state = STATE.write().unwrap();
    if state.backend.pointer.primary_down {
        ctx.fill_rect(
            state.backend.pointer.pos.x.into(),
            state.backend.pointer.pos.y.into(),
            10.0,
            10.0,
        );
    }
    for touch in state.backend.touches.active_touches.iter() {
        ctx.fill_rect(
            touch.position.x.into(),
            touch.position.y.into(),
            10.0,
            10.0,
        );
    }
    if state.backend.keyboard.w {
        state.box_pos.y -= 10;
    }
    if state.backend.keyboard.a {
        state.box_pos.x -= 10;
    }
    if state.backend.keyboard.s {
        state.box_pos.y += 10;
    }
    if state.backend.keyboard.d {
        state.box_pos.x += 10;
    }
    ctx.fill_rect(state.box_pos.x.into(), state.box_pos.y.into(), 10.0, 10.0);
    request_animation_frame(recursive_closure.get().unwrap());
}

type RecursiveClosure = Rc<OnceCell<Closure<dyn FnMut()>>>;

#[derive(Debug)]
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

#[derive(Debug)]
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
    w: bool,
    a: bool,
    s: bool,
    d: bool,
}

impl Keyboard {
    const START: Self = Self {
        w: false,
        a: false,
        s: false,
        d: false,
    };
}

struct Touch {
    identifier: i32,
    position: Vec2,
}

struct Touches {
    active_touches: Vec<Touch>,
}

impl Touches {
    const START: Self = Self {
        active_touches: Vec::new(),
    };
}

struct Backend {
    pointer: Pointer,
    keyboard: Keyboard,
    touches: Touches,
}

impl Backend {
    const START: Self = Self {
        pointer: Pointer::START,
        keyboard: Keyboard::START,
        touches: Touches::START,
    };

    fn mouse_down(&mut self, event: MouseEvent) {
        let pointer = &mut self.pointer;
        match event.button() {
            0 => pointer.primary_down = true,
            1 => pointer.auxillary_down = true,
            2 => pointer.secondary_down = true,
            3 => pointer.fourth_down = true,
            4 => pointer.fifth_down = true,
            x => warn!("Unknown mouse down event button id {x}"), // Ouside specs, log and move on
        };
        // log!("down {} {:?}", event.button(), pointer);
        pointer.pos = Vec2::new(event.x(), event.y());
    }

    fn mouse_move(&mut self, event: MouseEvent) {
        let pointer = &mut self.pointer;
        pointer.pos = Vec2::new(event.x(), event.y());
    }
    
    fn mouse_up(&mut self, event: MouseEvent) {
        let pointer = &mut self.pointer;
        match event.button() {
            0 => pointer.primary_down = false,
            1 => pointer.auxillary_down = false,
            2 => pointer.secondary_down = false,
            3 => pointer.fourth_down = false,
            4 => pointer.fifth_down = false,
            x => warn!("Unknown mouse up event button id {x}"), // Ouside specs, log and move on
        };
        // log!("up {} {:?}", event.button(), pointer);
        pointer.pos = Vec2::new(event.x(), event.y());
    }

    fn key_down(&mut self, event: KeyboardEvent) {
        let keyboard = &mut self.keyboard;
        match event.code().as_ref() {
            "KeyW" => keyboard.w = true,
            "KeyA" => keyboard.a = true,
            "KeyS" => keyboard.s = true,
            "KeyD" => keyboard.d = true,
            x => warn!("Not yet implemented key down event code {x}"),
        }
    }
    
    fn key_up(&mut self, event: KeyboardEvent) {
        let keyboard = &mut self.keyboard;
        match event.code().as_ref() {
            "KeyW" => keyboard.w = false,
            "KeyA" => keyboard.a = false,
            "KeyS" => keyboard.s = false,
            "KeyD" => keyboard.d = false,
            x => warn!("Not yet implemented key up event code {x}"),
        }
    }

    fn touch_start(&mut self, event: TouchEvent) {
        // event.prevent_default(); // Stop event propigations to stop touches from moving the page
        let touches = &mut self.touches;
        let changed_touches = event.changed_touches();
        for i in 0..changed_touches.length() {
            let touch = changed_touches.get(i).unwrap();
            touches.active_touches.push(Touch { identifier: touch.identifier(), position: Vec2::new(touch.client_x(), touch.client_y())});
        }
    }
}

struct State {
    backend: Backend,
    box_pos: Vec2
}

static STATE: RwLock<State> = RwLock::new(State {
    backend: Backend::START,
    box_pos: Vec2 { x: 0, y: 0 },
});
