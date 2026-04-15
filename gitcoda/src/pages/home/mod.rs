use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

mod lang_bar;
use lang_bar::{LanguageBar, RepoLanguageBar};

mod repo_table;
use repo_table::RepoTable;

mod repo_overview;
use repo_overview::RepoOverview;

use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::ui::avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
};
use crate::components::ui::badge::Badge;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuTrigger,
};
use crate::components::ui::input::Input;
use crate::components::ui::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectOption, SelectTrigger,
    SelectValue,
};
use crate::components::ui::separator::Separator;

#[component]
fn RenderMain() -> impl IntoView {
    const OPTIONS: [&str; 3] = ["Components", "Extensions", "Icons"];

    view! {
        <div class="w-full max-w-full flex flex-col gap-6">

            <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <Select>
                        <SelectTrigger class="w-[150px]">
                            <SelectValue placeholder="Please select" />
                        </SelectTrigger>

                        <SelectContent>
                            <SelectGroup>
                                {OPTIONS
                                    .into_iter()
                                    .map(|option| {
                                        view! { <SelectOption value=option>{option}</SelectOption> }
                                    })
                                    .collect_view()}
                            </SelectGroup>
                        </SelectContent>
                    </Select>

                    <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost>
                        "Sponsor"
                    </Button>

                    <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost>
                        "Tags"
                    </Button>
                </div>

                <div class="flex items-center gap-2">
                    <Input placeholder="Enter text..." />

                    <DropdownMenu>
                        <DropdownMenuTrigger>"Open Menu"</DropdownMenuTrigger>
                        <DropdownMenuContent>
                            <DropdownMenuLabel>"Menu Label"</DropdownMenuLabel>
                            <DropdownMenuGroup>
                                <DropdownMenuItem>
                                    <DropdownMenuAction>"Action 1"</DropdownMenuAction>
                                </DropdownMenuItem>
                                <DropdownMenuItem>
                                    <DropdownMenuAction>"Action 2"</DropdownMenuAction>
                                </DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuContent>
                    </DropdownMenu>

                    <Button size=ButtonSize::Sm variant=ButtonVariant::Default>
                        "Tags"
                    </Button>
                </div>
            </div>

            <RepoTable />
            <RepoOverview is_owner=true />

        </div>
    }
}

fn render_icon(icon: icondata::Icon, text: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <a href=href target="_blank" class="group flex items-center gap-2 w-fit text-muted-foreground">
            <Icon icon=icon width="20" height="20" attr:class="group-hover:text-primary"/>
            <span class="text-sm group-hover:text-primary">{text}</span>
        </a>
    }
}

