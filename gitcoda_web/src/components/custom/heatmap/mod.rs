mod injection;
mod script;
mod types;

pub use types::HeatmapData;

use leptos::prelude::*;

#[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
use injection::inject_or_run_echarts_script;

/// Activity heatmap component showing contributions throughout a year
/// Uses ECharts calendar visualization with interactive date selection
#[island]
#[allow(unused_variables)]
pub fn ActivityHeatmap(data: Vec<HeatmapData>) -> impl IntoView {
    let status_text = RwSignal::new("Select a date to see details".to_string());
    let selected_date = RwSignal::new(None::<String>);
    let heatmap_ref = NodeRef::<leptos::html::Div>::new();

    #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
    {
        let data_for_effect = data.clone();
        let status_signal = status_text;
        let selected_signal = selected_date;
        Effect::new(move |_| {
            if heatmap_ref.get().is_none() {
                return;
            }

            inject_or_run_echarts_script(
                &data_for_effect,
                selected_signal.get_untracked(),
                status_signal,
                selected_signal,
            );
        });
    }

    view! {
        <div
            node_ref=heatmap_ref
            class="relative overflow-hidden w-fit"
            style="width: 820px; height: 180px;"
        >
            <div id="heatmap" style="width: 100%; height: 100%;"></div>
        </div>

        /*
        <div class="flex flex-col items-center justify-start gap-6">
            <div
                node_ref=heatmap_ref
                class="relative overflow-hidden w-fit"
                style="width: 820px; height: 180px;"
            >
                <div id="heatmap" style="width: 100%; height: 100%;"></div>
            </div>
            <div class="flex flex-col items-center gap-2 rounded-md border border-border bg-card px-6 py-3">
                <span class="font-mono text-sm text-muted-foreground">
                    {move || status_text.get()}
                </span>
            </div>
        </div>
        */
    }
}
