extern crate gtk4;

use gtk4::prelude::*;
use gtk4::CssProvider;

pub fn apply_css_style(widgets: &[&gtk4::Widget], css: &str) {
    let provider = CssProvider::new();
    provider.load_from_data(css);

    for widget in widgets {
        let context = widget.style_context();
        context.add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }
}
