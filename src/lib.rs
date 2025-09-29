use worker::*;

mod utils;

fn log_request(req: &Request) {
  console_log!(
    "{} - [{}], located at: {:?}, within: {}",
    Date::now().to_string(),
    req.path(),
    req.cf().map(|c| c.colo()).unwrap_or_default(),
    req
      .cf()
      .and_then(|c| c.country())
      .unwrap_or("unknown region".into())
  );
}

#[event(fetch)]
pub async fn main(
  req: Request,
  env: Env,
  _ctx: Context,
) -> Result<Response> {
  log_request(&req);

  // Optionally, get more helpful error messages written to the console in the case of a panic.
  utils::set_panic_hook();

  // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
  // catch-alls to match on specific patterns. The Router takes some data with its `new` method
  // that can be shared throughout all routes. If you don't need any shared data, use `()`.
  let router = Router::new();

  // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
  // functionality and a `RouteContext` which you can use to  and get route parameters and
  // Enviornment bindings like KV Stores, Durable Objects, Secrets, and Variables.
  let result = router
    .get_async("/", |_req, ctx| async move {
      let headers = Headers::new();
      let home_redirect = ctx.var("HOME")?;
      headers.append("Location", &home_redirect.to_string())?;

      // Add cache control and security headers
      headers.append("Cache-Control", "private, max-age=90")?;
      headers.append("X-Frame-Options", "DENY")?;
      headers.append("X-Content-Type-Options", "nosniff")?;

      // Use 302 (temporary) redirect for URL shortener
      Response::empty().map(|r| r.with_headers(headers).with_status(302))
    })
    .get_async("/:url", |_req, ctx| async move {
      console_log!("Looking for url");

      if let Some(name) = ctx.param("url") {
        console_log!("Looking for url: {}", name);

        if let Some(location) = ctx.kv("urls")?.get(name).text().await? {
          console_log!("Found KV for url: {}", name);
          console_log!("Redirecting to: {}", location);

          // Validate URL format
          if !location.starts_with("http://") && !location.starts_with("https://") {
            console_log!("Invalid URL format: {}", location);
            let error_response = serde_json::json!({
              "error": "Invalid URL format",
              "message": "Stored URL must start with http:// or https://"
            });
            return Response::from_json(&error_response).map(|r| r.with_status(500));
          }

          let headers = Headers::new();
          headers.append("Location", &location)?;

          // Add cache control and security headers
          headers.append("Cache-Control", "private, max-age=90")?;
          headers.append("X-Frame-Options", "DENY")?;
          headers.append("X-Content-Type-Options", "nosniff")?;

          console_log!("Responding...");
          // Use 302 (temporary) redirect for URL shortener
          Response::empty().map(|r| r.with_headers(headers).with_status(302))
        } else {
          console_log!("No url for: {}", name);
          let error_response = serde_json::json!({
            "error": "URL not found",
            "message": format!("The short URL '{}' was not found", name)
          });
          Response::from_json(&error_response).map(|r| r.with_status(404))
        }
      } else {
        let error_response = serde_json::json!({
          "error": "Bad Request",
          "message": "URL parameter is required"
        });
        Response::from_json(&error_response).map(|r| r.with_status(400))
      }
    })
    .run(req, env)
    .await;

  console_log!("Responding: {:?}", result);

  result
}
