use gtk4::{prelude::*, Application, Button, Entry, Notebook};
use url::Url;
use webkit6::prelude::*;
use webkit6::WebView;

use crate::tabs::add_tab;

pub fn get_webview(notebook: &Notebook) -> Option<WebView> {
    let current_page = notebook.current_page();

    if let Some(widget) = notebook.nth_page(current_page) {
        if let Some(webview) = widget.downcast_ref::<WebView>() {
            return Some(webview.clone());
        }
    }

    return None;
}

pub fn back_button_clicked(notebook: &Notebook, back_button: &Button) {
    back_button.connect_clicked({
        let notebook = notebook.clone();
        move |_| match get_webview(&notebook) {
            Some(webview) => {
                if webview.can_go_back() {
                    webview.go_back();
                }
            }
            None => {
                println!("Current tab doesn't have a webview")
            }
        }
    });
}

pub fn forward_button_clicked(notebook: &Notebook, forward_button: &Button) {
    forward_button.connect_clicked({
        let notebook = notebook.clone();
        move |_| match get_webview(&notebook) {
            Some(webview) => {
                if webview.can_go_forward() {
                    webview.go_forward();
                }
            }
            None => {
                println!("Current tab doesn't have a webview")
            }
        }
    });
}

pub fn refresh_button_clicked(notebook: &Notebook, refresh_button: &Button) {
    refresh_button.connect_clicked({
        let notebook = notebook.clone();
        move |_| match get_webview(&notebook) {
            Some(webview) => {
                webview.reload();
            }
            None => {
                println!("Current tab doesn't have a webview")
            }
        }
    });
}

pub fn new_tab_button_clicked(
    notebook: &Notebook,
    new_tab_button: &Button,
    search_entry: &Entry,
    app: &Application,
) {
    let notebook = notebook.clone();
    let search_entry = search_entry.clone();
    let app = app.clone();

    new_tab_button.connect_clicked(move |_| {
        let notebook = notebook.clone();
        let search_entry = search_entry.clone();

        add_tab(&notebook, &search_entry, None, &app);
    });
}

pub fn search_entry_activate(search_entry: &Entry, notebook: &Notebook) {
    search_entry.connect_activate({
        let notebook = notebook.clone();

        move |search_entry| {
            let url = search_entry.text();

            if url.is_empty() {
                return;
            }

            match get_webview(&notebook) {
                Some(webview) => {
                    let url_str = url.as_str();

                    if let Ok(url) = Url::parse(url_str) {
                        if url.scheme() == "http" || url.scheme() == "https" {
                            if url.host_str() == Some("localhost") || url.path() == "/" {
                                println!("Local URL detected!");
                                webview.load_uri(&url_str);
                                return;
                            }
                        } else if url.scheme() == "file" {
                            println!("File URL detected!");
                            webview.load_uri(&url_str);
                            return;
                        }

                        webview.load_uri(&url_str);

                        return;
                    }

                    let domain_like = url_str.contains('.') && !url_str.contains(' ');

                    if domain_like {
                        println!("URL detected (no scheme)!");
                        webview.load_uri(&format!("https://{}", &url_str));

                        return;
                    }

                    println!("Search query detected");
                    let search_query = url.to_string().replace(" ", "+");
                    webview.load_uri(&format!("https://duckduckgo.com/?q={}", &search_query));
                    return;
                }
                None => println!("Current tab doesn't have a webview"),
            }
        }
    });
}

pub fn notebook_switch_page(notebook: &Notebook, search_entry: &Entry) {
    notebook.connect_switch_page({
        let search_entry = search_entry.clone();

        move |notebook, _, page_num| {
            if let Some(widget) = notebook.nth_page(Some(page_num)) {
                if let Some(webview) = widget.downcast_ref::<WebView>() {
                    let uri = webview.uri().unwrap();
                    search_entry.set_text(&uri);
                }
            }
        }
    });
}
