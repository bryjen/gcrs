use leptos::html::Canvas;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::components::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[derive(Clone)]
pub struct LanguageBar {
    pub label: String,
    pub percent: f64,
    pub color: String,
}

#[component]
pub fn RepoLanguageBar(languages: Vec<LanguageBar>) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let open = RwSignal::new(false);

    let languages_for_effect = languages.clone();
    Effect::new(move |_| {
        if canvas_ref.get().is_none() {
            return;
        }

        let handlers = build_handlers_array(&languages_for_effect, open);
        js_sys::Reflect::set(
            &web_sys::window().unwrap(),
            &wasm_bindgen::JsValue::from_str("__langBarHandlers"),
            &handlers,
        )
        .unwrap();

        inject_or_run_script(build_chart_script(&languages_for_effect));
    });

    let legend = languages
        .iter()
        .map(|l| {
            let label = l.label.clone();
            let color = l.color.clone();
            let percent = l.percent;
            view! {
                <div class="flex items-center gap-1.5 text-xs text-muted-foreground">
                    <span style=format!("background:{color}") class="inline-block w-3 h-3 rounded-sm" />
                    {label}" "{percent}"%"
                </div>
            }
        })
        .collect_view();

    view! {
        <div>
            <div class="relative w-full h-[30px]">
                <canvas id="lang-bar" node_ref=canvas_ref />
            </div>
            <div
                id="lang-tooltip"
                style="position:fixed;background:#333;color:#fff;padding:6px 10px;border-radius:4px;font-size:12px;pointer-events:none;opacity:0;"
            />
            <Collapsible open=open>
                <CollapsibleContent>
                    <div class="flex flex-wrap gap-3 pt-2">
                        {legend}
                    </div>
                </CollapsibleContent>
            </Collapsible>
        </div>
    }
}

fn on_language_click(_lang: &LanguageBar, open: RwSignal<bool>) {
    // web_sys::window()
    //     .unwrap()
    //     .alert_with_message(&format!("Language: {}", _lang.label))
    //     .unwrap();
    open.update(|v| *v = !*v);
}

fn build_handlers_array(languages: &[LanguageBar], open: RwSignal<bool>) -> js_sys::Array {
    let array = js_sys::Array::new();
    for lang in languages {
        let lang = lang.clone();
        let closure = Closure::<dyn Fn()>::new(move || on_language_click(&lang, open));
        array.push(closure.as_ref().unchecked_ref());
        closure.forget();
    }
    array
}

fn build_chart_script(languages: &[LanguageBar]) -> String {
    let languages_json = languages
        .iter()
        .map(|l| {
            format!(
                r#"{{ label: "{}", data: [{:.1}], backgroundColor: "{}", borderSkipped: false }}"#,
                l.label, l.percent, l.color
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let callbacks_js = languages
        .iter()
        .enumerate()
        .map(|(i, l)| format!(r#""{label}": () => handlers[{i}]()"#, label = l.label))
        .collect::<Vec<_>>()
        .join(",");

    format!(
        r#"
        (function() {{
            const canvas = document.getElementById('lang-bar');
            const tooltip = document.getElementById('lang-tooltip');
            const handlers = window.__langBarHandlers;
            const handlerMap = {{ {callbacks_js} }};

            canvas.addEventListener('mousemove', e => {{
                if (tooltip.style.opacity === '1') {{
                    tooltip.style.left = e.clientX + 14 + 'px';
                    tooltip.style.top  = e.clientY - 28 + 'px';
                }}
            }});
            canvas.addEventListener('mouseleave', () => {{
                tooltip.style.opacity = 0;
                canvas.style.cursor = 'default';
            }});

            const chart = new Chart(canvas, {{
                type: 'bar',
                data: {{ labels: [''], datasets: [{languages_json}] }},
                options: {{
                    indexAxis: 'y', responsive: true, maintainAspectRatio: false,
                    plugins: {{
                        legend: {{ display: false }},
                        tooltip: {{
                            enabled: false,
                            external({{ tooltip: t }}) {{
                                if (t.opacity === 0) {{ tooltip.style.opacity = 0; canvas.style.cursor = 'default'; return; }}
                                const rect = canvas.getBoundingClientRect();
                                tooltip.style.opacity = 1;
                                tooltip.style.left = rect.left + t._eventPosition.x + 14 + 'px';
                                tooltip.style.top  = rect.top  + t._eventPosition.y - 28 + 'px';
                                tooltip.textContent = t.dataPoints[0].dataset.label + ': ' + t.dataPoints[0].raw + '%';
                                canvas.style.cursor = 'pointer';
                            }}
                        }}
                    }},
                    onClick(e) {{
                        const hits = chart.getElementsAtEventForMode(e, 'nearest', {{ intersect: true }}, false);
                        if (!hits.length) return;
                        const label = chart.data.datasets[hits[0].datasetIndex].label;
                        handlerMap[label]?.();
                    }},
                    scales: {{
                        x: {{ stacked: true, display: false, max: 100 }},
                        y: {{ stacked: true, display: false }}
                    }}
                }}
            }});
        }})();
    "#
    )
}

fn inject_or_run_script(script: String) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if document.get_element_by_id("chartjs-script").is_none() {
        let s = document.create_element("script").unwrap();
        s.set_attribute("id", "chartjs-script").unwrap();
        s.set_attribute(
            "src",
            "https://cdnjs.cloudflare.com/ajax/libs/Chart.js/4.4.1/chart.umd.js",
        )
        .unwrap();
        let closure = Closure::once(move || {
            js_sys::eval(&script).unwrap();
        });
        s.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
        document.head().unwrap().append_child(&s).unwrap();
    } else {
        js_sys::eval(&script).unwrap();
    }
}
