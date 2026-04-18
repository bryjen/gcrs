use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::pages::auth::{AuthLayout, server_fns::signup};
use components::ui::input::{Input, InputType};
use components::ui::label::Label;
use leptos::web_sys;

#[component]
pub fn SignupPage() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let navigate = use_navigate();

    let signup_action = Action::new(move |_: &()| {
        let username_val = username.get();
        let email_val = email.get();
        let password_val = password.get();
        let nav = navigate.clone();
        async move {
            match signup(username_val, email_val, password_val).await {
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
                        <h1 class=crate::cls!("text-3xl font-bold")>"Create Account"</h1>
                        <p class=crate::cls!("text-muted-foreground mt-2")>"Start exploring repositories"</p>
                    </div>

                    <form
                        on:submit=move |ev: web_sys::SubmitEvent| {
                            ev.prevent_default();
                            signup_action.dispatch(());
                        }
                        class=crate::cls!("space-y-4")
                    >
                        <div class=crate::cls!("space-y-2")>
                            <Label html_for="username">"Username"</Label>
                            <Input id="username" placeholder="yourname" bind_value=username />
                        </div>

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
                            disabled=signup_action.pending()
                            class=crate::cls!("w-full px-4 py-2 rounded-md bg-primary text-primary-foreground font-medium hover:bg-primary/90 disabled:opacity-50")
                        >
                            {move || {
                                if signup_action.pending()() {
                                    "Creating..."
                                } else {
                                    "Sign up"
                                }
                            }}
                        </button>
                    </form>

                    <div class=crate::cls!("text-center text-sm")>
                        <a href="/login" class=crate::cls!("text-primary hover:underline")>"Already have account?"</a>
                    </div>
                </div>
            </div>
        </AuthLayout>
    }
}
