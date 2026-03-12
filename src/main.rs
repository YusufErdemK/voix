use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Entry, Orientation};
use webkit2gtk::{WebView, WebViewExt};

fn main() {
    let app = Application::builder()
        .application_id("com.voix.browser")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Voix Browser")
            .default_width(1280)
            .default_height(800)
            .build();

        let vbox = Box::new(Orientation::Vertical, 0);
        let hbox = Box::new(Orientation::Horizontal, 4);

        let url_bar = Entry::new();
        url_bar.set_placeholder_text(Some("example.com"));
        url_bar.set_hexpand(true);

        let go_btn = Button::with_label("Git");

        let webview = WebView::new();
        webview.set_vexpand(true);
        webview.load_uri("https://www.google.com");

        // Enter
        let wv1 = webview.clone();
        url_bar.connect_activate(move |entry| {
            let mut url = entry.text().to_string();
            if !url.starts_with("http://") && !url.starts_with("https://") {
                url = format!("https://{}", url);
            }
            wv1.load_uri(&url);
        });

        // Buton
        let wv2 = webview.clone();
        let ub2 = url_bar.clone();
        go_btn.connect_clicked(move |_| {
            let mut url = ub2.text().to_string();
            if !url.starts_with("http://") && !url.starts_with("https://") {
                url = format!("https://{}", url);
            }
            wv2.load_uri(&url);
        });

        hbox.pack_start(&url_bar, true, true, 4);
        hbox.pack_start(&go_btn, false, false, 4);
        vbox.pack_start(&hbox, false, false, 4);
        vbox.pack_start(&webview, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}