use leptos::*;
use regex::Regex;
use regex_syntax::hir::Hir;
use tailwind_fuse::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="px-4 pt-4">
            <Home />
        </main>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (regex_input, set_regex_input) = create_signal(String::new());

    view! {
        <div class="form-control mb-4">
            <label class="label">
                <span class="label-text">{"Regex: "}</span>
            </label>
            <input
                class="input input-bordered"
                type="text"
                prop:value=move || regex_input.get()
                on:input=move |e| {
                    set_regex_input.set(event_target_value(&e));
                }
            />
        </div>
        <div class="flex flex-col md:flex-row justify-stretch">
            <RegexTest class="w-full" regex_input/>
            <div class="divider max-h-min md:max-h-none md:divider-horizontal md:max-w-min" />
            <HirView class="w-full" regex_input />
        </div>
    }
}

#[component]
fn RegexTest(
    #[prop(optional)] class: Option<&'static str>,
    regex_input: ReadSignal<String>,
) -> impl IntoView {
    let regex = create_owning_memo::<Result<Regex, regex::Error>>(move |old_regex| {
        regex_input.with(|input| {
            old_regex
                .filter(|old_result| match old_result {
                    Ok(old_regex) => old_regex.as_str() == input,
                    Err(regex::Error::Syntax(old_input)) => old_input == input,
                    Err(regex::Error::CompiledTooBig(_)) => false,
                    Err(err) => unimplemented!("unsupported error variant: {err:?}"),
                })
                .map(|old_regex| (old_regex, false))
                .unwrap_or_else(|| (Regex::new(input), true))
        })
    });

    let (test_input, set_test_input) = create_signal(String::new());
    let is_match =
        move || with!(|regex, test_input| regex.as_ref().is_ok_and(|r| r.is_match(test_input)));

    view! {{move || regex.with(|regex| {
        match regex {
            Ok(_) => view! {
                <div id="test" class=tw_merge!("flex flex-col gap-2", class)>
                    <textarea
                        class="textarea textarea-bordered"
                        prop:value=move || test_input.get()
                        on:input=move |e| {
                            set_test_input.set(event_target_value(&e));
                        }
                    />
                    <div>
                        <span class="font-bold">{"Is match: "}</span>
                        {is_match}
                    </div>
                </div>
            }.into_any(),
            Err(err) => view! {
                <pre class=tw_merge!("text-error", class)>{err.to_string()}</pre>
            }.into_any(),
        }
    })}}
}

#[component]
fn HirView(
    #[prop(optional)] class: Option<&'static str>,
    regex_input: ReadSignal<String>,
) -> impl IntoView {
    let hir = create_memo::<Result<Hir, regex_syntax::Error>>(move |_| {
        regex_input.with(|input| regex_syntax::parse(input))
    });

    view! {{move || hir.with(|hir| match hir {
        Ok(hir) => view! {
            <div id="hir" class=tw_merge!("flex flex-col gap-2", class)>
                <pre>{hir.to_string()}</pre>
                <pre>{format!("{:#?}", hir.kind())}</pre>
                <pre>{format!("{:#?}", hir.properties())}</pre>
            </div>
        }.into_any(),
        Err(err) => view! {
            <pre class=tw_merge!("text-error", class)>{err.to_string()}</pre>
        }.into_any(),
    })}}
}
