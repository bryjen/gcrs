use chrono::{Duration, Local, NaiveDate, NaiveDateTime, Utc};
use leptos::prelude::*;
use leptos_icons::Icon;

use icondata as i;

use crate::components::ui::avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[derive(Clone)]
pub struct Item {
    pub is_dir: bool,
    pub name: String,
    pub desc: String, // generally msg of last commit that modifies it
    pub last_mod: NaiveDateTime,
}

#[component]
pub fn RepoTable() -> impl IntoView {
    let items = get_sample_items();
    view! {
        <table class="w-full">
            <thead>
                // empty
            </thead>
            <tbody>
                // header
                // in body, but to match styling and gh (this is what they do)
                <tr class="border-b border-border hover:bg-muted/50">
                    <td colspan="4" class="py-3 px-4 max-w-0 w-full">
                        <div class="flex justify-between items-center min-w-0">
                            <div class="flex items-center gap-2 min-w-0 overflow-hidden text-muted-foreground">
                                <Avatar size=AvatarSize::Sm>
                                    <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                                </Avatar>
                                <a href="" target="_blank" class="text-sm text-foreground font-bold hover:underline hover:text-primary shrink-0">
                                    "DanikVitek"
                                </a>
                                <a href="" target="_blank" class="truncate text-sm font-thin hover:underline hover:text-primary/90">
                                    "feat(either_of): Add Either! convenience macro (#4627)"
                                </a>
                            </div>
                            <div class="flex items-center gap-2 shrink-0 pl-4">
                                <p class="text-xs text-muted-foreground">
                                    <a href="" target="_blank" class="hover:underline hover:text-primary whitespace-nowrap">
                                        "59c2631"
                                    </a>
                                    " · 4 days ago"
                                </p>
                                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline>
                                    <div class="flex gap-1 text-xs">
                                        <Icon icon={i::ChGitCommit} width="16" height="16"/>
                                        "5,245 Commits"
                                    </div>
                                </Button>
                            </div>
                        </div>
                    </td>
                </tr>

                <For
                    each=move || items.clone().into_iter().enumerate()
                    key=|(i, _)| *i
                    children=move |(_, item)| view! {
                        {render_item(item)}
                    }
                />
            </tbody>
        </table>
    }
}

fn render_item(item: Item) -> impl IntoView {
    let icon = if item.is_dir {
        i::OcFileDirectoryFillLg
    } else {
        i::BiFileBlankRegular
    };
    view! {
        <tr class="border-b border-border hover:bg-muted/50">
            <td class="py-3 pl-4 ">
                <Icon icon=icon width="16" height="16"/>
            </td>
            <td class="py-3 pl-2 pr-4 text-sm">
                <a href="" target="_blank" class="hover:underline hover:text-primary">
                    {item.name}
                </a>
            </td>
            <td class="py-3 px-4 text-sm font-thin text-muted-foreground">
                <a href="" target="_blank" class="hover:underline hover:text-primary/90">
                    {item.desc}
                </a>
            </td>
            <td class="py-3 px-4 text-sm font-thin text-muted-foreground text-right whitespace-nowrap">
                {time_ago(item.last_mod)}
            </td>
        </tr>
    }
}

fn time_ago(dt: NaiveDateTime) -> String {
    let now = Local::now().naive_local();
    let diff = now.signed_duration_since(dt);

    let secs = diff.num_seconds();
    let mins = diff.num_minutes();
    let hours = diff.num_hours();
    let days = diff.num_days();
    let months = days / 30;
    let years = days / 365;

    if years > 0 {
        format!("{} year{} ago", years, if years == 1 { "" } else { "s" })
    } else if months > 0 {
        format!("{} month{} ago", months, if months == 1 { "" } else { "s" })
    } else if days > 0 {
        format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
    } else if hours > 0 {
        format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
    } else if mins > 0 {
        format!("{} minute{} ago", mins, if mins == 1 { "" } else { "s" })
    } else {
        format!("{} second{} ago", secs, if secs == 1 { "" } else { "s" })
    }
}

