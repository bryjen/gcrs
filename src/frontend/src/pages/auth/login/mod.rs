use crate::pages::auth::{AuthLayout, server_fns::login};
use components::ui::input::{Input, InputType};
use components::ui::label::Label;
use leptos::prelude::*;
use leptos::web_sys;
use leptos_router::hooks::use_navigate;

#[component]
pub fn LoginPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let navigate = use_navigate();

    let login_action = Action::new(move |_: &()| {
        let email_val = email.get();
        let password_val = password.get();
        let nav = navigate.clone();
        async move {
            match login(email_val, password_val).await {
                Ok(_user) => {
                    nav("/", Default::default());
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        }
    });

    view! {
        <AuthLayout>
            <div class=crate::cls!("w-full max-w-sm")>
                <div class=crate::cls!("space-y-6")>
                    <div>
                        <h1 class=crate::cls!("text-3xl font-bold")>"Sign In"</h1>
                        <p class=crate::cls!("text-muted-foreground mt-2")>"Access your repositories"</p>
                    </div>

                    <form
                        on:submit=move |ev: web_sys::SubmitEvent| {
                            ev.prevent_default();
                            login_action.dispatch(());
                        }
                        class=crate::cls!("space-y-4")
                    >
                        <div class=crate::cls!("space-y-2")>
                            <Label html_for="email">"Email"</Label>
                            <Input id="email" r#type=InputType::Email placeholder="you@example.com" bind_value=email />
                        </div>

                        <div class=crate::cls!("space-y-2")>
                            <Label html_for="password">"Password"</Label>
                            <Input id="password" r#type=InputType::Password placeholder="••••••••" bind_value=password />
                        </div>

                        <button
                            type="submit"
                            disabled=login_action.pending()
                            class=crate::cls!("w-full px-4 py-2 rounded-md bg-primary text-primary-foreground font-medium hover:bg-primary/90 disabled:opacity-50")
                        >
                            {move || {
                                if login_action.pending()() {
                                    "Signing in..."
                                } else {
                                    "Sign in"
                                }
                            }}
                        </button>
                    </form>

                    <div class=crate::cls!("text-center text-sm")>
                        <a href="/signup" class=crate::cls!("text-primary hover:underline")>"Create account"</a>
                    </div>
                </div>
            </div>
        </AuthLayout>
    }
}
