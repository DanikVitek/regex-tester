use leptos::{either::either, prelude::*};
use reactive_stores::{Field, Store};
use regex::RegexBuilder;
use regex_syntax::hir::Hir;
use tailwind_fuse::*;

use crate::model::{HirDebug, Settings, SettingsStoreFields};

#[component]
pub fn App() -> impl IntoView {
    let (regex_input, set_regex_input) = signal(String::new());
    let settings = Store::new(Settings::default());
    view! {
        <main class="px-4 pt-4">
            <div class="form-control w-full mb-4">
                <label class="label" for="regex">
                    <span class="label-text">{"Regex:"}</span>
                </label>
                <div class="flex flex-row gap-4 w-full items-center">
                    <textarea
                        id="regex"
                        class="textarea textarea-bordered w-full"
                        style:field-sizing="content"
                        bind:value=(regex_input, set_regex_input)
                    />
                    <Settings settings />
                </div>
            </div>
            <div class="flex flex-col md:flex-row justify-stretch">
                <RegexTest class="w-full" regex_input settings />
                <div class="divider max-h-min md:max-h-none md:divider-horizontal md:max-w-min" />
                <HirView class="w-full" regex_input settings />
            </div>
        </main>
    }
}

#[component]
fn Settings(settings: Store<Settings>) -> impl IntoView {
    view! {
        <details class="dropdown dropdown-end">
            <summary class="btn btn-square btn-ghost">
                <SettingsIcon aria_label="Settings" />
            </summary>
            <div class="form-control menu dropdown-content bg-base-300 rounded-box z-[1] mt-4 p-2 w-56 shadow">
                <SettingsFlag label="Case-insensitive" value=settings.case_insensitive() />
                <SettingsFlag label="Multi-line" value=settings.multi_line() />
                <SettingsFlag label="Dot matches new line" value=settings.dot_matches_new_line() />
                <SettingsFlag label="Swap greed" value=settings.swap_greed() />
                <SettingsFlag label="Unicode" value=settings.unicode() />
                <SettingsFlag label="CRLF" value=settings.crlf() />
            </div>
        </details>
    }
}

#[component]
fn SettingsFlag(label: &'static str, #[prop(into)] value: Field<bool>) -> impl IntoView {
    view! {
        <label class="label cursor-pointer w-full">
            <span class="label-text">{label}</span>
            <input type="checkbox" bind:checked=value class="checkbox" />
        </label>
    }
}

#[component]
fn SettingsIcon(
    #[prop(optional)] class: Option<&'static str>,
    aria_label: &'static str,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            class=class.unwrap_or_default()
            aria-label=aria_label
        >
            <path
                fill="currentColor"
                d="M10.825 22q-.675 0-1.162-.45t-.588-1.1L8.85 18.8q-.325-.125-.612-.3t-.563-.375l-1.55.65q-.625.275-1.25.05t-.975-.8l-1.175-2.05q-.35-.575-.2-1.225t.675-1.075l1.325-1Q4.5 12.5 4.5 12.337v-.675q0-.162.025-.337l-1.325-1Q2.675 9.9 2.525 9.25t.2-1.225L3.9 5.975q.35-.575.975-.8t1.25.05l1.55.65q.275-.2.575-.375t.6-.3l.225-1.65q.1-.65.588-1.1T10.825 2h2.35q.675 0 1.163.45t.587 1.1l.225 1.65q.325.125.613.3t.562.375l1.55-.65q.625-.275 1.25-.05t.975.8l1.175 2.05q.35.575.2 1.225t-.675 1.075l-1.325 1q.025.175.025.338v.674q0 .163-.05.338l1.325 1q.525.425.675 1.075t-.2 1.225l-1.2 2.05q-.35.575-.975.8t-1.25-.05l-1.5-.65q-.275.2-.575.375t-.6.3l-.225 1.65q-.1.65-.587 1.1t-1.163.45zM11 20h1.975l.35-2.65q.775-.2 1.438-.587t1.212-.938l2.475 1.025l.975-1.7l-2.15-1.625q.125-.35.175-.737T17.5 12t-.05-.787t-.175-.738l2.15-1.625l-.975-1.7l-2.475 1.05q-.55-.575-1.212-.962t-1.438-.588L13 4h-1.975l-.35 2.65q-.775.2-1.437.588t-1.213.937L5.55 7.15l-.975 1.7l2.15 1.6q-.125.375-.175.75t-.05.8q0 .4.05.775t.175.75l-2.15 1.625l.975 1.7l2.475-1.05q.55.575 1.213.963t1.437.587zm1.05-4.5q1.45 0 2.475-1.025T15.55 12t-1.025-2.475T12.05 8.5q-1.475 0-2.487 1.025T8.55 12t1.013 2.475T12.05 15.5M12 12"
            />
        </svg>
    }
}

