use leptos::prelude::*;
use crate::components::editor::JsonEditor;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <JsonEditor />
        </main>
    }
}
