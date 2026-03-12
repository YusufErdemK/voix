use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Dialog, Entry,
    EventBox, Label, Menu, MenuItem, Notebook, Orientation, Spinner,
};
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

    let event_box = EventBox::new();
    event_box.add(&tab_hbox);
    event_box.show_all();

    let page_index = notebook.append_page(&webview, Some(&event_box));
    notebook.set_current_page(Some(page_index));
    notebook.show_all();

    // Başlık güncelle
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

    // Kapat butonu
    let nb = notebook.clone();
    let wv = webview.clone();
    close_btn.connect_clicked(move |_| {
        if let Some(idx) = nb.page_num(&wv) {
            nb.remove_page(Some(idx));
        }
    });

    // Sağ tık menüsü
    let nb = notebook.clone();
    let wv = webview.clone();
    let close_clone = close_btn.clone();
    event_box.connect_button_press_event(move |_, event| {
        if event.button() == 3 {
            let menu = Menu::new();
            let pin_item   = MenuItem::with_label("Sekmeyi Sabitle");
            let unpin_item = MenuItem::with_label("Sabitlemeyi Kaldır");
            menu.append(&pin_item);
            menu.append(&unpin_item);
            menu.show_all();

            let nb2 = nb.clone();
            let wv2 = wv.clone();
            let close2 = close_clone.clone();
            pin_item.connect_activate(move |_| {
                close2.hide();
                nb2.set_tab_reorderable(&wv2, false);
                nb2.reorder_child(&wv2, Some(0));
            });

            let nb2 = nb.clone();
            let wv2 = wv.clone();
            let close2 = close_clone.clone();
            unpin_item.connect_activate(move |_| {
                close2.show();
                nb2.set_tab_reorderable(&wv2, true);
            });

            menu.popup_at_pointer(Some(&**event));
            return true.into();
        }
        false.into()
    });
}

fn show_settings(homepage: &std::rc::Rc<std::cell::RefCell<String>>) {
    let dialog = Dialog::with_buttons(
        Some("Voix Ayarları"),
        gtk::Window::NONE,
        gtk::DialogFlags::MODAL,
        &[("Tamam", gtk::ResponseType::Ok), ("İptal", gtk::ResponseType::Cancel)],
    );
    dialog.set_default_size(400, 300);

    let content = dialog.content_area();
    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);

    let hp_label = Label::new(Some("Ana Sayfa:"));
    hp_label.set_halign(gtk::Align::Start);
    let hp_entry = Entry::new();
    hp_entry.set_text(&homepage.borrow());

    let font_label = Label::new(Some("Yazı Boyutu:"));
    font_label.set_halign(gtk::Align::Start);
    let font_box = Box::new(Orientation::Horizontal, 8);
    let small_btn  = Button::with_label("Küçük");
    let medium_btn = Button::with_label("Orta");
    let large_btn  = Button::with_label("Büyük");
    font_box.pack_start(&small_btn,  false, false, 0);
    font_box.pack_start(&medium_btn, false, false, 0);
    font_box.pack_start(&large_btn,  false, false, 0);

    let about_label = Label::new(Some("Hakkında"));
    about_label.set_halign(gtk::Align::Start);
    let about_box = Box::new(Orientation::Vertical, 4);
    let name_label = Label::new(Some("Voix Browser"));
    let ver_label  = Label::new(Some("Versiyon: 0.1.0"));
    let tech_label = Label::new(Some("Rust + GTK3 + WebKitGTK ile yapıldı"));
    name_label.set_halign(gtk::Align::Start);
    ver_label.set_halign(gtk::Align::Start);
    tech_label.set_halign(gtk::Align::Start);
    about_box.pack_start(&name_label, false, false, 0);
    about_box.pack_start(&ver_label,  false, false, 0);
    about_box.pack_start(&tech_label, false, false, 0);

    vbox.pack_start(&hp_label,    false, false, 0);
    vbox.pack_start(&hp_entry,    false, false, 0);
    vbox.pack_start(&font_label,  false, false, 0);
    vbox.pack_start(&font_box,    false, false, 0);
    vbox.pack_start(&about_label, false, false, 0);
    vbox.pack_start(&about_box,   false, false, 0);
    content.pack_start(&vbox, true, true, 0);
    content.show_all();

    let hp_clone = homepage.clone();
    if dialog.run() == gtk::ResponseType::Ok {
        *hp_clone.borrow_mut() = hp_entry.text().to_string();
    }
    dialog.close();
}

fn main() {
    let app = Application::builder()
        .application_id("com.voix.browser")
        .build();

    app.connect_activate(|app| {
        let homepage = std::rc::Rc::new(std::cell::RefCell::new(
            "https://www.google.com".to_string()
        ));

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Voix")
            .default_width(1280)
            .default_height(800)
            .build();

        let vbox = Box::new(Orientation::Vertical, 0);
        let hbox = Box::new(Orientation::Horizontal, 4);

        let back_btn     = Button::with_label("◀");
        let forward_btn  = Button::with_label("▶");
        let reload_btn   = Button::with_label("↺");
        let new_tab_btn  = Button::with_label("+");
        let settings_btn = Button::with_label("⚙");
        let url_bar      = Entry::new();
        let spinner      = Spinner::new();

        url_bar.set_placeholder_text(Some("Ara veya adres yaz..."));
        url_bar.set_hexpand(true);
        back_btn.set_sensitive(false);
        forward_btn.set_sensitive(false);

        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_scrollable(true);

        new_tab(&notebook, &homepage.borrow());

        let get_webview = |nb: &Notebook| -> Option<WebView> {
            let idx = nb.current_page()?;
            let widget = nb.nth_page(Some(idx))?;
            widget.downcast::<WebView>().ok()
        };

        let nb = notebook.clone();
        back_btn.connect_clicked(move |_| {
            if let Some(wv) = get_webview(&nb) { wv.go_back(); }
        });

        let nb = notebook.clone();
        forward_btn.connect_clicked(move |_| {
            if let Some(wv) = get_webview(&nb) { wv.go_forward(); }
        });

        let nb = notebook.clone();
        reload_btn.connect_clicked(move |_| {
            if let Some(wv) = get_webview(&nb) { wv.reload(); }
        });

        let nb = notebook.clone();
        new_tab_btn.connect_clicked(move |_| {
            new_tab(&nb, "https://www.google.com");
        });

        let hp = homepage.clone();
        settings_btn.connect_clicked(move |_| {
            show_settings(&hp);
        });

        let nb   = notebook.clone();
        let sp   = spinner.clone();
        let back = back_btn.clone();
        let fwd  = forward_btn.clone();
        let ub   = url_bar.clone();

        url_bar.connect_activate(move |entry| {
            if let Some(wv) = get_webview(&nb) {
                navigate(&wv, &entry.text());

                let ub2   = ub.clone();
                let sp2   = sp.clone();
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

        let ub = url_bar.clone();
        let nb = notebook.clone();
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

        hbox.pack_start(&back_btn,     false, false, 2);
        hbox.pack_start(&forward_btn,  false, false, 2);
        hbox.pack_start(&reload_btn,   false, false, 2);
        hbox.pack_start(&url_bar,      true,  true,  4);
        hbox.pack_start(&spinner,      false, false, 4);
        hbox.pack_start(&new_tab_btn,  false, false, 2);
        hbox.pack_start(&settings_btn, false, false, 2);
        vbox.pack_start(&hbox,     false, false, 4);
        vbox.pack_start(&notebook, true,  true,  0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}