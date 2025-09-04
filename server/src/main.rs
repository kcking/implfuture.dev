use std::collections::HashMap;
use std::convert::Infallible;
use std::marker::PhantomData;

use anyhow::Result;
use axum::body::{Body, BoxBody};
use axum::extract::Query;
use axum::http::{header, HeaderMap, HeaderValue, Request, Response, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::routing::{get_service, MethodRouter};
use axum::Extension;
use axum::{routing::get, Router};
use futures::future::BoxFuture;
use futures::ready;
use implfuture::ServerAppProps;
use once_cell::sync::Lazy;
use serde_json::Value as JsonValue;
use tokio_util::task::LocalPoolHandle;
use tower::Service;
use tower_http::services::ServeDir;
use yew_router::Routable;

lazy_static::lazy_static!(
    // Use the source HTML as a template; inject built assets from manifest.
    static ref INDEX_HTML: String = {
        String::from_utf8(std::fs::read("bundle/index.html").unwrap().try_into().unwrap()).unwrap()
    };
    static ref APP_WASM_PATH: &'static str = {
        option_env!("APP_WASM_PATH").unwrap_or("/app_wasm_bg.wasm")
    };
    static ref APP_JS_PATH: &'static str = {
        option_env!("APP_JS_PATH").unwrap_or("/app_wasm.js")
    };
    static ref BUNDLE_ASSETS: Option<(String, Vec<String>)> = {
        // Parse Vite manifest for entry js and css
        match std::fs::read_to_string("bundle/dist/manifest.json") {
            Ok(contents) => {
                let manifest: JsonValue = match serde_json::from_str(&contents) {
                    Ok(v) => v,
                    Err(_) => return None,
                };
                // Find an entry with isEntry = true; prefer key ending in index.html or index.ts/tsx/js
                let mut chosen: Option<&JsonValue> = None;
                let mut chosen_key_score: i32 = -1;
                if let Some(obj) = manifest.as_object() {
                    for (k, v) in obj.iter() {
                        let is_entry = v.get("isEntry").and_then(|b| b.as_bool()).unwrap_or(false);
                        if !is_entry { continue; }
                        // scoring: prefer keys that look like index
                        let score = if k.ends_with("index.html") { 3 }
                            else if k.ends_with("index.ts") || k.ends_with("index.tsx") { 2 }
                            else if k.ends_with("index.js") || k.ends_with("index.jsx") { 2 }
                            else { 1 };
                        if score > chosen_key_score {
                            chosen = Some(v);
                            chosen_key_score = score;
                        }
                    }
                }
                if let Some(entry) = chosen {
                    let js_file = entry.get("file").and_then(|s| s.as_str()).map(|s| format!("/{}", s));
                    let css_files: Vec<String> = entry
                        .get("css")
                        .and_then(|arr| arr.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| format!("/{}", s))).collect())
                        .unwrap_or_default();
                    if let Some(js) = js_file { Some((js, css_files)) } else { None }
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    };
);

static LOCAL_POOL: Lazy<LocalPoolHandle> = Lazy::new(|| LocalPoolHandle::new(num_cpus::get()));

fn html_wasm_init_head() -> String {
    format!(
        r#"
    <script type="module">
      import init from "{js_path}";
      init("{wasm_path}");
    </script>
"#,
        js_path = *APP_JS_PATH,
        wasm_path = *APP_WASM_PATH,
    )
}

fn bundle_head_tags() -> String {
    if let Some((ref js, ref css_list)) = *BUNDLE_ASSETS {
        let mut out = String::new();
        for css in css_list {
            out.push_str(&format!("\n    <link rel=\"stylesheet\" href=\"{}\" />", css));
        }
        out.push_str(&format!("\n    <script type=\"module\" src=\"{}\"></script>\n", js));
        out
    } else {
        // Fallback to dev index.js if manifest missing
        "\n    <script type=\"module\" src=\"/index.js\"></script>\n".to_string()
    }
}

