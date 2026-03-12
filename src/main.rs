use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Entry, Orientation, Spinner};
use webkit2gtk::{LoadEvent, WebView, WebViewExt};

fn navigate(webview: &WebView, input: &str) {
    let url = if input.starts_with("http://") || input.starts_with("https://") {
        input.to_string()
    } else if input.contains('.') && !input.contains(' ') {
        format!("https://{}", input)
    } else {
        format!("https://www.google.com/search?q={}", input.replace(' ', "+"))
    };
    webview.load_uri(&url);
}

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

        let back_btn    = Button::with_label("◀");
        let forward_btn = Button::with_label("▶");
        let reload_btn  = Button::with_label("↺");
        let url_bar     = Entry::new();
        let spinner     = Spinner::new();

        url_bar.set_placeholder_text(Some("Ara veya adres yaz..."));
        url_bar.set_hexpand(true);
        back_btn.set_sensitive(false);
        forward_btn.set_sensitive(false);

        let webview = WebView::new();
        webview.set_vexpand(true);
        webview.load_uri("https://www.google.com");

        let wv = webview.clone();
        back_btn.connect_clicked(move |_| { wv.go_back(); });

        let wv = webview.clone();
        forward_btn.connect_clicked(move |_| { wv.go_forward(); });

        let wv = webview.clone();
        reload_btn.connect_clicked(move |_| { wv.reload(); });

        let wv = webview.clone();
        url_bar.connect_activate(move |entry| {
            navigate(&wv, &entry.text());
        });

        let ub   = url_bar.clone();
        let sp   = spinner.clone();
        let back = back_btn.clone();
        let fwd  = forward_btn.clone();
        webview.connect_load_changed(move |wv, event| {
            match event {
                LoadEvent::Started   => { sp.start(); }
                LoadEvent::Committed => {
                    if let Some(uri) = wv.uri() { ub.set_text(&uri); }
                }
                LoadEvent::Finished  => {
                    sp.stop();
                    back.set_sensitive(wv.can_go_back());
                    fwd.set_sensitive(wv.can_go_forward());
                }
                _ => {}
            }
        });

        let ub = url_bar.clone();
        window.connect_key_press_event(move |_, key| {
            if key.state().contains(gtk::gdk::ModifierType::CONTROL_MASK)
                && key.keyval() == gtk::gdk::keys::constants::l
            {
                ub.grab_focus();
                return true.into();
            }
            false.into()
        });

        hbox.pack_start(&back_btn,    false, false, 2);
        hbox.pack_start(&forward_btn, false, false, 2);
        hbox.pack_start(&reload_btn,  false, false, 2);
        hbox.pack_start(&url_bar,     true,  true,  4);
        hbox.pack_start(&spinner,     false, false, 4);
        vbox.pack_start(&hbox,    false, false, 4);
        vbox.pack_start(&webview, true,  true,  0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}