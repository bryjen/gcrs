use crate::components::custom::heatmap::{ActivityHeatmap, HeatmapData};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let heatmap_data = generate_heatmap_data();

    view! {
        <div class="flex flex-col gap-6 py-12 pt-8 pb-[50vh]">
            <ActivityHeatmap data=heatmap_data />
        </div>
    }
}

fn generate_heatmap_data() -> Vec<HeatmapData> {
    let mut data = Vec::new();

    for day in 0..365 {
        let month = (day / 31) + 1;
        let day_of_month = (day % 31) + 1;
        let date = format!("2026-{:02}-{:02}", month, day_of_month);

        // Simple pseudorandom: spread activity across days
        let seed = (day * 73 + 17) as u32;
        let count = ((seed ^ (seed >> 16)) % 300) + (day as u32 % 50);

        if count > 20 {
            data.push(HeatmapData { date, count });
        }
    }

    data
}
