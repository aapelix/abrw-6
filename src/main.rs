use gtk4::glib::Propagation;
use gtk4::prelude::ApplicationExtManual;
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Button;
use gtk4::Entry;
use gtk4::Notebook;
use gtk4::{prelude::ApplicationExt, Application};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use styles::apply_css_style;
use tabs::add_tab;
use webkit6::WebContext;

mod connections;
mod styles;
mod tabs;

static WINDOW_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    std::env::set_var("GDK_BACKEND", "x11");

    let app = Application::builder()
        .application_id("com.aapelix.abrw-6")
        .build();

    app.connect_activate(|app| {
        create_window(None, &app);
    });

    app.run();
}

pub fn create_window(default_tab_url: Option<&str>, app: &Application) {
    WINDOW_COUNT.fetch_add(1, Ordering::SeqCst);

    let adblock_enabled = Rc::new(RefCell::new(true));

    let context = WebContext::default().unwrap();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("abrw")
        .default_width(1500)
        .default_height(900)
        .build();

    let hbox = Box::new(gtk4::Orientation::Vertical, 0);
    let top_bar = Box::new(gtk4::Orientation::Horizontal, 0);

    let control_buttons_box = Box::new(gtk4::Orientation::Horizontal, 0);

    let back_button = create_button_with_icon("<");
    let forward_button = create_button_with_icon(">");
    let refresh_button = create_button_with_icon("R");

    control_buttons_box.append(&back_button);
    control_buttons_box.append(&forward_button);
    control_buttons_box.append(&refresh_button);

    apply_css_style(
        &[
            &back_button.upcast_ref(),
            &forward_button.upcast_ref(),
            &refresh_button.upcast_ref(),
        ],
        "
        button { background: transparent; border: none; box-shadow: none; }
        button:hover { background: #2a2a2a; }
        ",
    );

    let search_bar = Entry::new();

    control_buttons_box.set_halign(gtk4::Align::Start);

    let menu_buttons_box = Box::new(gtk4::Orientation::Horizontal, 0);

    let new_tab_button = create_button_with_icon("+");
    let download_button = create_button_with_icon("D");

    let menu_button = create_button_with_icon("A");
    let settings_button = create_button_with_icon("|");

    menu_buttons_box.append(&download_button);
    menu_buttons_box.append(&menu_button);
    menu_buttons_box.append(&settings_button);

    let notebook = Notebook::new();
    notebook.set_action_widget(&new_tab_button, gtk4::PackType::End);

    new_tab_button.show();

    notebook.set_show_border(false);

    apply_css_style(
        &[
            &new_tab_button.upcast_ref(),
            &download_button.upcast_ref(),
            &menu_button.upcast_ref(),
            &settings_button.upcast_ref(),
        ],
        "
        button { background: transparent; border: none; box-shadow: none; }
        button:hover { background: #2a2a2a; }
        ",
    );

    top_bar.append(&control_buttons_box);
    top_bar.append(&search_bar);
    top_bar.append(&menu_buttons_box);

    match default_tab_url {
        Some(url) => search_bar.set_text(&url),
        None => search_bar.set_text(""),
    }

    search_bar.set_halign(gtk4::Align::Fill);
    search_bar.set_hexpand(true);

    hbox.append(&top_bar);
    hbox.append(&notebook);
    notebook.set_scrollable(true);

    match default_tab_url {
        Some(url) => add_tab(&notebook, &search_bar, Some(url), app),
        None => add_tab(&notebook, &search_bar, None, app),
    }

    apply_css_style(
        &[
            &hbox.upcast_ref(),
            &search_bar.upcast_ref(),
            &notebook.upcast_ref(),
        ],
        "
        box { background: #202020; }
        entry { background: #2a2a2a; border-color: #2d2d2d; margin-bottom: 5px; }
        notebook header.top { background: #202020; box-shadow: none; }
        notebook header.top action-widget { background: #2a2a2a; padding: 5px; box-shadow: none; }
        notebook header.top tabs { background: #202020; }
        notebook header.top tabs tab {
            min-height: 15px;
            min-width: 100px;
            background: transparent;
            border: none;
            border-radius: 7px;
            margin: 4px;
            padding: 2px;
            padding-left: 10px;
            padding-right: 10px;
            transition-duration: 300ms;
        }
        notebook header.top tabs tab:checked { background: #2a2a2a; }
        notebook header.top tabs tab.reorderable-page { box-shadow: none; }
        ",
    );

    window.set_child(Some(&hbox));

    connections::back_button_clicked(&notebook, &back_button);
    connections::forward_button_clicked(&notebook, &forward_button);
    connections::refresh_button_clicked(&notebook, &refresh_button);
    connections::new_tab_button_clicked(&notebook, &new_tab_button, &search_bar, app);
    connections::search_entry_activate(&search_bar, &notebook);
    connections::notebook_switch_page(&notebook, &search_bar);

    hbox.show();

    window.present();
    window.show();
}

pub fn create_button_with_icon(str: &str) -> Button {
    let button = Button::with_label(str);

    button
}
