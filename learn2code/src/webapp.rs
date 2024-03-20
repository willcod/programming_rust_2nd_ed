use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

use learn2code::utils::gcd;

pub async fn webapp_main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Listening on http://localhost:3000/");

    let _ = server
        .bind("127.0.0.1:3000")
        .expect("failed to bind to address")
        .run()
        .await
        .expect("failed to start server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
            "#,
        )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html; charset=utf-8")
            .body("should not be 0");
    }

    let response = format!(
        "The greatest common divisor of {} and {} is {}",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}
