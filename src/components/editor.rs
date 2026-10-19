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
                    <h1 class="text-foreground text-5xl font-bold tracking-tight mb-6">"JSON Validator"</h1>

                    <p class="text-xl text-muted-foreground mb-8">
                        "Paste JSON to validate its syntax"
                    </p>

                    <div class="rounded-2xl border border-border bg-card p-8 shadow-md">
                        <div class="flex items-center justify-between mb-5">
    <label class="text-xl font-semibold text-foreground">"JSON Data"</label>
                            <select
                                class="custom-select bg-background text-foreground border border-input
                                   rounded-lg py-2.5 pl-4 pr-12 text-sm cursor-pointer
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

                    <div class="mt-12 flex justify-center">
                        <a
                            href="https://github.com/daconjurer"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-foreground hover:text-foreground/80 transition-colors"
                            aria-label="GitHub"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-10 h-10" fill="currentColor">
                                <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577
                                    0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729
                                    1.205.084 1.84 1.236 1.84 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.419-1.305.762-1.605-2.665-.305-5.467-1.334-5.467-5.931
                                    0-1.31.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404
                                    1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221
                                    0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222 0 1.606-.015 2.898-.015 3.293 0 .319.218.694.825.577
                                    4.765-1.585 8.199-6.082 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                        </a>
                    </div>
                </div>
            </div>
        }
}
