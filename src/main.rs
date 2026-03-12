use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Entry, Notebook, Orientation, Spinner, Label};
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

fn new_tab(notebook: &Notebook, url: &str) {
    let webview = WebView::new();
    webview.set_vexpand(true);
    webview.load_uri(url);

    let label = Label::new(Some("Yeni Sekme"));
    let tab_hbox = Box::new(Orientation::Horizontal, 4);
    let close_btn = Button::with_label("✕");
    close_btn.set_relief(gtk::ReliefStyle::None);

    tab_hbox.pack_start(&label, true, true, 0);
    tab_hbox.pack_start(&close_btn, false, false, 0);
    tab_hbox.show_all();

    let page_index = notebook.append_page(&webview, Some(&tab_hbox));
    notebook.set_current_page(Some(page_index));
    notebook.show_all();

    // Sekme başlığını güncelle
    let label_clone = label.clone();
    webview.connect_load_changed(move |wv, event| {
        if event == LoadEvent::Finished {
            if let Some(title) = wv.title() {
                let short = if title.len() > 20 {
                    format!("{}...", &title[..20])
                } else {
                    title.to_string()
                };
                label_clone.set_text(&short);
            }
        }
    });

    // Sekmeyi kapat
    let nb = notebook.clone();
    let wv = webview.clone();
    close_btn.connect_clicked(move |_| {
        if let Some(idx) = nb.page_num(&wv) {
            nb.remove_page(Some(idx));
        }
    });
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
        let new_tab_btn = Button::with_label("+");
        let url_bar     = Entry::new();
        let spinner     = Spinner::new();

        url_bar.set_placeholder_text(Some("Ara veya adres yaz..."));
        url_bar.set_hexpand(true);
        back_btn.set_sensitive(false);
        forward_btn.set_sensitive(false);

        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_scrollable(true);

        // İlk tab
        new_tab(&notebook, "https://www.google.com");

        // Aktif webview'i al
        let get_webview = |nb: &Notebook| -> Option<WebView> {
            let idx = nb.current_page()?;
            let widget = nb.nth_page(Some(idx))?;
            widget.downcast::<WebView>().ok()
        };

        // Geri
        let nb = notebook.clone();
        back_btn.connect_clicked(move |_| {
            if let Some(wv) = get_webview(&nb) { wv.go_back(); }
        });

        // İleri
        let nb = notebook.clone();
        forward_btn.connect_clicked(move |_| {
            if let Some(wv) = get_webview(&nb) { wv.go_forward(); }
        });

        // Yenile
        let nb = notebook.clone();
        reload_btn.connect_clicked(move |_| {
            if let Some(wv) = get_webview(&nb) { wv.reload(); }
        });

        // Yeni tab butonu
        let nb = notebook.clone();
        new_tab_btn.connect_clicked(move |_| {
            new_tab(&nb, "https://www.google.com");
        });

        // URL bar Enter
        let nb = notebook.clone();
        let sp = spinner.clone();
        let back = back_btn.clone();
        let fwd  = forward_btn.clone();
        let ub   = url_bar.clone();

        url_bar.connect_activate(move |entry| {
            if let Some(wv) = get_webview(&nb) {
                navigate(&wv, &entry.text());

                let ub2  = ub.clone();
                let sp2  = sp.clone();
                let back2 = back.clone();
                let fwd2  = fwd.clone();
                wv.connect_load_changed(move |wv, event| {
                    match event {
                        LoadEvent::Started   => { sp2.start(); }
                        LoadEvent::Committed => {
                            if let Some(uri) = wv.uri() { ub2.set_text(&uri); }
                        }
                        LoadEvent::Finished  => {
                            sp2.stop();
                            back2.set_sensitive(wv.can_go_back());
                            fwd2.set_sensitive(wv.can_go_forward());
                        }
                        _ => {}
                    }
                });
            }
        });

        // Ctrl+L, Ctrl+T, Ctrl+W
        let ub  = url_bar.clone();
        let nb  = notebook.clone();
        window.connect_key_press_event(move |_, key| {
            let ctrl = key.state().contains(gtk::gdk::ModifierType::CONTROL_MASK);
            if ctrl && key.keyval() == gtk::gdk::keys::constants::l {
                ub.grab_focus();
                return true.into();
            }
            if ctrl && key.keyval() == gtk::gdk::keys::constants::t {
                new_tab(&nb, "https://www.google.com");
                return true.into();
            }
            if ctrl && key.keyval() == gtk::gdk::keys::constants::w {
                if let Some(idx) = nb.current_page() {
                    nb.remove_page(Some(idx));
                }
                return true.into();
            }
            false.into()
        });

        hbox.pack_start(&back_btn,    false, false, 2);
        hbox.pack_start(&forward_btn, false, false, 2);
        hbox.pack_start(&reload_btn,  false, false, 2);
        hbox.pack_start(&url_bar,     true,  true,  4);
        hbox.pack_start(&spinner,     false, false, 4);
        hbox.pack_start(&new_tab_btn, false, false, 2);
        vbox.pack_start(&hbox,     false, false, 4);
        vbox.pack_start(&notebook, true,  true,  0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}