use crate::normalize;
use crate::Prism;
use crate::WrappedGetElementById;
use crate::CSS;
use chrono::{DateTime, Duration, Utc};
use std::cell::RefCell;
use wasm_bindgen::JsValue;
use web_sys::HtmlButtonElement;
use web_sys::HtmlDivElement;
use web_sys::HtmlInputElement;
use web_sys::{Element, HtmlStyleElement};

#[derive(Clone)]
pub struct Action {
    pub date: DateTime<Utc>,
    pub value: String,
}

pub struct ApplicationState {
    pub recording_started: RefCell<bool>,
    pub recording_actions: RefCell<Vec<Action>>,
    pub recording_start_time: RefCell<Option<DateTime<Utc>>>,

    pub fix_interpolation: RefCell<bool>,
    pub start_wait: RefCell<bool>,
    pub end_delay_seconds: RefCell<f64>,

    pub elements: ApplicationElements,
}

pub struct ApplicationElements {
    pub typing_animation_style: HtmlStyleElement,
    pub generated_css_element: Element,
    pub finished_state: HtmlDivElement,
    pub fix_interpolation_checkbox: HtmlInputElement,
    pub wait_at_start_checkbox: HtmlInputElement,
    pub generate_button: HtmlButtonElement,
    pub input: HtmlInputElement,
    pub end_delay_seconds_input: HtmlInputElement,
}

impl ApplicationElements {
    fn new() -> ApplicationElements {
        let document = gloo::utils::document();

        ApplicationElements {
            typing_animation_style: document.wr_get_element_by_id("typing-animation-style"),
            generated_css_element: document.wr_get_element_by_id("generated-css"),
            finished_state: document.wr_get_element_by_id("finished-state"),
            fix_interpolation_checkbox: document.wr_get_element_by_id("fix-interpolation"),
            wait_at_start_checkbox: document.wr_get_element_by_id("wait-at-start"),
            generate_button: document.wr_get_element_by_id("generate-button"),
            input: document.wr_get_element_by_id("main-input"),
            end_delay_seconds_input: document.wr_get_element_by_id("end-delay-seconds"),
        }
    }
}
impl ApplicationState {
    pub fn new() -> Self {
        ApplicationState {
            recording_started: RefCell::new(false),
            recording_actions: RefCell::new(Vec::new()),
            recording_start_time: RefCell::new(None),

            fix_interpolation: RefCell::new(true),
            start_wait: RefCell::new(true),
            end_delay_seconds: RefCell::new(0.0),
            elements: ApplicationElements::new(),
        }
    }

    pub fn set_css(&self, generated_css: String) {
        self.elements
            .typing_animation_style
            .set_inner_html(&generated_css);
        self.elements
            .generated_css_element
            .set_inner_html(&Prism::highlight(
                &normalize(&generated_css, JsValue::null()),
            &CSS.with(JsValue::clone),
                None,
            ));
    }

    pub fn generate_css(&self) -> Option<String> {
        let mut actions = self.recording_actions.borrow().clone();
        if *self.end_delay_seconds.borrow() > 0.0 {
            let action = actions.last()?;
            let new_date = action.date + Duration::seconds(*self.end_delay_seconds.borrow() as i64);

            actions.push(Action {
                date: new_date,
                value: action.value.clone(),
            });
        }

        let stop_date: DateTime<Utc> = actions.last()?.date;

        let start_date: DateTime<Utc> = if *self.start_wait.borrow() {
            (*self.recording_start_time.borrow()).unwrap()
        } else {
            actions.first()?.date
        };

        // Finds animation duration
        let diff = (start_date - stop_date).num_milliseconds() as f64;
        let animation_duration_seconds = (diff / 1000.0).abs();

        let keyframes = actions
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, action)| {
                // Calculates percentage relative to the animation duration
                let temp_diff = (start_date - action.date).num_milliseconds() as f64;
                let action_time = (temp_diff / 1000.0).abs();
                let current_percent = (action_time / animation_duration_seconds) * 100.0;

                acc + &if *self.fix_interpolation.borrow() {
                    let last_value = match actions.get(i - 1) {
                        Some(s) => &s.value,
                        None => "",
                    };

                    let mut last_percent: f64 = current_percent - 0.1;
                    if last_percent < 0.0 {
                        last_percent = 0.0
                    };

                    format!(
                        "
                    {last_percent}% {{
                        content: \"{last_value}\";
                    }}
                    {current_percent}% {{
                        content: \"{}\";
                    }}",
                        action.value
                    )
                } else {
                    format!(
                        "
                    {current_percent}% {{
                        content: \"{}\";
                    }}",
                        action.value
                    )
                }
            });

        // If i just generate static name animation wont start from beginning when applying to <style>
        let animation_timestamp = chrono::Utc::now().timestamp_millis();

        Some(format!(
            "
                #typing-animation-box::after {{
                    animation-name: typing-animation-{animation_timestamp};
                    animation-duration: {animation_duration_seconds}s;
                    animation-iteration-count: infinite;
                    content: \"\";
                }}

                @keyframes typing-animation-{animation_timestamp} {{
                    0% {{
                        content: \"\";
                    }}{keyframes}
                }}
                "
        ))
    }
}
