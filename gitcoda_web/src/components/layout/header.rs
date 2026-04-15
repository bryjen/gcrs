use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerPosition, DrawerTitle,
    DrawerTrigger, DrawerVariant,
};
use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

// ── Row 1 sub-components ──────────────────────────────────────────────────────

#[component]
fn GlobalBreadcrumb() -> impl IntoView {
    view! {
        <nav class="flex items-center gap-1 text-sm">
            <a href="/" class="font-semibold text-muted-foreground hover:text-foreground">"username"</a>
            <span class="text-muted-foreground/40">"/"</span>
            <a href="/" class="font-semibold text-foreground hover:underline">"repo-name"</a>
        </nav>
    }
}

#[component]
fn GlobalSearch() -> impl IntoView {
    view! {
        <div class="hidden flex-1 max-w-sm md:flex">
            <div class="relative flex w-full items-center">
                <input
                    type="text"
                    placeholder="Search or jump to..."
                    class="w-full rounded-md border border-border bg-transparent px-3 py-1 text-sm
                           text-foreground placeholder:text-muted-foreground/60
                           focus:border-ring focus:outline-none focus:ring-1 focus:ring-ring"
                />
                <div class="pointer-events-none absolute right-2">
                    <kbd class="rounded border border-border px-1.5 py-0.5 text-[11px] text-muted-foreground">"/"</kbd>
                </div>
            </div>
        </div>
    }
}

#[component]
fn GlobalUserActions() -> impl IntoView {
    view! {
        <div class="flex items-center gap-0.5">
            // Notification bell
            <button class="relative rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-foreground">
                <Icon icon=i::BsBell width="16" height="16" />
                <span class="absolute right-1 top-1 h-1.5 w-1.5 rounded-full bg-orange-500" />
            </button>
            // New (+) button
            <button class="flex items-center gap-0.5 rounded-md px-1.5 py-1.5 text-muted-foreground hover:bg-accent hover:text-foreground">
                <svg width="16" height="16" viewBox="0 0 16 16" class="fill-current">
                    <path d="M7.75 2a.75.75 0 0 1 .75.75V7h4.25a.75.75 0 0 1 0 1.5H8.5v4.25a.75.75 0 0 1-1.5 0V8.5H2.75a.75.75 0 0 1 0-1.5H7V2.75A.75.75 0 0 1 7.75 2Z" />
                </svg>
                <svg width="10" height="10" viewBox="0 0 10 10" class="fill-current opacity-60">
                    <path d="M0 3l5 5 5-5z" />
                </svg>
            </button>
            // Avatar
            <button class="ml-0.5 flex items-center rounded-full border border-border hover:border-ring">
                <div class="flex h-7 w-7 items-center justify-center rounded-full bg-muted text-xs font-medium text-foreground">
                    "U"
                </div>
            </button>
        </div>
    }
}

// ── Row 2 sub-components ──────────────────────────────────────────────────────

#[component]
fn NavDrawer() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger
                class="border-border text-muted-foreground hover:text-foreground"
                variant=ButtonVariant::Outline
                size=ButtonSize::Icon
            >
                <Icon icon=i::CgMenuLeft width="18" height="18" />
            </DrawerTrigger>

            <DrawerContent
                position=DrawerPosition::Left
                variant=DrawerVariant::Floating
                class="top-0 bottom-0 left-0 right-auto h-screen w-[280px] rounded-none border-r border-border px-0 pt-0 pb-0 max-h-none"
                style="--initial-transform: 100%;"
            >
                <div class="flex h-full flex-col bg-card">
                    <div class="flex items-center justify-between border-b px-4 py-3">
                        <div class="flex items-center gap-3">
                            <div class="flex size-8 items-center justify-center rounded-md bg-foreground text-background">
                                <Icon icon=i::BsGithub width="18" height="18" />
                            </div>
                            <div class="flex flex-col leading-none">
                                <DrawerTitle>"GitCoda"</DrawerTitle>
                                <DrawerDescription>"Repository navigation"</DrawerDescription>
                            </div>
                        </div>
                        <DrawerClose
                            class="text-muted-foreground"
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Icon
                        >
                            <Icon icon=i::IoClose width="18" height="18" />
                        </DrawerClose>
                    </div>

                    <DrawerBody class="mx-0 max-w-none flex-1 gap-0 px-3 py-3">
                        <nav class="flex flex-col gap-1">
                            <a href="/" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent">
                                <Icon icon=i::BiHomeAlt2Regular width="18" height="18" />
                                <span>"Overview"</span>
                            </a>
                            <a href="/" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground">
                                <Icon icon=i::LuGitBranch width="18" height="18" />
                                <span>"Branches"</span>
                            </a>
                            <a href="/" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground">
                                <Icon icon=i::BsGraphUp width="18" height="18" />
                                <span>"Insights"</span>
                            </a>
                            <a href="/" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground">
                                <Icon icon=i::BsActivity width="18" height="18" />
                                <span>"Integrations"</span>
                            </a>
                            <a href="/" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground">
                                <Icon icon=i::MdiSecurity width="18" height="18" />
                                <span>"Settings"</span>
                            </a>
                        </nav>
                    </DrawerBody>
                </div>
            </DrawerContent>
        </Drawer>
    }
}

#[derive(Clone)]
struct RepoTab {
    label: &'static str,
    count: Option<u32>,
}

#[component]
fn RepoTabStrip() -> impl IntoView {
    let tabs = vec![
        RepoTab {
            label: "Code",
            count: None,
        },
        RepoTab {
            label: "Issues",
            count: Some(3),
        },
        RepoTab {
            label: "Pull requests",
            count: Some(2),
        },
        RepoTab {
            label: "Actions",
            count: None,
        },
        RepoTab {
            label: "Projects",
            count: None,
        },
        RepoTab {
            label: "Wiki",
            count: None,
        },
        RepoTab {
            label: "Security",
            count: None,
        },
        RepoTab {
            label: "Insights",
            count: None,
        },
        RepoTab {
            label: "Settings",
            count: None,
        },
    ];

    view! {
        <nav class="flex items-end gap-0 overflow-x-auto">
            {tabs.into_iter().map(|tab| view! {
                <a
                    href="/"
                    class="flex items-center gap-1.5 border-b-2 border-transparent px-3 py-2 font-bold text-sm text-muted-foreground hover:border-border hover:text-foreground whitespace-nowrap"
                >
                    {tab.label}
                    {tab.count.map(|n| view! {
                        <span class="rounded-full bg-card px-1.5 py-0.5 text-[11px] leading-none text-primary">
                            {n}
                        </span>
                    })}
                </a>
            }).collect_view()}
        </nav>
    }
}

// ── Public component ──────────────────────────────────────────────────────────

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="z-50 border-b border-border bg-card">
            // Row 1 — global nav
            <div class="flex items-center gap-4 px-6 pt-4 pb-2">
                <div class="flex items-center gap-6">
                    <NavDrawer />
                    <Icon icon=i::ImGit width="20" height="20"/>
                    <GlobalBreadcrumb />
                </div>
                <div class="ml-auto flex items-center gap-5">
                    <GlobalSearch />
                    <GlobalUserActions />
                </div>
            </div>

            // Row 2 — repo tab strip
            <div class="flex items-center gap-4 px-6">
                <RepoTabStrip />
            </div>
        </header>
    }
}