async fn index(
    Extension(index_html_s): Extension<String>,
    url: Request<Body>,
    Query(queries): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let out = LOCAL_POOL
        .spawn_pinned(|| async move {
            let props = ServerAppProps {
                path: url.uri().path().to_owned().into(),
                queries,
            };
            let mut out = String::new();
            yew::ServerRenderer::<implfuture::ServerApp>::with_props(props)
                .render_to_string(&mut out)
                .await;
            out
        })
        .await
        .unwrap();
    // Remove dev script tag if present to avoid duplicate loads
    let cleaned = index_html_s.replace("<script type=\"module\" src=\"/index.js\"></script>", "");
    let html = cleaned
        .replace("<body>", &format!("<body>{}", out))
        .replace("</head>", &format!("{}{}</head>", bundle_head_tags(), html_wasm_init_head()));
    (
        HeaderMap::from_iter([(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"))]),
        Html(html),
    )
}

async fn handle_error(e: impl std::fmt::Debug) -> impl IntoResponse {
    eprintln!("{e:?}");
    StatusCode::BAD_REQUEST
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app_wasm_serve = ServeDir::new(".");
    if option_env!("AXUM_PRECOMPRESSED_WASM").is_some() {
        app_wasm_serve = app_wasm_serve.precompressed_br();
    }
    let app_wasm_serve = get_service(app_wasm_serve).handle_error(handle_error);
    let static_serve = get_service(ServeDir::new("static")).handle_error(handle_error);
    let dist_serve = get_service(ServeDir::new("bundle/dist")).handle_error(handle_error);
    let route_service = RoutableService::<implfuture::Route, _, _>::new(
        get(index),
        route(*APP_JS_PATH, app_wasm_serve.clone())
            .route(*APP_WASM_PATH, app_wasm_serve)
            // Serve built assets from Vite dist first
            .fallback(dist_serve)
            // Fallback to legacy static dir
            .fallback(static_serve),
    );
    let route_service = get_service(route_service).layer(Extension(INDEX_HTML.to_string()));

    if lambda_web::is_running_on_lambda() {
        eprintln!("starting server on lambda");
        lambda_web::run_hyper_on_lambda(route_service)
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    } else {
        let addr = std::env::var("HTTP_LISTEN_ADDR").unwrap_or("127.0.0.1:8080".into());
        eprintln!("starting server on {}", addr);
        axum::Server::bind(&addr.parse()?)
            .serve(get_service(route_service).into_make_service())
            .await?;
    }

    Ok(())
}

#[derive(Clone)]
struct RoutableService<R, S: Clone, F: Clone> {
    r: PhantomData<R>,
    s_ready: bool,
    s: S,
    f_ready: bool,
    f: F,
}

impl<R, S: Clone, F: Clone> RoutableService<R, S, F> {
    pub fn new(s: S, f: F) -> Self {
        Self {
            s,
            f,
            s_ready: false,
            f_ready: false,
            r: PhantomData,
        }
    }
}

impl<R, S, F> Service<Request<Body>> for RoutableService<R, S, F>
where
    R: Routable,
    S: Service<Request<Body>, Error = Infallible> + Clone,
    S::Response: IntoResponse,
    S::Future: Send + 'static,
    F: Service<Request<Body>, Error = Infallible> + Clone,
    F::Response: IntoResponse,
    F::Future: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        loop {
            match (self.s_ready, self.f_ready) {
                (true, true) => {
                    return Ok(()).into();
                }
                (false, _) => {
                    ready!(self.s.poll_ready(cx))?;
                    self.s_ready = true;
                }
                (_, false) => {
                    ready!(self.f.poll_ready(cx))?;
                    self.f_ready = true;
                }
            }
        }
    }

    //  send known paths to Yew to be SSR'd, otherwise fall-back to `f`
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        //  TODO: think about how this treats not_found_path
        match <R as Routable>::recognize(req.uri().path()).is_some() {
            true => {
                self.s_ready = false;
                let fut = self.s.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.into_response())
                })
            }
            false => {
                self.f_ready = false;
                let fut = self.f.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.into_response())
                })
            }
        }
    }
}

fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}
