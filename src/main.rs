use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Entry, Orientation, ScrolledWindow, TextView};

fn main() {
    let app = Application::builder()
        .application_id("com.voix.browser")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Voix Browser")
            .default_width(1024)
            .default_height(768)
            .build();

        let vbox = Box::new(Orientation::Vertical, 4);

        let url_bar = Entry::new();
        url_bar.set_placeholder_text(Some("https://example.com"));

        let text_view = TextView::new();
        text_view.set_editable(false);
        text_view.set_wrap_mode(gtk::WrapMode::Word);

        let scrolled = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled.set_vexpand(true);
        scrolled.add(&text_view);

        let tv_clone = text_view.clone();
        url_bar.connect_activate(move |entry| {
            let url = entry.text().to_string();
            match reqwest::blocking::get(&url).and_then(|r| r.text()) {
                Ok(html) => {
                    let document = scraper::Html::parse_document(&html);
                    let sel = scraper::Selector::parse("p, h1, h2, h3").unwrap();
                    let content: Vec<String> = document
                        .select(&sel)
                        .map(|e| e.text().collect::<String>())
                        .collect();
                    tv_clone.buffer().unwrap().set_text(&content.join("\n\n"));
                }
                Err(e) => {
                    tv_clone.buffer().unwrap().set_text(&format!("Hata: {}", e));
                }
            }
        });

        vbox.pack_start(&url_bar, false, false, 0);
        vbox.pack_start(&scrolled, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}