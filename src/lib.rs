use cipherstash_client::api::{self, Config};
use std::convert::TryInto;
use uuid::Uuid;
use worker::*;

mod movie;
mod schema;
mod utils;
use self::movie::Movie;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    let config = utils::load_config::<Config>(&env);
    if let Err(err) = config {
        return Response::error(err.to_string(), 500);
    }

    let auth = req.headers().get("authorization")?;
    if auth.is_none() {
        return Response::error("Not Authorized", 401);
    }

    Router::with_data((config.unwrap(), auth.unwrap().to_string()))
        .post_async("/", |mut req, ctx| async move {
            let schema = schema::collection_schema();
            let (config, authn) = ctx.data;
            let mut api = api::Api::connect(config, schema, authn);

            let movie: Movie = req.json().await?;
            let record = movie.into();

            match api.put(record).await {
                Ok(result) => Response::from_json(&result),
                Err(err) => Response::error(err.to_string(), 400),
            }
        })
        .get_async("/:id", |_req, ctx| async move {
            let schema = schema::collection_schema();
            if let Some(id) = &ctx.param("id").and_then(|str| Uuid::parse_str(str).ok()) {
                let (config, authn) = ctx.data;
                let mut api = api::Api::connect(config, schema, authn);

                return match api.get(*id).await {
                    Ok(record) => Response::from_json::<Movie>(&record.try_into()?),
                    // TODO: Handle more errors
                    Err(err) => Response::error(err.to_string(), 500),
                };
            } else {
                return Response::error("Missing or invalid ID parameter", 400);
            }
        })
        .run(req, env)
        .await
}
