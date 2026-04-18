use leptos::prelude::*;

pub mod cool_effect;
pub mod login;
pub mod signup;
pub mod server_fns;

pub use cool_effect::PerlinNoiseBg;

#[component]
#[allow(non_snake_case)]
pub fn AuthLayout(children: Children) -> impl IntoView {
    view! {
        <main class="relative w-screen h-screen grid grid-cols-2 bg-background" data-vaul-drawer-wrapper>
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
