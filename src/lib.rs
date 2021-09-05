use worker::*;

mod utils;

fn log_request(req: &Request) {
  console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
  log_request(&req);

  // Optionally, get more helpful error messages written to the console in the case of a panic.
  utils::set_panic_hook();

  // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
  // catch-alls to match on specific patterns. The Router takes some data with its `new` method
  // that can be shared throughout all routes. If you don't need any shared data, use `()`.
  let router = Router::new(());

  // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
  // functionality and a `RouteContext` which you can use to  and get route parameters and
  // Enviornment bindings like KV Stores, Durable Objects, Secrets, and Variables.
  let result = router
    .get_async("/:url", |mut _req, ctx| async move {
      console_log!("Looking for url");

      if let Some(name) = ctx.param("url") {
        console_log!("Looking for url: {}", name);

        if let Some(value) = ctx.kv("urls")?.get(name).await? {
          console_log!("Found KV for url: {}", name);

          let location = value.as_string();

          console_log!("Redirecting to: {}", location);

          let mut headers = Headers::new();

          headers.append("Location", &location)?;

          console_log!("Responding...");
          Response::empty()
            .map(|r| r.with_headers(headers).with_status(301))
        } else {
          console_log!("No url for: {}", name);
          Response::error("Url not found", 404)
        }
      } else {
        Response::error("Bad Request", 400)
      }
    })
    .run(req, env)
    .await;

  console_log!("Responding: {:?}", result);

  result
}
