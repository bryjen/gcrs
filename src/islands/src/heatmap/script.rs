/// Build echarts initialization script with heatmap data and styling
#[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
pub fn build_echarts_script(
    data: &[crate::heatmap::HeatmapData],
    selected_date: Option<String>,
) -> String {
    // Build complete year with all dates, fill missing with 0
    let mut date_map: std::collections::HashMap<String, u32> =
        data.iter().map(|d| (d.date.clone(), d.count)).collect();

    let mut all_dates = Vec::new();
    for day in 0..365 {
        let month = (day / 31) + 1;
        let day_of_month = (day % 31) + 1;
        let date = format!("2026-{:02}-{:02}", month, day_of_month);
        let count = date_map.remove(&date).unwrap_or(0);
        all_dates.push((date, count));
    }

    let data_json = all_dates
        .iter()
        .map(|(d, c)| format!(r#"["{}",{}]"#, d, c))
        .collect::<Vec<_>>()
        .join(",");

    let max_count = all_dates.iter().map(|(_, c)| *c).max().unwrap_or(1);

    let selected_date_json = selected_date
        .as_ref()
        .map(|d| format!(r#""{}""#, d))
        .unwrap_or_else(|| "null".to_string());

    format!(
        r#"
        (function() {{
            const chartDom = document.getElementById('heatmap');
            if (!chartDom) return;

            const myChart = echarts.init(chartDom);
            let selectedDate = {selected_date_json};

            const rawData = [
                {data_json}
            ];

            // Activity gradient
            const colors = [
                '#242422',  // empty/base
                '#3B3B38',  // step-1
                '#46433C',  // step-2
                '#514E41',  // step-3
                '#5C5848',  // step-4
                '#9E9669',  // step-5
                '#DFCA8A'   // step-6
            ];

            function getAnimatedData(selection) {{
                return rawData.map(item => {{
                    const isSelected = selection && item[0] === selection;
                    return {{
                        value: item,
                        itemStyle: {{
                            color: isSelected ? '#DFCA8A' : undefined,
                            opacity: isSelected ? 1 : (selection ? 0.15 : 1)
                        }}
                    }};
                }});
            }}

            const option = {{
                backgroundColor: 'transparent',
                animation: true,
                animationDurationUpdate: 300,
                animationEasingUpdate: 'quadraticOut',
                tooltip: {{
                    show: true,
                    trigger: 'item',
                    backgroundColor: 'rgba(59, 59, 56, 1)',
                    borderColor: '#2D2B28',
                    borderWidth: 1,
                    textStyle: {{ color: '#F8F7F2', fontSize: 12 }},
                    formatter: (p) => `<strong>${{p.data.value[1]}}</strong> activities on ${{p.data.value[0]}}`
                }},
                visualMap: {{
                    show: false,
                    min: 0,
                    max: {max_count},
                    type: 'piecewise',
                    pieces: [
                        {{ min: 0, max: 0, color: '#242422' }},
                        {{ min: 1, max: Math.ceil({max_count} * 0.166), color: '#3B3B38' }},
                        {{ min: Math.ceil({max_count} * 0.166) + 1, max: Math.ceil({max_count} * 0.333), color: '#46433C' }},
                        {{ min: Math.ceil({max_count} * 0.333) + 1, max: Math.ceil({max_count} * 0.5), color: '#514E41' }},
                        {{ min: Math.ceil({max_count} * 0.5) + 1, max: Math.ceil({max_count} * 0.666), color: '#5C5848' }},
                        {{ min: Math.ceil({max_count} * 0.666) + 1, max: Math.ceil({max_count} * 0.833), color: '#9E9669' }},
                        {{ min: Math.ceil({max_count} * 0.833) + 1, max: {max_count}, color: '#DFCA8A' }}
                    ]
                }},
                calendar: {{
                    top: 35,
                    bottom: 50,
                    left: 30,
                    right: 15,
                    cellSize: 100,
                    range: '2026',
                    itemStyle: {{
                        color: '#242422',
                        borderWidth: 3,
                        borderColor: '#141412',
                        borderRadius: 5
                    }},
                    emptyItemStyle: {{
                        color: '#242422'
                    }},
                    splitLine: {{ show: false }},
                    dayLabel: {{ show: false }},
                    monthLabel: {{
                        nameMap: 'en',
                        color: '#f6f6f6',
                        fontSize: 11,
                        margin: 10
                    }},
                    yearLabel: {{ show: false }}
                }},
                graphic: [
                    {{
                        type: 'text',
                        left: 30,
                        bottom: 20,
                        style: {{
                            text: rawData.length + ' total activities in 2026',
                            fill: '#f6f6f6',
                            font: '500 11px system-ui'
                        }}
                    }},
                    {{
                        type: 'group',
                        right: 15,
                        bottom: 20,
                        children: [
                            {{ type: 'text', left: 0, top: 2, style: {{ text: 'Less', fill: '#f6f6f6', font: '10px system-ui' }} }},
                            {{ type: 'rect', left: 32, shape: {{ width: 8, height: 8, r: 1 }}, style: {{ fill: '#242422' }} }},
                            {{ type: 'rect', left: 43, shape: {{ width: 8, height: 8, r: 1 }}, style: {{ fill: '#3B3B38' }} }},
                            {{ type: 'rect', left: 54, shape: {{ width: 8, height: 8, r: 1 }}, style: {{ fill: '#46433C' }} }},
                            {{ type: 'rect', left: 65, shape: {{ width: 8, height: 8, r: 1 }}, style: {{ fill: '#514E41' }} }},
                            {{ type: 'rect', left: 76, shape: {{ width: 8, height: 8, r: 1 }}, style: {{ fill: '#5C5848' }} }},
                            {{ type: 'rect', left: 87, shape: {{ width: 8, height: 8, r: 1 }}, style: {{ fill: '#9E9669' }} }},
                            {{ type: 'rect', left: 98, shape: {{ width: 8, height: 8, r: 1 }}, style: {{ fill: '#DFCA8A' }} }},
                            {{ type: 'text', left: 111, top: 2, style: {{ text: 'More', fill: '#f6f6f6', font: '10px system-ui' }} }}
                        ]
                    }}
                ],
                series: {{
                    type: 'heatmap',
                    coordinateSystem: 'calendar',
                    data: getAnimatedData(selectedDate),
                    animation: true
                }}
            }};

            myChart.setOption(option);

            myChart.on('click', function(params) {{
                if (params.componentType === 'series') {{
                    const date = params.data.value[0];
                    const count = params.data.value[1];

                    if (selectedDate === date) {{
                        selectedDate = null;
                    }} else {{
                        selectedDate = date;
                    }}

                    window.__heatmapOnClick(date, count);

                    myChart.setOption({{
                        series: [{{ data: getAnimatedData(selectedDate) }}]
                    }});
                }}
            }});

            window.addEventListener('resize', () => {{
                if (myChart) myChart.resize();
            }});
        }})();
    "#
    )
}
