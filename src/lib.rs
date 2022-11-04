use std::panic;
use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Utc};
use gloo::events::EventListener;
use wasm_bindgen::{prelude::*, JsCast};

struct Action {
    date: DateTime<Utc>,
    value: String,
}

struct ApplicationState {
    recording_started: RefCell<bool>,
    recording_actions: RefCell<Vec<Action>>,
    recording_start_time: RefCell<Option<DateTime<Utc>>>,
}

const SHOULD_REPEAT_ANIMATION: bool = true;

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
pub async fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = gloo::utils::document();

    let state: Rc<ApplicationState> = Rc::new(ApplicationState {
        recording_started: RefCell::new(false),
        recording_actions: RefCell::new(Vec::new()),
        recording_start_time: RefCell::new(None),
    });
    let state2 = Rc::clone(&state);

    // DOM >///<
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

    let generated_css_element = document
        .get_element_by_id("generated-css")
        .unwrap();

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

    // Generate button click listener
    EventListener::new(&generate_button, "click", move |_| {
        let mut recording_started = state.recording_started.borrow_mut();

        if *recording_started {
            *recording_started = false;

            generate_button2.set_inner_html("Start recording");

            let start_date = (*state.recording_start_time.borrow()).unwrap();

            let actions = state.recording_actions.borrow();
            let last_action = match actions.last() {
                Some(v) => v,
                None => {
                    main_input.set_hidden(true);
                    finished_state.set_hidden(true);
                    return;
                }
            };

            // Finds animation duration
            let diff = (start_date - last_action.date).num_milliseconds() as f64;
            let animation_duration_seconds = (diff / 1000.0).abs();

            let keyframes = actions
                .iter()
                .map(|action| {
                    // Calculates percentage relative to the animation duration
                    let temp_diff = (start_date - action.date).num_milliseconds() as f64;
                    let action_time = (temp_diff / 1000.0).abs();

                    let current_percent = (action_time / animation_duration_seconds) * 100.0;

                    format!(
                        "
                {}% {{
                    content: \"{}\";
                }}",
                        current_percent, action.value
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            let repeat_part = if SHOULD_REPEAT_ANIMATION {
                "
                animation-iteration-count: infinite;
                animation-delay: 0s;
                "
            } else {
                ""
            };

            // If i just generate static name animation wont start from beginning when applying to <style>
            let animation_timestamp = start_date.timestamp();

            let generated_css = &format!(
                "
            #typing-animation-box::after {{
                animation-name: typing-animation-{animation_timestamp};
                animation-duration: {animation_duration_seconds}s;
                {repeat_part}
                content: \"\";
            }}

            @keyframes typing-animation-{animation_timestamp} {{
                0% {{
                    content: \"\";
                }}
                {keyframes}
            }}
            "
            );

            main_input.set_hidden(true);
            finished_state.set_hidden(false);
            typing_animation_style.set_inner_html(generated_css);
            generated_css_element.set_inner_html(&Prism::highlight(
                &normalize(generated_css, JsValue::null()),
                &CSS.clone(),
                None,
            ));
        } else {
            *recording_started = true;
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
