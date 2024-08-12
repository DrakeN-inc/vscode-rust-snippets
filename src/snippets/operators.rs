use vscode_generator::{ Snippets, Snippet };

// Returns the operators snippets list
pub(crate) fn operators_snippets() -> Snippets {
    Snippets::new(
        "rust",
        "Operators",
        "The code operators snippets",
        vec![
            Snippet::text("operator-use-prelude", "use prelude::", "use crate::prelude::${1:*};").set_descr("use crate::prelude::*;"),
            Snippet::text("operator-use-crate", "use crate::", "use crate::$1;").set_descr("use crate::...;"),
            Snippet::text("operator-use-super", "use super::", "use super::$1;").set_descr("use super::...;"),

            Snippet::operator("operator-mod", "mod", Some("")),
            Snippet::operator("operator-use", "use", Some(""))
                .set_descr("use ...::...;")
                .set_body(vec!["use $1::$2;"]),
            Snippet::operator("operator-mod-use", "mod use", Some(""))
                .set_descr("mod ...;  pub use ...::...;")
                .set_body(vec!["mod $1;  pub use $1::$2;"]),

            Snippet::operator("operator-let", "let", Some(""))
                .set_descr("let ... = ...;")
                .set_body(vec!["let $1 = $2;"]),
            Snippet::operator("operator-static", "static", Some(""))
                .set_descr("static ...:... = ...;")
                .set_body(vec!["static $1: $2 = $3;"]),
            Snippet::operator("operator-const", "const", Some(""))
                .set_descr("const ...:... = ...;")
                .set_body(vec!["const $1: $2 = $3;"]),

            Snippet::operator("operator-return", "return", Some("")),
            Snippet::operator("operator-break-return", "break", Some("")),
            Snippet::operator("operator-break", "break", None),
            Snippet::operator("operator-continue", "continue", None),
        ]
    )
}
