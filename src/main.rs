fn main() {
    let url = "https://example.com";
    let body = reqwest::blocking::get(url)
        .expect("istek başarısız:(")
        .text()
        .expect("bunasılbodyaq");
    
    println!("{}", body);
}