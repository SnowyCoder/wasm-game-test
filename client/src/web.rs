use std::cell::{RefCell};
use std::rc::Rc;
use std::time::Duration;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent as WebKeyboardEvent;

use crate::app::App;
use crate::console_log;
use crate::utils;
use specs::WorldExt;
use specs::shrev::EventChannel;
use crate::input::{KeyboardEvent, KeyState, MouseMoveEvent, ResizeEvent};

#[wasm_bindgen]
pub struct WebApp {
    app: Rc<RefCell<App>>,
    last_time: u32,
}

#[wasm_bindgen]
impl WebApp {
    pub fn create() -> Result<WebApp, JsValue> {
        utils::set_panic_hook();
        console_log!("Starting up");

        //connection::start_websocket().expect("Cannot start websocket");
        let app = App::create()?;

        let mut webapp = WebApp{
            app: Rc::new(RefCell::new(app)),
            last_time: 0,
        };

        webapp.on_resize();

        console_log!("Started");

        Ok(webapp)
    }

    pub fn on_click(&self) {
        console_log!("click");
    }

    pub fn on_mouse_move(&mut self, dx: f64, dy: f64) {
        self.app
            .borrow_mut()
            .world
            .write_resource::<EventChannel<MouseMoveEvent>>()
            .single_write(MouseMoveEvent {
                dx, dy
            })
    }

    pub fn on_resize(&mut self) {
        let app = self.app.borrow_mut();
        let width = app.canvas.client_width() as u32;
        let height = app.canvas.client_height() as u32;
        app
            .world
            .write_resource::<EventChannel<ResizeEvent>>()
            .single_write(ResizeEvent {
                width, height
            });
    }

    pub fn update(&mut self, now: u32) {
        /*if self.last_time == 0 {
            self.last_time = now;
        }*/
        let deltatime = now - self.last_time;

        self.app.borrow_mut().update(Duration::from_millis(deltatime as u64));

        self.last_time = now;
    }

    pub fn on_key_up(&mut self, e: &WebKeyboardEvent) {
        self.app
            .borrow_mut()
            .world
            .write_resource::<EventChannel<KeyboardEvent>>()
            .single_write(KeyboardEvent {
                state: KeyState::UP,
                key: e.key()
            })
    }

    pub fn on_key_down(&mut self, e: &WebKeyboardEvent) {
        self.app
            .borrow_mut()
            .world
            .write_resource::<EventChannel<KeyboardEvent>>()
            .single_write(KeyboardEvent {
                state: KeyState::DOWN,
                key: e.key()
            })
    }
}