#[component]
fn RenderSidePanel() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 w-full">
            <div class="flex flex-col gap-4">
                /* about */
                <div>
                    <h2>"About"</h2>
                    <span>"Build fast web applications with Rust."</span>
                </div>

                <div class="flex items-center gap-2">
                    <Icon icon={i::AiLinkOutlined} width="22" height="22"/>
                    <a href="https://leptos.dev/" target="_blank">"leptos.dev"</a>
                </div>

                <div class="flex flex-row flex-wrap gap-2 my-2">
                    <For
                        each=move || 1..=10
                        key=|i| *i
                        children=move |i| view! {
                            <Badge>"Badge "{i}</Badge>
                        }
                    />
                </div>

                <div class="flex flex-col gap-2 mt-4">
                    {render_icon(i::CgReadme, "Readme", "")}
                    {render_icon(i::OcLawLg, "MIT License", "")}
                    {render_icon(i::TbHeartHandshakeOutline, "Code of conduct", "")}
                    {render_icon(i::ChPeople, "Contributing", "")}
                    {render_icon(i::MdiSecurity, "Security", "")}
                    {render_icon(i::BsActivity, "Activity", "")}
                    {render_icon(i::AiStarOutlined, "20.1k stars", "")}
                    {render_icon(i::BsEye, "100 watching", "")}
                    {render_icon(i::LuGitFork, "857 forks", "")}
                    {render_icon(i::TbMessageReportOutline, "Report repository", "")}
                </div>

                <Separator class="mt-4 mb-2" />

                /* releases */
                <div class="flex items-center gap-2">
                    <h2>"Releases"</h2>
                    <Badge class="h-fit">"67"</Badge>
                </div>

                <a href="" target="_blank" class="group flex items-start gap-2 w-fit text-muted-foreground">
                    <Icon icon=i::BiPurchaseTagRegular width="24" height="24" attr:class="w-fit shrink-0 group-hover:text-primary"/>
                    <div class="flex flex-col flex-1 text-start">
                        <p class="font-bold leading-none">
                            <span class="text-sm group-hover:text-primary">"v0.7.18"</span>
                            <span class="ml-2 -mt-2 text-primary text-sm group-hover:brightness-125">"latest"</span>
                        </p>
                        <span class="text-xs text-muted-foreground leading-none group-hover:text-primary/75">"on Apr 1"</span>
                    </div>
                </a>

                <a href="" target="_blank" class="mt-4 text-sm text-muted-foreground hover:text-primary">
                    "+ 66 other releases text-sm"
                </a>

                <Separator class="mt-4 mb-2" />

                /* sponsor */
                <h2>"Sponsor this project"</h2>

                <div class="flex items-center text-sm gap-2">
                    <Avatar>
                        <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                    </Avatar>
                    <span>
                        "gbj"
                    </span>
                    <span class="text-muted-foreground">
                        "Greg Johnston"
                    </span>
                </div>

                <div class="flex flex-col gap-2">
                    <Button size=ButtonSize::Sm variant=ButtonVariant::Secondary>
                        <Icon icon=i::AiHeartOutlined width="16" height="16" attr:class="mr-2"/>
                        "Sponsor"
                    </Button>

                    <a href="" target="_blank" class="text-[0.7rem] text-muted-foreground hover:text-primary">
                        "Learn more about sponsoring on GitCoda"
                    </a>
                </div>

                <Separator class="mt-4 mb-2" />

                /* contributors */
                <div class="flex items-center gap-2">
                    <h2>"Contributors"</h2>
                    <Badge class="h-fit">"368"</Badge>
                </div>

                <div class="flex flex-row flex-wrap gap-2 my-2">
                    <For
                        each=move || 1..=25
                        key=|i| *i
                        children=move |i| view! {
                            <Avatar class="hover:border-2 hover:border-primary">
                                <AvatarImage attr:src=format!("https://api.dicebear.com/9.x/notionists/svg?seed={i}") attr:alt="@rustify" />
                            </Avatar>
                        }
                    />
                </div>

                <a href="" target="_blank" class="mt-4 text-sm text-muted-foreground hover:text-primary">
                    "View all contributors"
                </a>


            </div>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let langs = vec![
        LanguageBar {
            label: "Rust".into(),
            percent: 89.4,
            color: "#c97b4b".into(),
        },
        LanguageBar {
            label: "Nix".into(),
            percent: 10.6,
            color: "#6e76d4".into(),
        },
    ];
    view! {
        <div class="flex flex-col gap-6 py-12 pt-8 pb-[50vh]">

            <div id="repo-header" class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <Avatar size=AvatarSize::Lg>
                        <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                    </Avatar>
                    <span class="text-lg">
                        "leptos"
                    </span>
                </div>

                <div class="flex items-center gap-2">
                    <Button size=ButtonSize::Sm variant=ButtonVariant::Secondary>"Sponsor"</Button>
                    <Button size=ButtonSize::Sm variant=ButtonVariant::Secondary>"Watch"</Button>
                    <Button size=ButtonSize::Sm variant=ButtonVariant::Secondary>"Fork"</Button>
                    <Button size=ButtonSize::Sm variant=ButtonVariant::Secondary>"Star"</Button>
                </div>
            </div>

            <RepoLanguageBar languages=langs />

            <div id="repo-contents" class="grid grid-cols-[72.5%_27.5%] gap-12">
                <RenderMain />
                <RenderSidePanel />
            </div>

        </div>
    }
}
