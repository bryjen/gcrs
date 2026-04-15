use leptos::prelude::*;
use leptos_icons::Icon;

use icondata as i;

use crate::components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

#[island]
pub fn RepoOverview(is_owner: bool) -> impl IntoView {
    let tab_configs = vec![
        ("preview", i::CgReadme, "README", true),
        ("codeofconduct", i::TbHeartHandshakeOutline, "Code of conduct", true),
        ("contributing", i::ChPeople, "Contributing", is_owner),
        ("license", i::OcLawLg, "MIT License", is_owner),
        ("security", i::MdiSecurity, "Security", is_owner),
    ];

    let visible_tabs: Vec<_> = tab_configs
        .into_iter()
        .filter(|t| t.3)
        .map(|(val, icon, label, _)| (val.to_string(), icon, label))
        .collect();

    let triggers = visible_tabs.clone();

    view! {
        <Tabs default_value="preview" class="w-full">
            <TabsList variant=TabsVariant::Line>
                {triggers.into_iter().map(|(val, icon, label)| {
                    view! {
                        <TabsTrigger value={val}>
                            <div class="mx-1 flex gap-2">
                                <Icon icon={icon} width="20" height="20"/>
                                {label}
                            </div>
                        </TabsTrigger>
                    }
                }).collect_view()}
            </TabsList>
            {visible_tabs.into_iter().map(|(val, _, _)| {
                let val_clone = val.clone();
                view! {
                    <TabsContent value={val}>
                        {render_tab_content(val_clone)}
                    </TabsContent>
                }
            }).collect_view()}
        </Tabs>
    }
}

fn render_tab_content(tab: String) -> AnyView {
    match tab.as_str() {
        "preview" => preview_content().into_any(),
        "codeofconduct" => code_of_conduct_content().into_any(),
        "contributing" => contributing_content().into_any(),
        "license" => license_content().into_any(),
        "security" => security_content().into_any(),
        _ => view! { <p>"unknown"</p> }.into_any(),
    }
}

fn preview_content() -> impl IntoView {
    view! { <p>"preview"</p> }
}

fn code_of_conduct_content() -> impl IntoView {
    view! { <p>"code of conduct"</p> }
}

fn contributing_content() -> impl IntoView {
    view! { <p>"contributing"</p> }
}

fn license_content() -> impl IntoView {
    view! { <p>"license"</p> }
}

fn security_content() -> impl IntoView {
    view! { <p>"security"</p> }
}
