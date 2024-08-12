use vscode_generator::{ Snippets, Snippet };

// Returns the blocks snippets list
pub(crate) fn blocks_snippets() -> Snippets {
    Snippets::new(
        "rust",
        "Blocks",
        "The blocks snippets",
        vec![
            Snippet::simple_block("block-mod", "mod"),

            Snippet::block("block-struct", "struct"),
            Snippet::block("block-enum", "enum"),
            Snippet::double_block("block-struct-impl", "struct", "impl"),
            Snippet::double_block("block-enum-impl", "enum", "impl"),

            Snippet::function_block("block-fn", "fn"),
            Snippet::function_block("fn-main", "fn main")
                .set_descr("fn main() { ... }")
                .set_body(vec![
                    "fn main() {",
                    "    $1",
                    "}",
                ]),
            Snippet::function_block("fn-async-main", "async fn main")
                .set_descr("async fn main() { ... }")
                .set_body(vec![
                    "#[${1|tokio,async_std,actix_web,axum|}::main]",
                    "async fn main() {",
                    "    $2",
                    "}",
                ]),

            Snippet::block("block-macro_rules", "macro_rules!")
                .set_body(vec![
                    "macro_rules! $1 {",
                    "    ($2) => {",
                    "        $3",
                    "    };",
                    "}",
                ]),
            
            Snippet::block("block-if", "if"),
            Snippet::double_block("block-if-else", "if", "else")
                .set_descr("if ... { ... } else { ... }")
                .set_body(vec![
                    "if $1 {",
                    "    $2",
                    "} else {",
                    "    $3",
                    "}"
                ]),
            Snippet::block("block-short-if-else", "if?")
                .set_descr("if ... { ... }else{ ... }")
                .set_body(vec!["if $1 { $2 }else{ $3 }"]),

            Snippet::block("block-match", "match"),
            Snippet::block("block-match-result", "match Result<T, E>")
                .set_descr("match Result<T, E> { ... }")
                .set_body(vec![
                    "match $1 {",
                    "    Ok(v) => $2,",
                    "    Err(e) => $3",
                    "}"
                ]),
            Snippet::block("block-match-option", "match Option<T>")
                .set_descr("match Option<T> { ... }")
                .set_body(vec![
                    "match $1 {",
                    "    Some(v) => $2,",
                    "    None => $3",
                    "}"
                ]),

            Snippet::block("block-for", "for")
                .set_descr("for v in ... { ... }")
                .set_body(vec![
                    "for ${1:v} in $2 {",
                    "    $3",
                    "}"
                ]),
            Snippet::block("block-for-key-value", "for (k, v)")
                .set_descr("for (k, v) in ... { ... }")
                .set_body(vec![
                    "for ${1:(k, v)} in ${2:.enumerate()} {",
                    "    $3",
                    "}"
                ]),
            Snippet::block("block-while", "while"),
            Snippet::simple_block("block-loop", "loop"),
        ]
    )
}
