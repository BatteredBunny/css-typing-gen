use gloo::events::EventListener;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Document;

mod state;

use crate::state::Action;
use crate::state::ApplicationState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Prism)]
    type Prism;

    #[wasm_bindgen(thread_local_v2, js_namespace = ["Prism", "languages"], js_name = css)]
    static CSS: JsValue;

    #[wasm_bindgen(js_namespace = Prism)]
    fn highlight(code: &str, language: &JsValue, langName: Option<&str>) -> String;

    #[wasm_bindgen(js_namespace = ["Prism", "plugins", "NormalizeWhitespace"])]
    fn normalize(code: &str, extra_settings: JsValue) -> String;
}

trait WrappedGetElementById {
    fn wr_get_element_by_id<T: JsCast>(&self, id: &str) -> T;
}

impl WrappedGetElementById for Document {
    fn wr_get_element_by_id<T: JsCast>(&self, id: &str) -> T {
        self.get_element_by_id(id)
            .expect("Can't find element with that ID!")
            .dyn_into::<T>()
            .expect("Wrong element present")
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let state: Rc<ApplicationState> = Rc::new(ApplicationState::new());

    let inner_state = state.clone();
    EventListener::new(
        &state.elements.end_delay_seconds_input,
        "input",
        move |_| {
            let mut new_length = inner_state
                .elements
                .end_delay_seconds_input
                .value_as_number();
            let previous_length = *inner_state.end_delay_seconds.borrow();

            if new_length.is_nan() {
                // If invalid sets previous value
                inner_state
                    .elements
                    .end_delay_seconds_input
                    .set_value_as_number(previous_length);
                return;
            } else if new_length.is_sign_negative() {
                // changes negative to positive
                new_length = new_length.abs();

                if new_length == previous_length {
                    return;
                }

                inner_state
                    .elements
                    .end_delay_seconds_input
                    .set_value_as_number(new_length);
            }

            *inner_state.end_delay_seconds.borrow_mut() = new_length;

            if inner_state.recording_start_time.borrow().is_some() {
                let generated_css = match inner_state.generate_css() {
                    Some(c) => c,
                    None => return,
                };

                inner_state.set_css(generated_css);
            }
        },
    )
    .forget();

    let inner_state = state.clone();
    EventListener::new(
        &state.elements.wait_at_start_checkbox,
        "change",
        move |_| {
            let mut toggle = inner_state.start_wait.borrow_mut();
            *toggle = !*toggle;
            drop(toggle);

            if inner_state.recording_start_time.borrow().is_some() {
                let generated_css = match inner_state.generate_css() {
                    Some(c) => c,
                    None => return,
                };

                inner_state.set_css(generated_css);
            }
        },
    )
    .forget();

    let inner_state = state.clone();
    EventListener::new(
        &state.elements.fix_interpolation_checkbox,
        "change",
        move |_| {
            let mut toggle = inner_state.fix_interpolation.borrow_mut();
            *toggle = !*toggle;
            drop(toggle);

            if inner_state.recording_start_time.borrow().is_some() {
                let generated_css = match inner_state.generate_css() {
                    Some(c) => c,
                    None => return,
                };

                inner_state.set_css(generated_css);
            }
        },
    )
    .forget();

    // Generate button click listener
    let inner_state = state.clone();
    EventListener::new(&state.elements.generate_button, "click", move |_| {
        let mut recording_started = inner_state.recording_started.borrow_mut();
        if *recording_started {
            inner_state
                .elements
                .generate_button
                .set_inner_html("Start recording");

            let generated_css = match inner_state.generate_css() {
                Some(c) => c,
                None => {
                    inner_state.elements.input.set_hidden(true);
                    inner_state.elements.finished_state.set_hidden(true);
                    *recording_started = false;
                    return;
                }
            };

            inner_state.elements.input.set_hidden(true);
            inner_state.elements.finished_state.set_hidden(false);
            inner_state.set_css(generated_css);
        } else {
            inner_state
                .elements
                .typing_animation_style
                .set_inner_html("");
            inner_state
                .elements
                .generated_css_element
                .set_inner_html("");
            inner_state.elements.generate_button.set_inner_html("Stop");
            inner_state.elements.input.set_value("");
            inner_state.elements.input.set_hidden(false);
            inner_state.elements.finished_state.set_hidden(true);

            inner_state.elements.input.focus().unwrap();

            *inner_state.recording_start_time.borrow_mut() = Some(chrono::Utc::now());
            *inner_state.recording_actions.borrow_mut() = Vec::new();
        }

        *recording_started = !*recording_started;
    })
    .forget();

    // Input element listener!
    let inner_state = state.clone();
    EventListener::new(&state.elements.input, "input", move |_| {
        if *inner_state.recording_started.borrow() {
            inner_state.recording_actions.borrow_mut().push(Action {
                date: chrono::Utc::now(),
                value: inner_state.elements.input.value(),
            });
        }
    })
    .forget();
}
