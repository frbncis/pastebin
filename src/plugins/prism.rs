use std::collections::HashMap;

use crate::plugins::CssImport;
use crate::plugins::plugin::PastebinPlugin;

pub fn new<'r>() -> PastebinPlugin<'r> {
    PastebinPlugin {
        css_imports: vec![
            CssImport {
                url: "/static/prism.css",
                media_query: "(prefers-color-scheme: light)",
            },
            CssImport {
                url: "/static/prism-dark.css",
                media_query: "(prefers-color-scheme: dark)",
            },
        ],
        js_imports: vec!["/static/prism.js"],
        js_init: Some(
            "var holder = $('#pastebin-code-block:first').get(0); \
            if (holder) { Prism.highlightElement(holder); }",
        ),
        static_resources: load_static_resources! {
            "/static/prism.js" => "../../static/prism.js",
            "/static/prism.css" =>"../../static/prism.css",
            "/static/prism-dark.css" =>"../../static/prism-dark.css"
        },
    }
}
