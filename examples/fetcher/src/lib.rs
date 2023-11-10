mod error;

use error::CustomError;
use worker::{event, Context, Env, Fetch, Request, Response, Result, RouteContext, Router, Url};

const ENDPOINT: &str = "https://pokeapi.co/api/v2/pokemon";

async fn fetch_pokemon(url_string: &str) -> std::result::Result<Response, CustomError> {
    // construct a new Url
    match Url::parse(url_string) {
        Ok(url) => {
            Ok(Fetch::Url(url).send().await?)

            // to return a Response with new response headers, do the following:
            // let mut resp = Fetch::Url(url).send().await?;
            // let res = resp.json::<serde_json::Value>().await?;
            // Ok(Response::from_json(&res)?)
        }
        Err(err) => Err(CustomError::new(err.to_string(), 500)),
    }
}

async fn handle_single(_: Request, ctx: RouteContext<()>) -> Result<Response> {
    Ok(fetch_pokemon(&format!("{ENDPOINT}/{}", ctx.param("name").unwrap())).await?)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/pokemon/:name", handle_single)
        .run(req, env)
        .await
}
