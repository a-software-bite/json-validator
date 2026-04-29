use crate::components::editor::JsonEditor;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <JsonEditor />
        </main>
    }
}