#[component]
fn RegexTest(
    #[prop(optional)] class: Option<&'static str>,
    regex_input: ReadSignal<String>,
    settings: Store<Settings>,
) -> impl IntoView {
    let regex = Memo::new_with_compare(
        move |_| {
            RegexBuilder::new(&regex_input.read())
                .case_insensitive(settings.case_insensitive().get())
                .multi_line(settings.multi_line().get())
                .dot_matches_new_line(settings.dot_matches_new_line().get())
                .swap_greed(settings.swap_greed().get())
                .unicode(settings.unicode().get())
                .crlf(settings.crlf().get())
                .build()
        },
        |old, new| match (old, new) {
            (Some(old), Some(new)) => match (old, new) {
                (Err(old), Err(new)) => old != new,
                _ => true,
            },
            (Some(_), None) | (None, Some(_)) => true,
            (None, None) => false,
        },
    );

    let (test_input, set_test_input) = signal_local(String::new());
    let is_match = move || {
        regex
            .read()
            .as_ref()
            .is_ok_and(|r| r.is_match(&test_input.read()))
    };

    view! {{move || either!(&*regex.read(),
        Ok(_) => view! {
            <div id="test" class=tw_join!("flex flex-col gap-2", class)>
                <div class="form-control">
                    <label class="label" for="test-input">
                        <span class="label-text">{"Test input:"}</span>
                    </label>
                    <textarea
                        id="test-input"
                        class="textarea textarea-bordered"
                        style:field-sizing="content"
                        bind:value=(test_input, set_test_input)
                    />
                </div>
                <div>
                    <span class="font-bold">{"Is match: "}</span>
                    {is_match}
                </div>
            </div>
        },
        Err(err) => view! {
            <pre class=tw_join!("text-error", class)>{err.to_string()}</pre>
        },
    )}}
}

#[component]
fn HirView(
    #[prop(optional)] class: Option<&'static str>,
    regex_input: ReadSignal<String>,
    settings: Store<Settings>,
) -> impl IntoView {
    let hir = Memo::<Result<Hir, regex_syntax::Error>>::new(move |_| {
        regex_syntax::ParserBuilder::new()
            .case_insensitive(settings.case_insensitive().get())
            .multi_line(settings.multi_line().get())
            .dot_matches_new_line(settings.dot_matches_new_line().get())
            .swap_greed(settings.swap_greed().get())
            .unicode(settings.unicode().get())
            .crlf(settings.crlf().get())
            .build()
            .parse(&regex_input.read())
    });

    view! {{move || either!(&*hir.read(),
        Ok(hir) => view! {
            <div id="hir" class=tw_join!("flex flex-col gap-2", class)>
                <pre>{hir.to_string()}</pre>
                <pre>{format!("{:#?}", HirDebug(hir))}</pre>
            </div>
        },
        Err(err) => view! {
            <pre class=tw_join!("text-error", class)>{err.to_string()}</pre>
        },
    )}}
}