pub fn get_sample_items() -> Vec<Item> {
    let now = Utc::now().naive_utc();
    vec![
        // --- DIRECTORIES ---
        Item {
            is_dir: true,
            name: ".config".into(),
            desc: "feat: provide tower services for serving static files (#4528)".into(),
            last_mod: now - Duration::days(90), // 3 months ago
        },
        Item {
            is_dir: true,
            name: ".github".into(),
            desc: "chore(deps): bump pnpm/action-setup from 4 to 5 (#4658)".into(),
            last_mod: now - Duration::days(4), // 4 days ago
        },
        Item {
            is_dir: true,
            name: "any_error".into(),
            desc: "chore: publish patch versions".into(),
            last_mod: now - Duration::days(180), // 6 months ago
        },
        Item {
            is_dir: true,
            name: "any_spawner".into(),
            desc: "chore: unify all deps + exact versioning in root workspace for...".into(),
            last_mod: now - Duration::days(330), // 11 months ago
        },
        Item {
            is_dir: true,
            name: "benchmarks".into(),
            desc: "change: set MSRV to 1.88 (proc-macro spans stabilized = sta...".into(),
            last_mod: now - Duration::days(270), // 9 months ago
        },
        Item {
            is_dir: true,
            name: "cargo-make".into(),
            desc: "fix(CI): the proposed cargo-all-features's feature is already m...".into(),
            last_mod: now - Duration::days(60), // 2 months ago
        },
        Item {
            is_dir: true,
            name: "const_str_slice_concat".into(),
            desc: "Fix spelling typos. (#3965)".into(),
            last_mod: now - Duration::days(330), // 11 months ago
        },
        Item {
            is_dir: true,
            name: "docs".into(),
            desc: "chore: update syntax in COMMON_BUGS.md doc".into(),
            last_mod: now - Duration::days(90), // 3 months ago
        },
        Item {
            is_dir: true,
            name: "either_of".into(),
            desc: "feat(either_of): Add Either! convenience macro (#4627)".into(),
            last_mod: now - Duration::days(4), // 4 days ago
        },
        Item {
            is_dir: true,
            name: "examples".into(),
            desc: "chore: use #[patch] in example".into(),
            last_mod: now - Duration::days(30), // last month
        },
        Item {
            is_dir: true,
            name: "hydration_context".into(),
            desc: "Removed crate once_cell (#4083)".into(),
            last_mod: now - Duration::days(300), // 10 months ago
        },
        Item {
            is_dir: true,
            name: "integrations".into(),
            desc: "feat: implement separate css_path and css_file_path and fix ...".into(),
            last_mod: now - Duration::days(30), // last month
        },
        Item {
            is_dir: true,
            name: "leptos".into(),
            desc: "chore: be consistent about requiring both nightly feature and ...".into(),
            last_mod: now - Duration::days(30), // last month
        },
        Item {
            is_dir: true,
            name: "leptos_config".into(),
            desc: "chore: typo in doc comment".into(),
            last_mod: now - Duration::weeks(3),
        },
        Item {
            is_dir: true,
            name: "leptos_dom".into(),
            desc: "chore: publish patch versions".into(),
            last_mod: now - Duration::days(60), // 2 months ago
        },
        Item {
            is_dir: true,
            name: "leptos_hot_reload".into(),
            desc: "chore: publish patch versions".into(),
            last_mod: now - Duration::days(60), // 2 months ago
        },
        Item {
            is_dir: true,
            name: "leptos_macro".into(),
            desc: "chore: be consistent about requiring both nightly feature and ...".into(),
            last_mod: now - Duration::days(30), // last month
        },
        Item {
            is_dir: true,
            name: "leptos_server".into(),
            desc: "chore: publish patch versions".into(),
            last_mod: now - Duration::days(60), // 2 months ago
        },
        Item {
            is_dir: true,
            name: "logos".into(),
            desc: "Add simple icon logo (#468)".into(),
            last_mod: now - Duration::days(1095), // 3 years ago
        },
        Item {
            is_dir: true,
            name: "meta".into(),
            desc: "chore: publish patch versions".into(),
            last_mod: now - Duration::days(60), // 2 months ago
        },
        Item {
            is_dir: true,
            name: "next_tuple".into(),
            desc: "v0.7.0".into(),
            last_mod: now - Duration::days(730), // 2 years ago
        },
        Item {
            is_dir: true,
            name: "oco".into(),
            desc: "chore: bump oco_ref version number (#4168)".into(),
            last_mod: now - Duration::days(270), // 9 months ago
        },
        Item {
            is_dir: true,
            name: "or_poisoned".into(),
            desc: "fix: relax bounds on OrPoisoned blanket impls".into(),
            last_mod: now - Duration::days(365), // last year
        },
        Item {
            is_dir: true,
            name: "projects".into(),
            desc: "chore(deps): bump playwright (#4399)".into(),
            last_mod: now - Duration::days(180), // 6 months ago
        },
        Item {
            is_dir: true,
            name: "reactive_graph".into(),
            desc: "chore: be consistent about requiring both nightly feature and ...".into(),
            last_mod: now - Duration::days(30), // last month
        },
        Item {
            is_dir: true,
            name: "reactive_stores".into(),
            desc: "chore: allocate correct capacity when patching HashMaps".into(),
            last_mod: now - Duration::days(30), // last month
        },
        Item {
            is_dir: true,
            name: "reactive_stores_macro".into(),
            desc: "chore: add #[allow(missing_docs)] and #[automatically_deri...".into(),
            last_mod: now - Duration::weeks(2),
        },
        Item {
            is_dir: true,
            name: "router".into(),
            desc: "leptos_router-v0.8.13".into(),
            last_mod: now - Duration::weeks(3),
        },
        Item {
            is_dir: true,
            name: "router_macro".into(),
            desc: "chore: publish patch versions".into(),
            last_mod: now - Duration::days(180), // 6 months ago
        },
        Item {
            is_dir: true,
            name: "scripts".into(),
            desc: "feat: provide tower services for serving static files (#4528)".into(),
            last_mod: now - Duration::days(90), // 3 months ago
        },
        Item {
            is_dir: true,
            name: "server_fn".into(),
            desc: "fix: adjust trybuild stderr based on the latest nightly toolchain".into(),
            last_mod: now - Duration::days(30), // last month
        },
        Item {
            is_dir: true,
            name: "server_fn_macro".into(),
            desc: "chore: publish patch versions".into(),
            last_mod: now - Duration::days(60), // 2 months ago
        },
        Item {
            is_dir: true,
            name: "tachys".into(),
            desc: "chore: be consistent about requiring both nightly feature and ...".into(),
            last_mod: now - Duration::days(30), // last month
        },
        // --- FILES ---
        Item {
            is_dir: false,
            name: ".gitignore".into(),
            desc: "Remove hash.txt from tracking, update dependencies in tests".into(),
            last_mod: now - Duration::days(730), // 2 years ago
        },
        Item {
            is_dir: false,
            name: "ARCHITECTURE.md".into(),
            desc: "docs: typos".into(),
            last_mod: now - Duration::days(1095), // 3 years ago
        },
        Item {
            is_dir: false,
            name: "CODE_OF_CONDUCT.md".into(),
            desc: "fix: memo with_untracked (#1213)".into(),
            last_mod: now - Duration::days(1095), // 3 years ago
        },
        Item {
            is_dir: false,
            name: "CONTRIBUTING.md".into(),
            desc: "feat: allow spreading of both attributes and event handlers (...".into(),
            last_mod: now - Duration::days(730), // 2 years ago
        },
        Item {
            is_dir: false,
            name: "Cargo.lock".into(),
            desc: "leptos_router-v0.8.13".into(),
            last_mod: now - Duration::weeks(3),
        },
        Item {
            is_dir: false,
            name: "Cargo.toml".into(),
            desc: "chore: publish patch versions".into(),
            last_mod: now - Duration::days(60), // 2 months ago
        },
        Item {
            is_dir: false,
            name: "LICENSE".into(),
            desc: "Initial commit".into(),
            last_mod: now - Duration::days(1460), // 4 years ago
        },
        Item {
            is_dir: false,
            name: "Makefile.toml".into(),
            desc: "Introducing cargo-all-features; clippy|nextest part of build ...".into(),
            last_mod: now - Duration::days(365), // last year
        },
        Item {
            is_dir: false,
            name: "README.md".into(),
            desc: "docs: document getrandom/rand config for wasm32 (#4648)".into(),
            last_mod: now - Duration::days(30), // last month
        },
        Item {
            is_dir: false,
            name: "SECURITY.md".into(),
            desc: "chore: create SECURITY.MD".into(),
            last_mod: now - Duration::days(1095), // 3 years ago
        },
        Item {
            is_dir: false,
            name: "TODO.md".into(),
            desc: "work on routing".into(),
            last_mod: now - Duration::days(730), // 2 years ago
        },
        Item {
            is_dir: false,
            name: "flake.lock".into(),
            desc: "feat: allow using different error types for req/resp with WebS...".into(),
            last_mod: now - Duration::days(365), // last year
        },
        Item {
            is_dir: false,
            name: "flake.nix".into(),
            desc: "feat: allow using different error types for req/resp with WebS...".into(),
            last_mod: now - Duration::days(365), // last year
        },
        Item {
            is_dir: false,
            name: "rustfmt.toml".into(),
            desc: "feat: allow spreading of both attributes and event handlers (...".into(),
            last_mod: now - Duration::days(730), // 2 years ago
        },
    ]
}
