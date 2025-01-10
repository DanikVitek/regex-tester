use core::fmt;
use reactive_stores::Store;
use regex_syntax::hir::{Capture, Class, Hir, HirKind, Literal, Look, Properties, Repetition};

#[derive(Debug, Store, PartialEq, Eq)]
pub struct Settings {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    swap_greed: bool,
    unicode: bool,
    crlf: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            unicode: true,
            crlf: false,
        }
    }
}

pub struct HirDebug<'a>(pub &'a Hir);

impl fmt::Debug for HirDebug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
