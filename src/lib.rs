use std::panic;
use gloo::events::EventListener;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen::JsValue;
use std::rc::Rc;

mod state;

use crate::state::ApplicationState;
use crate::state::Action;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Prism)]
    type Prism;

    #[wasm_bindgen(js_namespace = ["Prism", "languages"], js_name = css)]
    static CSS: JsValue;

    #[wasm_bindgen(js_namespace = Prism)]
    fn highlight(code: &str, language: &JsValue, langName: Option<&str>) -> String;

    #[wasm_bindgen(js_namespace = ["Prism", "plugins", "NormalizeWhitespace"])]
    fn normalize(code: &str, extra_settings: JsValue) -> String;
}

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = gloo::utils::document();

    let state: Rc<ApplicationState> = Rc::new(ApplicationState::new());
    let state2 = Rc::clone(&state);
    let state3 = Rc::clone(&state);
    let state4 = Rc::clone(&state);
    let state5 = Rc::clone(&state);

    let generate_button = document
        .get_element_by_id("generate-button")
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();
    let generate_button2 = generate_button.clone();

    let main_input = document
        .get_element_by_id("main-input")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    let main_input2 = main_input.clone();

    let generated_css_element = document.get_element_by_id("generated-css").unwrap();

    let typing_animation_style = document
        .get_element_by_id("typing-animation-style")
        .unwrap()
        .dyn_into::<web_sys::HtmlStyleElement>()
        .unwrap();

    let finished_state = document
        .get_element_by_id("finished-state")
        .unwrap()
        .dyn_into::<web_sys::HtmlDivElement>()
        .unwrap();

    let fix_interpolation_checkbox = document
        .get_element_by_id("fix-interpolation")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();

    let wait_at_start_checkbox = document
        .get_element_by_id("wait-at-start")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();

    let end_delay_seconds_input = Rc::new(document
        .get_element_by_id("end-delay-seconds")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap());
    let end_delay_seconds_input2 = Rc::clone(&end_delay_seconds_input);

    EventListener::new(&end_delay_seconds_input, "input", move |_| {
        let mut new_length = end_delay_seconds_input2.value_as_number();
        
        if new_length.is_nan() { // If invalid sets previous value
            end_delay_seconds_input2.set_value_as_number(*state5.end_delay_seconds.borrow());
            return
        } else if new_length.is_sign_negative() { // changes negative to positive
            new_length = new_length.abs();
            end_delay_seconds_input2.set_value_as_number(new_length);
        } else {
            *state5.end_delay_seconds.borrow_mut() = new_length;
        }

        if state5.recording_start_time.borrow().is_some() {
            let generated_css = match state5.generate_css() {
                Some(c) => c,
                None => return,
            };

            state5.set_css(generated_css);
        }
    })
    .forget();

    EventListener::new(&wait_at_start_checkbox, "change", move |_| {
        let mut toggle = state4.start_wait.borrow_mut();
        *toggle = !*toggle;
        drop(toggle);

        if state4.recording_start_time.borrow().is_some() {
            let generated_css = match state4.generate_css() {
                Some(c) => c,
                None => return,
            };

            state4.set_css(generated_css);
        }
    })
    .forget();

    EventListener::new(&fix_interpolation_checkbox, "change", move |_| {
        let mut toggle = state3.fix_interpolation.borrow_mut();
        *toggle = !*toggle;
        drop(toggle);

        if state3.recording_start_time.borrow().is_some() {
            let generated_css = match state3.generate_css() {
                Some(c) => c,
                None => return,
            };

            state3.set_css(generated_css);
        }
    })
    .forget();

    // Generate button click listener
    EventListener::new(&generate_button, "click", move |_| {
        let mut recording_started = state.recording_started.borrow_mut();

        if *recording_started {
            generate_button2.set_inner_html("Start recording");

            let generated_css = match state.generate_css() {
                Some(c) => c,
                None => {
                    main_input.set_hidden(true);
                    finished_state.set_hidden(true);
                    *recording_started = false;
                    return;
                }
            };

            main_input.set_hidden(true);
            finished_state.set_hidden(false);
            state.set_css(generated_css);
        } else {
            typing_animation_style.set_inner_html("");
            generated_css_element.set_inner_html("");
            generate_button2.set_inner_html("Stop");
            main_input.set_value("");
            main_input.set_hidden(false);
            finished_state.set_hidden(true);

            main_input.focus().unwrap();

            *state.recording_start_time.borrow_mut() = Some(chrono::Utc::now());
            *state.recording_actions.borrow_mut() = Vec::new();
        }

        *recording_started = !*recording_started;
    })
    .forget();

    // Input element listener!
    EventListener::new(&main_input2, "input", move |event| {
        if *state2.recording_started.borrow() {
            state2.recording_actions.borrow_mut().push(Action {
                date: chrono::Utc::now(),
                value: event
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            });
        }
    })
    .forget();
}
