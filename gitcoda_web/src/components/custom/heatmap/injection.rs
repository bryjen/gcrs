/// Load echarts library and run heatmap initialization script
#[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
pub fn inject_or_run_echarts_script(
    data: &[super::HeatmapData],
    selected_date: Option<String>,
    status_signal: leptos::prelude::RwSignal<String>,
    selected_signal: leptos::prelude::RwSignal<Option<String>>,
) {
    use super::script::build_echarts_script;
    use leptos::prelude::{GetUntracked, Set};
    use leptos::wasm_bindgen::{closure::Closure, JsCast};

    // Set up click callback
    let closure = Closure::<dyn Fn(String, u32)>::new(move |date: String, count: u32| {
        let current = selected_signal.get_untracked();
        if current.as_ref() == Some(&date) {
            selected_signal.set(None);
            status_signal.set("Select a date to see details".to_string());
        } else {
            selected_signal.set(Some(date.clone()));
            status_signal.set(format!("Date: {} | Activities: {}", date, count));
        }
    });

    js_sys::Reflect::set(
        &leptos::web_sys::window().unwrap(),
        &leptos::wasm_bindgen::JsValue::from_str("__heatmapOnClick"),
        closure.as_ref(),
    )
    .unwrap();
    closure.forget();

    let window = leptos::web_sys::window().unwrap();
    let document = window.document().unwrap();
    let data_owned = data.to_vec();

    if document.get_element_by_id("echarts-script").is_none() {
        let s = document.create_element("script").unwrap();
        s.set_attribute("id", "echarts-script").unwrap();
        s.set_attribute(
            "src",
            "https://fastly.jsdelivr.net/npm/echarts@5/dist/echarts.min.js",
        )
        .unwrap();
        let closure = Closure::once(move || {
            js_sys::eval(&build_echarts_script(&data_owned, selected_date.clone())).unwrap();
        });
        s.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
        document.head().unwrap().append_child(&s).unwrap();
    } else {
        js_sys::eval(&build_echarts_script(data, selected_date)).unwrap();
    }
}
