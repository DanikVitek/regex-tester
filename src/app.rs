use core::fmt;

use leptos::*;
use regex::Regex;
use regex_syntax::hir::{Capture, Class, Hir, HirKind, Literal, Look, Properties, Repetition};
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
                <pre>{format!("{:#?}", HirDebug(hir))}</pre>
            </div>
        }.into_any(),
        Err(err) => view! {
            <pre class=tw_merge!("text-error", class)>{err.to_string()}</pre>
        }.into_any(),
    })}}
}

struct HirDebug<'a>(&'a Hir);

impl fmt::Debug for HirDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let box_dbg_kind: Box<dyn fmt::Debug> = match self.0.kind() {
            HirKind::Empty => {
                struct EmptyDebug;
                impl fmt::Debug for EmptyDebug {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.debug_tuple("Empty").finish()
                    }
                }
                Box::new(EmptyDebug)
            }
            HirKind::Literal(lit) => {
                struct LiteralDebug<'a>(&'a Literal);
                impl fmt::Debug for LiteralDebug<'_> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.debug_tuple("Literal").field(&self.0).finish()
                    }
                }

                Box::new(LiteralDebug(lit))
            }
            HirKind::Class(class) => {
                struct ClassDebug<'a>(&'a Class);
                impl fmt::Debug for ClassDebug<'_> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.debug_tuple("Class").field(self.0).finish()
                    }
                }

                Box::new(ClassDebug(class))
            }
            HirKind::Look(look) => {
                struct LookDebug<'a>(&'a Look);
                impl fmt::Debug for LookDebug<'_> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.debug_tuple("Look").field(self.0).finish()
                    }
                }

                Box::new(LookDebug(look))
            }
            HirKind::Repetition(rep) => {
                struct RepetitionDebug<'a>(&'a Repetition);
                impl fmt::Debug for RepetitionDebug<'_> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.debug_struct("Repetition")
                            .field("min", &self.0.min)
                            .field("max", &self.0.max)
                            .field("greedy", &self.0.greedy)
                            .field("sub", &HirDebug(&self.0.sub))
                            .finish()
                    }
                }

                Box::new(RepetitionDebug(rep))
            }
            HirKind::Capture(cap) => {
                struct CaptureDebug<'a>(&'a Capture);
                impl fmt::Debug for CaptureDebug<'_> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.debug_struct("Capture")
                            .field("index", &self.0.index)
                            .field("name", &self.0.name)
                            .field("sub", &HirDebug(&self.0.sub))
                            .finish()
                    }
                }

                Box::new(CaptureDebug(cap))
            }
            HirKind::Concat(subs) => {
                struct ConcatDebug<'a>(&'a [Hir]);
                impl fmt::Debug for ConcatDebug<'_> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.debug_tuple("Concat")
                            .field(&self.0.iter().map(HirDebug).collect::<Vec<_>>())
                            .finish()
                    }
                }

                Box::new(ConcatDebug(subs))
            }
            HirKind::Alternation(subs) => {
                struct AlternationDebug<'a>(&'a [Hir]);
                impl fmt::Debug for AlternationDebug<'_> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.debug_tuple("Alternation")
                            .field(&self.0.iter().map(HirDebug).collect::<Vec<_>>())
                            .finish()
                    }
                }

                Box::new(AlternationDebug(subs))
            }
        };

        struct HirPropertiesDebug<'a>(&'a Properties);
        impl fmt::Debug for HirPropertiesDebug<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Properties")
                    .field("minimum_len", &self.0.minimum_len())
                    .field("maximum_len", &self.0.maximum_len())
                    .field("look_set", &self.0.look_set())
                    .field("look_set_prefix", &self.0.look_set_prefix())
                    .field("look_set_suffix", &self.0.look_set_suffix())
                    .field("look_set_prefix_any", &self.0.look_set_prefix_any())
                    .field("look_set_suffix_any", &self.0.look_set_suffix_any())
                    .field("utf8", &self.0.is_utf8())
                    .field("explicit_captures_len", &self.0.explicit_captures_len())
                    .field(
                        "static_explicit_captures_len",
                        &self.0.static_explicit_captures_len(),
                    )
                    .field("literal", &self.0.is_literal())
                    .field("alternation_literal", &self.0.is_alternation_literal())
                    .finish()
            }
        }

        f.debug_struct("Hir")
            .field("kind", &*box_dbg_kind)
            .field("properties", &HirPropertiesDebug(self.0.properties()))
            .finish()
    }
}
