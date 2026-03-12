use scraper::{Html, Selector};

fn main() {
    let url = "https://example.com";
    let body = reqwest::blocking::get(url)
        .expect("Request başarısız")
        .text()
        .expect("Body okunamadı");

    let document = Html::parse_document(&body);

    let h1 = Selector::parse("h1").unwrap();
    for element in document.select(&h1) {
        println!("H1: {}", element.text().collect::<String>());
    }

    let p = Selector::parse("p").unwrap();
    for element in document.select(&p) {
        println!("P: {}", element.text().collect::<String>());
    }
}