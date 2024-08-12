use vscode_generator::{ Snippets, Snippet };

// Returns the functions snippets list
pub(crate) fn functions_snippets() -> Snippets {
    Snippets::new(
        "rust",
        "Functions",
        "The functions, methods && macros snippets",
        vec![
            Snippet::function("method-unwrap", ".unwrap", None, None),
            Snippet::function("method-unwrap_or", ".unwrap_or", None, Some("")),
            Snippet::function("method-unwrap_or_else", ".unwrap_or_else", None, Some("")),
            Snippet::function("method-expect", ".expect", None, Some("")),
            Snippet::function("method-map", ".map", None, Some("")),
            Snippet::function("method-map_err", ".map_err", None, Some("Error::from")),

            Snippet::function("method-to_lowercase", ".to_lowercase", None, None),
            Snippet::function("method-to_uppercase", ".to_uppercase", None, None),
            Snippet::function("method-split", ".split", None, Some("")),
            Snippet::function("method-rsplit", ".rsplit", None, Some("")),
            Snippet::function("method-splitn", ".splitn", None, Some("")),
            Snippet::function("method-rsplitn", ".rsplitn", None, Some("")),
            Snippet::function("method-split_once", ".split_once", None, Some("")),
            Snippet::function("method-rsplit_once", ".rsplit_once", None, Some("")),
            Snippet::function("method-split_whitespace", ".split_whitespace", None, Some("")),
            Snippet::function("method-split_terminator", ".split_terminator", None, Some("")),
            Snippet::function("method-rsplit_terminator", ".rsplit_terminator", None, Some("")),
            Snippet::function("method-replace", ".replace", None, Some("")),
            Snippet::function("method-replacen", ".replacen", None, Some("")),
            Snippet::function("method-repeat", ".repeat", None, Some("")),

            Snippet::function("static-method-new", "::new", None, Some("")),
            Snippet::function("static-method-with_capacity", "::with_capacity", None, Some("")),
            Snippet::function("static-method-from", "::from", None, Some("")),
            Snippet::function("static-method-from_utf8", "::from_utf8", None, Some("")),
            Snippet::function("static-method-from_utf8_lossy", "::from_utf8_lossy", None, Some("")),
            Snippet::function("static-method-from_utf16", "::from_utf16", None, Some("")),
            Snippet::function("static-method-from_utf16_lossy", "::from_utf16_lossy", None, Some("")),
            Snippet::function("static-method-default", "::default", None, None),

            Snippet::function("macro-str", "str!", None, Some("")),
            Snippet::function("macro-vec", "vec!", Some(("[", "]")), Some("")),
            Snippet::function("macro-deq", "deq!", Some(("[", "]")), Some("")),
            Snippet::function("macro-map", "map!", Some(("{", "}")), Some("")),
            Snippet::function("macro-set", "set!", Some(("[", "]")), Some("")),
            Snippet::function("macro-heap", "heap!", Some(("[", "]")), Some("")),
            Snippet::function("macro-list", "list!", Some(("[", "]")), Some("")),
            Snippet::function("macro-btree_map", "btree_map!", Some(("{", "}")), Some("")),
            Snippet::function("macro-btree_set", "btree_set!", Some(("[", "]")), Some("")),
            Snippet::function("macro-bson", "bson!", Some(("{", "}")), Some("")),
            Snippet::function("macro-rawbson", "rawbson!", Some(("{", "}")), Some("")),
            Snippet::function("macro-doc", "doc!", Some(("{", "}")), Some("")),
            Snippet::function("macro-rawdoc", "rawdoc!", Some(("{", "}")), Some("")),
            Snippet::function("macro-html", "html!", Some(("{", "}")), Some("")),
            Snippet::function("macro-json", "json!", Some(("{", "}")), Some("")),
            
            Snippet::function("macro-format", "format!", None, Some("")),
            Snippet::function("macro-print", "print!", None, Some("")),
            Snippet::function("macro-println", "println!", None, Some("")),
            Snippet::function("macro-eprint", "eprint!", None, Some("")),
            Snippet::function("macro-eprintln", "eprintln!", None, Some("")),
            Snippet::function("macro-input", "input!", None, Some("")),
            Snippet::function("macro-panic", "panic!", None, Some("")),
            Snippet::function("macro-todo", "todo!", None, Some("")),
            Snippet::function("macro-unimplemented", "unimplemented!", None, Some("")),
            Snippet::function("macro-unreachable", "unreachable!", None, Some("")),
        ]
    )
}
