use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

pub mod cool_effect;
pub mod login;
pub mod server_fns;
pub mod signup;

pub use cool_effect::PerlinNoiseBg;

#[component]
#[allow(non_snake_case)]
pub fn AuthLayout(children: Children) -> impl IntoView {
    view! {
        <main class="relative w-screen h-screen grid grid-cols-2 bg-background" data-vaul-drawer-wrapper>
            <a href="/" class="absolute z-10 flex items-center gap-2 text-lg text-foreground top-6 left-6 cursor-pointer underline-none">
                <Icon icon=i::BsGit width="32" height="32"/>
                <span class="text-xl font-semibold select-none">"GitCoda"</span>
            </a>

            <div class="relative h-full w-full bg-background flex items-center justify-center">
                {children()}
            </div>

            <div class="relative h-full w-full overflow-hidden pointer-events-none">
                <PerlinNoiseBg />
                /*
                <div class="absolute left-1/2 -translate-x-1/2 top-0">
                    "Centered horizontally, at the top"
                </div>
                */
            </div>
        </main>
    }
}
