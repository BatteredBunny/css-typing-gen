
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlStyleElement};
use chrono::{DateTime, Utc, Duration};
use std::{cell::RefCell};
use crate::CSS;
use crate::normalize;
use crate::Prism;

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

    typing_animation_style: HtmlStyleElement,
    generated_css_element: Element,
}

impl ApplicationState {
    pub fn new() -> Self {
        let document = gloo::utils::document();

        ApplicationState {
            recording_started: RefCell::new(false),
            recording_actions: RefCell::new(Vec::new()),
            recording_start_time: RefCell::new(None),

            fix_interpolation: RefCell::new(true),
            start_wait: RefCell::new(true),
            end_delay_seconds: RefCell::new(0.0),

            typing_animation_style: document
                .get_element_by_id("typing-animation-style")
                .unwrap()
                .dyn_into::<web_sys::HtmlStyleElement>()
                .unwrap(),
            generated_css_element: document.get_element_by_id("generated-css").unwrap(),
        }
    }

    pub fn set_css(&self, generated_css: String) {
        self.typing_animation_style.set_inner_html(&generated_css);
        self.generated_css_element.set_inner_html(&Prism::highlight(
            &normalize(&generated_css, JsValue::null()),
            &CSS.clone(),
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
                value: action.value.clone()
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
            .map(|(i, action)| {
                // Calculates percentage relative to the animation duration
                let temp_diff = (start_date - action.date).num_milliseconds() as f64;
                let action_time = (temp_diff / 1000.0).abs();
                let current_percent = (action_time / animation_duration_seconds) * 100.0;

                if *self.fix_interpolation.borrow() {
                    let last_value = if i == 0 { "" } else { &actions[i - 1].value };

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
            })
            .collect::<Vec<String>>()
            .join("");

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
