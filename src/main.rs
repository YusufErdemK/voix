use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, CssProvider, Entry,
    HeaderBar, Menu, MenuItem, Notebook, Orientation, Spinner,
    StyleContext, gdk::Screen, EventBox, Box, Label,
};
use webkit2gtk::{LoadEvent, WebView, WebViewExt};
use std::rc::Rc;
use std::cell::RefCell;

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

fn get_webview(nb: &Notebook) -> Option<WebView> {
    let idx = nb.current_page()?;
    nb.nth_page(Some(idx))?.downcast::<WebView>().ok()
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

    // Kapat
    let nb = notebook.clone();
    let wv = webview.clone();
    close_btn.connect_clicked(move |_| {
        if let Some(idx) = nb.page_num(&wv) {
            nb.remove_page(Some(idx));
        }
    });

    // Sağ tık — sabitle
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
            let c2  = close_clone.clone();
            pin_item.connect_activate(move |_| {
                c2.hide();
                nb2.set_tab_reorderable(&wv2, false);
                nb2.reorder_child(&wv2, Some(0));
            });

            let nb2 = nb.clone();
            let wv2 = wv.clone();
            let c2  = close_clone.clone();
            unpin_item.connect_activate(move |_| {
                c2.show();
                nb2.set_tab_reorderable(&wv2, true);
            });

            menu.popup_at_pointer(Some(&**event));
            return true.into();
        }
        false.into()
    });
}

fn main() {
    let app = Application::builder()
        .application_id("com.voix.browser")
        .build();

    app.connect_startup(|_| {
        let provider = CssProvider::new();
        let css = "
            headerbar {
                background: @theme_bg_color;
                border-bottom: 1px solid rgba(0,0,0,0.1);
                padding: 6px;
            }
            entry#url-bar {
                border-radius: 16px;
                background-color: #252525;
                border: 1px solid rgba(0,0,0,0.12);
                color: #d4d4d4;
                padding: 5px 12px;
                box-shadow: none;
            }
            entry#url-bar:focus {
                background-color: #383838;
                border: 1px solid rgba(0,0,0,0.2);
            }
            button.image-button,
            button.image-button:hover,
            button.image-button:active,
            button.image-button:focus {
                border: none;
                border-radius: 6px;
                background-color: transparent;
                box-shadow: none;
                outline: none;
                transition: none;
                -gtk-icon-shadow: none;
            }
            button.image-button:hover {
                background-color: rgba(128,128,128,0.15);
            }
            notebook tab {
                border: none;
                padding: 8px 16px;
            }
            notebook tab:checked {
                background-color: rgba(0,0,0,0.03);
                border-bottom: 2px solid #a0a0a0;
            }
        ";
        provider.load_from_data(css.as_bytes()).unwrap();
        StyleContext::add_provider_for_screen(
            &Screen::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Voix")
            .default_width(1280)
            .default_height(800)
            .build();

        let header_bar = HeaderBar::new();
        header_bar.set_show_close_button(true);

        let back_btn    = Button::from_icon_name(Some("go-previous-symbolic"), gtk::IconSize::Button);
        let forward_btn = Button::from_icon_name(Some("go-next-symbolic"), gtk::IconSize::Button);
        let reload_btn  = Button::from_icon_name(Some("view-refresh-symbolic"), gtk::IconSize::Button);
        let new_tab_btn = Button::from_icon_name(Some("tab-new-symbolic"), gtk::IconSize::Button);
        let url_bar     = Entry::new();
        let spinner     = Spinner::new();

        url_bar.set_widget_name("url-bar");
        url_bar.set_placeholder_text(Some("Adres..."));
        url_bar.set_alignment(0.5);
        url_bar.set_hexpand(true);

        header_bar.pack_start(&back_btn);
        header_bar.pack_start(&forward_btn);
        header_bar.pack_start(&reload_btn);
        header_bar.set_custom_title(Some(&url_bar));
        header_bar.pack_end(&new_tab_btn);
        header_bar.pack_end(&spinner);
        window.set_titlebar(Some(&header_bar));

        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_scrollable(true);
        new_tab(&notebook, "https://www.google.com");

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

        let nb = notebook.clone();
        let ub = url_bar.clone();
        let sp = spinner.clone();
        let bc = back_btn.clone();
        let fc = forward_btn.clone();
        url_bar.connect_activate(move |entry| {
            if let Some(wv) = get_webview(&nb) {
                navigate(&wv, &entry.text());
                let ub2 = ub.clone();
                let sp2 = sp.clone();
                let bc2 = bc.clone();
                let fc2 = fc.clone();
                wv.connect_load_changed(move |wv, event| {
                    match event {
                        LoadEvent::Started   => { sp2.start(); }
                        LoadEvent::Committed => {
                            if let Some(uri) = wv.uri() { ub2.set_text(&uri); }
                        }
                        LoadEvent::Finished  => {
                            sp2.stop();
                            bc2.set_sensitive(wv.can_go_back());
                            fc2.set_sensitive(wv.can_go_forward());
                        }
                        _ => {}
                    }
                });
            }
        });

        // Ctrl+L, Ctrl+T, Ctrl+W
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

        window.add(&notebook);
        window.show_all();
    });

    app.run();
}