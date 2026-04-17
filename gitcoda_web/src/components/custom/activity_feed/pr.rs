use crate::components::custom::activity_feed::shared::{PRIcon, ToggleButton};
use leptos::prelude::*;

#[component]
pub fn PREventRow(repo: String, branch: String, merge_count: u32, date: String) -> impl IntoView {
    view! {
        <div class="mb-10 pl-8 relative">
            <PRIcon />

            <div class="flex items-center justify-between text-foreground mb-2">
                <h3 class="text-base font-medium">
                    "Opened 1 pull request in 1 repository"
                </h3>
                <ToggleButton />
            </div>

            <div class="mb-3">
                <span class="text-muted-foreground text-sm">{repo}</span>
            </div>

            <div class="flex items-center justify-between text-sm">
                <div class="flex items-center gap-2">
                    <svg
                        class="text-accent w-4 h-4"
                        viewBox="0 0 16 16"
                        fill="currentColor"
                    >
                        <path d="M5 3.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Zm0 2.122a2.25 2.25 0 1 0-1.5 0v4.928a2.25 2.25 0 1 0 1.5.085V5.372Zm9.25 3.128a.75.75 0 1 0-1.5 0V11h-1.5V8.5a.75.75 0 0 0-.75-.75h-4.66l1.72-1.72a.75.75 0 1 0-1.06-1.06l-3 3a.75.75 0 0 0 0 1.06l3 3a.75.75 0 1 0 1.06-1.06l-1.72-1.72h3.91v2.25h1.5V11.5a.75.75 0 0 0 .75-.75v-2.25Z"></path>
                    </svg>
                    <a href="#" class="text-foreground hover:text-primary transition-colors font-medium hover:underline">
                        {branch}
                    </a>
                </div>
                <div class="flex items-center gap-3 text-xs">
                    <span class="text-muted-foreground flex items-center gap-1">
                        <span class="bg-[#8957e5]/20 text-accent px-2 py-0.5 rounded-full flex items-center gap-1 font-medium">
                            <span class="bg-accent rounded-full w-4 h-4 flex items-center justify-center text-white text-[10px]">
                                {merge_count}
                            </span>
                            " merged"
                        </span>
                    </span>
                    <span class="text-muted-foreground">{date}</span>
                </div>
            </div>
        </div>
    }
}
