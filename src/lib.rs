use std::cell::OnceCell;
use std::rc::Rc;
use std::sync::RwLock;

use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast as _,
};

use web_sys::CanvasRenderingContext2d;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use web_sys::HtmlCanvasElement;
use web_sys::TouchEvent;

macro_rules! warn {
    ( $( $t:tt )* ) => {
        web_sys::console::warn_1(&format!( $( $t )* ).into())
    }
}

macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into())
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

fn canvas() -> HtmlCanvasElement {
    document()
        .get_element_by_id("game_canvas")
        .expect("element id game_canvas should exist")
        .dyn_into::<HtmlCanvasElement>()
        .expect("if element id game_canvas exists it should be a canvas, and this cast shouldn't fail")
}

fn context() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .expect("the canvas's 2d context should have been loaded by now")
        .expect("the canvas's 2d context should have been loaded by now")
        .dyn_into()
        .expect("this cast should always succeed")
}

macro_rules! add_listener {
    ($target:expr, $event:literal, |$event_ident:ident: $c_v_t:ty| {$($closure:tt)*}) => {
        {
            let closure = Closure::<dyn FnMut(_)>::new(|$event_ident: $c_v_t| {
                $event_ident.prevent_default();
                $($closure)*
            });
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
    let recursive_closure: RecursiveClosure = Rc::new(OnceCell::new());
    let second_ref = Rc::clone(&recursive_closure);
    recursive_closure
        .set(Closure::new(move || draw(Rc::clone(&second_ref))))
        .expect("Cell should not have been set");
    request_animation_frame(recursive_closure.get().expect("Since JS is only single threaded, this should always succeed"));

    add_listener!(canvas(), "mousedown", |event: MouseEvent| {
        STATE.write().expect("Since JS is only single threaded, this should always succeed").backend.mouse_down(event);
    });
    add_listener!(canvas(), "mousemove", |event: MouseEvent| {
        STATE.write().expect("Since JS is only single threaded, this should always succeed").backend.mouse_move(event);
    });
    add_listener!(canvas(), "mouseup", |event: MouseEvent| {
        STATE.write().expect("Since JS is only single threaded, this should always succeed").backend.mouse_up(event);
    });
    
    add_listener!(document(), "keydown", |event: KeyboardEvent| {
        STATE.write().expect("Since JS is only single threaded, this should always succeed").backend.key_down(event);
    });
    add_listener!(document(), "keyup", |event: KeyboardEvent| {
        STATE.write().expect("Since JS is only single threaded, this should always succeed").backend.key_up(event);
    });

    add_listener!(canvas(), "touchstart", |event: TouchEvent| {
        STATE.write().expect("Since JS is only single threaded, this should always succeed").backend.touch_start(event);
    });
    add_listener!(canvas(), "touchmove", |event: TouchEvent| {
        STATE.write().expect("Since JS is only single threaded, this should always succeed").backend.touch_move(event);
    });
    add_listener!(canvas(), "touchend", |event: TouchEvent| {
        STATE.write().expect("Since JS is only single threaded, this should always succeed").backend.touch_end_or_cancel(event);
    });
    add_listener!(canvas(), "touchcancel", |event: TouchEvent| {
        STATE.write().expect("Since JS is only single threaded, this should always succeed").backend.touch_end_or_cancel(event);
    });
}

fn draw(recursive_closure: RecursiveClosure) {
    let canvas = canvas();
    let width = window().inner_width().expect("this property should always be accessable").as_f64().expect("I don't see how this could fail") as u32;
    let height = window().inner_height().expect("this property should always be accessable").as_f64().expect("I don't see how this could fail") as u32;
    canvas.set_width(width);
    canvas.set_height(height);
    let ctx = context();
    ctx.set_fill_style_str("rgb(200 0 0)");
    ctx.fill_rect(10.0, 10.0, 50.0, 50.0);
    ctx.set_fill_style_str("rgb(0 0 200 / 50%)");
    ctx.fill_rect(30.0, 30.0, 50.0, 50.0);
    ctx.set_fill_style_str("rgb(255 255 255)");
    ctx.fill_text(&format!("{width} {height}"), 10.0, 10.0)
        .expect("The default font should exist, so this shouldn't fail");
    let mut state = STATE.write().expect("Since JS is only single threaded, this should always succeed");
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
    request_animation_frame(recursive_closure.get().expect("Since JS is only single threaded, this should always succeed"));
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
        let touches = &mut self.touches;
        let changed_touches = event.changed_touches();
        for i in 0..changed_touches.length() {
            let touch = changed_touches.get(i).expect("Based on the above loop this should always succeed");
            touches.active_touches.push(Touch { identifier: touch.identifier(), position: Vec2::new(touch.client_x(), touch.client_y())});
        }
    }
    
    fn touch_move(&mut self, event: TouchEvent) {
        let touches = &mut self.touches;
        let changed_touches = event.changed_touches();
        for i in 0..changed_touches.length() {
            let touch = changed_touches.get(i).expect("Based on the above loop this should always succeed");
            if let Some(existing_touch) = touches.active_touches.iter_mut().find(|x| x.identifier == touch.identifier()) {
                existing_touch.position = Vec2::new(touch.client_x(), touch.client_y());
            } else {
                error!("Received touchmove event with id {} and no corresponding active_touches entry", touch.identifier());
            }
        }
    }
    
    fn touch_end_or_cancel(&mut self, event: TouchEvent) {
        let touches = &mut self.touches;
        let changed_touches = event.changed_touches();
        for i in 0..changed_touches.length() {
            let touch = changed_touches.get(i).expect("Based on the above loop this should always succeed");
            let active_touches = std::mem::take(&mut touches.active_touches);
            touches.active_touches = active_touches.into_iter().filter(|x| x.identifier != touch.identifier()).collect();
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
