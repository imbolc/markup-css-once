//! # markup-css-once
//!
//! Render embedded styles only once per template with [Markup][] Rust template engine
//!
//! ## Usage
//!
//! Let's say we have a template we'd like to use on the page multiple times. We also have styles
//! related to this template inside an embedded `<style>` tag.
//!
//! ```rust
//! use markup_css_once::{CssOnce, css_once};
//!
//! markup::define! {
//!     Hello<'a>(
//!         css: &'a CssOnce,
//!         name: &'a str,
//!     ) {
//!         @css_once!(css,
//!             "p { background: blue }"
//!             "b { color: yellow }"
//!         )
//!         p { "Hello, " b { @name } }
//!     }
//! }
//!
//! // We need an tracker for components with already rendered css
//! let css = CssOnce::new();
//!
//! // The first time the template is rendered with styles
//! assert_eq!(
//!     Hello { css: &css, name: "World" }.to_string(),
//!     "<style>p { background: blue }b { color: yellow }</style>\n<p>Hello, <b>World</b></p>"
//! );
//!
//! // But all subsequent calls will render only it's HTML
//! assert_eq!(
//!     Hello { css: &css, name: "World" }.to_string(),
//!     "<p>Hello, <b>World</b></p>"
//! );
//! ```
//!
//! [Markup]: https://github.com/utkarshkukreti/markup.rs

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

use std::any::type_name;
use std::cell::Cell;
use std::collections::HashSet;

/// A tracker for rendered css
#[derive(Default)]
pub struct CssOnce(Cell<HashSet<&'static str>>);

impl CssOnce {
    /// Creates a new instance of the tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if styles for template `T` is already rendered
    pub fn is_rendered<T>(&self) -> bool {
        let mut inner = self.0.take();
        let inserted = inner.insert(type_name::<T>());
        self.0.set(inner);
        !inserted
    }
}

/// A macro for the Markup templates to ensure the css is rendered only once
#[macro_export]
macro_rules! css_once {
    ($css:ident, $($str:tt)+) => {
        if CssOnce::is_rendered::<Self>($css) {
            markup::raw("")
        } else {
            markup::raw(concat!("<style>", $($str),+, "</style>\n"))
        }
    };
}
