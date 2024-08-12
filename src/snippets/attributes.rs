use vscode_generator::{ Snippets, Snippet };

// Returns the attributes snippets list
pub(crate) fn attributes_snippets() -> Snippets {
    Snippets::new(
        "rust",
        "Attributes",
        "The attributes snippets",
        vec![
            Snippet::attribute("attr-derive", "derive", Some(vec![
                "Debug\\, Clone",
                "Eq\\, PartialEq",
                "Ord\\, PartialOrd",
                "Serialize\\, Deserialize",
                "Debug",
                "Display",
                "Clone",
                "Copy",
                "Default",
                "Eq",
                "PartialEq",
                "Ord",
                "PartialOrd",
                "Serialize",
                "Deserialize",
            ])),
            
            Snippet::attribute("attr-allow", "allow", Some(vec![
                "dead_code",
                "unused_variables",
                "non_snake_case",
            ])),

            Snippet::attribute("attr-test", "test", None),
            Snippet::attribute("attr-cfg", "cfg", Some(vec![])),
            Snippet::attribute("attr-macro_use", "macro_use", None),

            Snippet::attribute("attr-serde", "serde", Some(vec![])),

            Snippet::attribute("attr-proc_macro", "proc_macro", None),
            Snippet::attribute("attr-proc_macro_attribute", "proc_macro_attribute", None),
            Snippet::attribute("attr-proc_macro_derive", "proc_macro_derive", Some(vec![]))
                .set_descr("#[proc_macro_derive(..., attributes(...))]")
                .set_body(vec!["#[proc_macro_derive($1, attributes($2))]"]),
        ]
    )
}
