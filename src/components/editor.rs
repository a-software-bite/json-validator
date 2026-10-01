use crate::validation::json_validator::{validate_json, ParserKind, ValidationResult};
use leptos::prelude::*;

#[component]
pub fn JsonEditor() -> impl IntoView {
    let (json_input, set_json_input) = signal(String::new());
    let (parser_kind, set_parser_kind) = signal(ParserKind::Fast);
    let (result, set_result) = signal(Option::<ValidationResult>::None);

    let on_validate = move |_| {
        let input = json_input.get();
        let validation = validate_json(&input, parser_kind.get());
        set_result.set(Some(validation));
    };

    view! {
            <div class="min-h-screen bg-background flex flex-col items-center p-8">
                <div class="w-full max-w-4xl">
                    <h1 class="text-foreground text-5xl font-bold tracking-tight mb-12">"JSON Validator"</h1>

                    <div class="rounded-2xl border border-border bg-card p-8 shadow-md">
                        <div class="flex items-center justify-between mb-5">
    <label class="text-xl font-semibold text-foreground">"JSON Data"</label>
                            <select
                                class="bg-background text-foreground border border-input
                                   rounded-lg py-2.5 px-4 text-sm cursor-pointer
                                   focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                                on:change=move |ev| {
                                    let value = event_target::<web_sys::HtmlSelectElement>(&ev).value();
                                    set_parser_kind.set(match value.as_str() {
                                        "slow" => ParserKind::Slow,
                                        _ => ParserKind::Fast,
                                    });
                                }
                                prop:value=move || match parser_kind.get() {
                                    ParserKind::Slow => "slow",
                                    ParserKind::Fast => "fast",
                                }
                            >
                                <option value="fast">"Fast (single pass)"</option>
                                <option value="slow">"Slow (tokenizer)"</option>
                            </select>
    </div>
                        <textarea
                            class="w-full h-96 p-5 rounded-xl border border-input bg-background
                               text-foreground placeholder:text-muted-foreground
                               focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2
                               resize-y font-mono text-base"
                            placeholder="Paste your JSON here..."
                            on:input=move |ev| set_json_input.set(event_target::<web_sys::HtmlTextAreaElement>(&ev).value())
                            prop:value=move || json_input.get()
                        />

                        <div class="mt-8 flex justify-center">

                            <button
                                class="w-full max-w-xs bg-primary text-primary-foreground
                                   text-xl font-semibold py-4 px-8 rounded-lg transition-colors
                                   hover:bg-primary/90 focus:outline-none focus:ring-2
                                   focus:ring-ring focus:ring-offset-2 cursor-pointer"
                                on:click=on_validate
                            >
                                "Validate"
                            </button>
                        </div>

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

                    <p class="mt-12 text-center text-xl text-muted-foreground">
                        "Paste JSON to validate its syntax"
                    </p>
                </div>
            </div>
        }
}
