use leptos::prelude::*;
use leptos::ev::Event;
use leptos::wasm_bindgen::JsCast;
use crate::validation::json_validator::{validate_json, ValidationResult};

fn event_target_value(ev: &Event) -> String {
    let target = ev.target().unwrap();
    let input: web_sys::HtmlTextAreaElement = target.unchecked_into();
    input.value()
}

#[component]
pub fn JsonEditor() -> impl IntoView {
    let (json_input, set_json_input) = signal(String::new());
    let (result, set_result) = signal(Option::<ValidationResult>::None);

    let on_validate = move |_| {
        let input = json_input.get();
        let validation = validate_json(&input);
        set_result.set(Some(validation));
    };

    view! {
        <div class="min-h-screen bg-background flex flex-col items-center p-8">
            <div class="w-full max-w-2xl">
                <h1 class="text-foreground text-3xl font-semibold tracking-tight mb-8">"JSON Validator"</h1>

                <div class="rounded-xl border border-border bg-card p-6 shadow-sm">
                    <label class="text-sm font-medium text-foreground mb-2 block">"JSON Data"</label>
                    <textarea
                        class="w-full h-64 p-3 rounded-lg border border-input bg-background
                               text-foreground placeholder:text-muted-foreground
                               focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2
                               resize-none font-mono text-sm"
                        placeholder="Paste your JSON here..."
                        on:input=move |ev| set_json_input.set(event_target_value(&ev))
                        prop:value=move || json_input.get()
                    />

                    <button
                        class="mt-4 w-full bg-primary text-primary-foreground
                               font-medium py-2.5 px-4 rounded-lg transition-colors
                               hover:bg-primary/90 focus:outline-none focus:ring-2
                               focus:ring-ring focus:ring-offset-2 cursor-pointer"
                        on:click=on_validate
                    >
                        "Validate"
                    </button>

                    {move || {
                        result.get().map(|r| {
                            if r.is_valid {
                                view! {
                                    <div class="mt-4 p-4 rounded-lg border border-success/30 bg-success/10 text-success">
                                        <p class="font-medium">"✓ Valid JSON"</p>
                                    </div>
                                }.into_any()
                            } else {
                                let error_msg = r.error_message.clone().unwrap_or_default();
                                let location = match (r.error_line, r.error_column) {
                                    (Some(line), Some(col)) => format!(" (line {}, column {})", line, col),
                                    _ => String::new(),
                                };
                                view! {
                                    <div class="mt-4 p-4 rounded-lg border border-destructive/30 bg-destructive/10 text-destructive">
                                        <p class="font-medium">"✗ Invalid JSON"{location}</p>
                                        <p class="mt-2 font-mono text-sm opacity-90">{error_msg}</p>
                                    </div>
                                }.into_any()
                            }
                        })
                    }}
                </div>

                <p class="mt-6 text-center text-sm text-muted-foreground">
                    "Paste JSON to validate its syntax"
                </p>
            </div>
        </div>
    }
}
