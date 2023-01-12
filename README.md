[![License](https://img.shields.io/crates/l/markup-css-once.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/markup-css-once.svg)](https://crates.io/crates/markup-css-once)
[![Docs.rs](https://docs.rs/markup-css-once/badge.svg)](https://docs.rs/markup-css-once)

<!-- cargo-sync-readme start -->

# markup-css-once

Render embedded styles only once per template with [Markup][] Rust template engine

## Usage

Let's say we have a template we'd like to use on the page multiple times. We also have styles
related to this template inside an embedded `<style>` tag.

```rust
use markup_css_once::{CssOnce, css_once};

markup::define! {
    Hello<'a>(
        css: &'a CssOnce,
        name: &'a str,
    ) {
        @css_once!(css,
            "p { background: blue }"
            "b { color: yellow }"
        )
        p { "Hello, " b { @name } }
    }
}

// We need an tracker for components with already rendered css
let css = CssOnce::new();

// The first time the template is rendered with styles
assert_eq!(
    Hello { css: &css, name: "World" }.to_string(),
    "<style>p { background: blue }b { color: yellow }</style>\n<p>Hello, <b>World</b></p>"
);

// But all subsequent calls will render only it's HTML
assert_eq!(
    Hello { css: &css, name: "World" }.to_string(),
    "<p>Hello, <b>World</b></p>"
);
```

[Markup]: https://github.com/utkarshkukreti/markup.rs

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
