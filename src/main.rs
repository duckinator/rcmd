#[macro_use]
extern crate rouille;
use rouille::Response;

fn url_for(q: String) -> String {
    let parts: Vec<&str> = q.split_whitespace().collect();
    let cmd = parts[0];
    let args = parts[1..].join(" ");

    println!("q={}", q);
    match cmd {
        "g"|"google"    => format!("https://google.com/search?q={}", args),
        _               => format!("/?invalid=true&q={}", cmd),
    }
}

fn search(request: &rouille::Request) -> rouille::Response {
    if let Some(q) = request.get_param("q") {
        Response::redirect_303(url_for(q))
    } else {
        Response::text("No query provided.")
    }
}

fn main() {
    println!("Listening on localhost:8000.");
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
                (GET) (/)       => { Response::html(FORM) },
                (GET) (/search) => { search(request) },
                _ => Response::empty_404()
        )
    })
}

static FORM: &'static str =
r#"<!doctype html>
<form action="/search">
    <input type="text" name="q" id="q">
    <input type="submit" value="Go">
</form>
"#;
