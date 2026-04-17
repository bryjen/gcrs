use leptos::prelude::*;

#[island]
pub fn LinkPanel() -> impl IntoView {
    view! {
        <h3 class="text-base font-semibold text-foreground mb-4">
            "Repositories"
        </h3>
        <div class="space-y-2">
            <a href="/repo" class="block text-primary hover:underline transition-colors">
                "noctua"
            </a>
            <a href="/repo" class="block text-primary hover:underline transition-colors">
                "ShadcnBlazor"
            </a>
            <a href="/repo" class="block text-primary hover:underline transition-colors">
                "bryjen.github.io"
            </a>
            <a href="/repo" class="block text-primary hover:underline transition-colors">
                "remote-infra"
            </a>
            <a href="/repo" class="block text-primary hover:underline transition-colors">
                "nixos-dotfiles"
            </a>
        </div>
    }
}
