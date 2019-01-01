use wasp::http::{HttpMapBody, HttpMessage, HttpRequest, HttpResponse};

#[no_mangle]
pub fn run() {
    wasp::set_panic_hook();

    let request: HttpRequest<String> = wasp::http::read_request()
        .expect("Could not read request")
        .map_body();

    HttpResponse::builder()
        .status(200)
        .body(format!(
            "Hello, {}!",
            request.uri().query().unwrap_or_else(|| "World")
        ))
        .expect("Could not build response")
        .send()
        .expect("Could not send response");
}
