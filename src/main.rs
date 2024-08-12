use vscode_generator::{ prelude::*, Package, License };

mod snippets;

fn main() -> Result<()> {
    // generate package:
    let pkg = Package::snippets(
        "vscode-rust-snippets",
        "Rust snippets",
        "The Rust programming language snippets for Visual Studio Code",
        "0.1.0".parse()?,
        "images/icon.png",
        Some("https://github.com/DrakeN-inc/vscode-rust-snippets"),
        vec![
            snippets::text_snippets(),
            snippets::attributes_snippets(),
            snippets::blocks_snippets(),
            snippets::operators_snippets(),
            snippets::functions_snippets(),
        ],
        License::mit("DrakeN-inc")
    );

    // write package to dir:
    pkg.write_to("./package")?;

    Ok(())
}
