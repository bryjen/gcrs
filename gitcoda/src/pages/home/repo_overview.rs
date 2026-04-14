use leptos::prelude::*;
use leptos_icons::Icon;

use icondata as i;

use crate::components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

#[derive(Clone)]
struct TabDef {
    value: &'static str,
    icon: icondata::Icon,
    label: &'static str,
    visible: bool,
    content: fn() -> AnyView,
}

#[component]
pub fn RepoOverview(is_owner: bool) -> impl IntoView {
    let tabs = vec![
        TabDef {
            value: "preview",
            icon: i::CgReadme,
            label: "README",
            visible: true,
            content: preview_content,
        },
        TabDef {
            value: "codeofconduct",
            icon: i::TbHeartHandshakeOutline,
            label: "Code of conduct",
            visible: true,
            content: code_content,
        },
        TabDef {
            value: "contributin",
            icon: i::ChPeople,
            label: "Contributing",
            visible: is_owner,
            content: settings_content,
        },
        TabDef {
            value: "license",
            icon: i::OcLawLg,
            label: "MIT License",
            visible: is_owner,
            content: settings_content,
        },
        TabDef {
            value: "security",
            icon: i::MdiSecurity,
            label: "Security",
            visible: is_owner,
            content: settings_content,
        },
    ];

    let visible_tabs: Vec<_> = tabs.into_iter().filter(|t| t.visible).collect();
    let visible_tabs_content = visible_tabs.clone();

    view! {
        <Tabs default_value="preview" class="w-full">
            <TabsList variant=TabsVariant::Line>
                {visible_tabs.into_iter().map(|t| view! {
                    <TabsTrigger value={t.value}>
                        <div class="mx-1 flex gap-2">
                            <Icon icon={t.icon} width="20" height="20"/>
                            {t.label}
                        </div>
                    </TabsTrigger>
                }).collect_view()}
            </TabsList>
            {visible_tabs_content.into_iter().map(|t| view! {
                <TabsContent value={t.value}>
                    {(t.content)()}
                </TabsContent>
            }).collect_view()}
        </Tabs>
    }
}

fn preview_content() -> AnyView {
    view! { <p>"preview"</p> }.into_any()
}

fn code_content() -> AnyView {
    view! { <p>"code"</p> }.into_any()
}

fn settings_content() -> AnyView {
    view! { <p>"settings"</p> }.into_any()
}
