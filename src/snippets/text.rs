use vscode_generator::{ Snippets, Snippet };

// Returns the text snippets list
pub(crate) fn text_snippets() -> Snippets {
    Snippets::new(
        "rust",
        "Text",
        "The simple text snippets",
        vec![
            Snippet::text("print-hello", "hello", r#"println!("Hello, world!");  // DEBUG: "Hello, world!" message"#),
            Snippet::comment("comment-todo", "TODO"),
            Snippet::comment("comment-note", "NOTE"),
            Snippet::comment("comment-debug", "DEBUG"),
            Snippet::comment("comment-fixme", "FIXME"),
        ]
    )
}
