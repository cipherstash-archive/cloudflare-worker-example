use cipherstash_client::{api, indexer::CollectionSchema};
use serde_json::json;
use std::convert::TryInto;
use uuid::Uuid;
use worker::*;

mod query;
mod user;
mod utils;

use query::UserQuery;
use serde::Deserialize;
use user::User;

#[derive(Deserialize)]
struct CollectionSchemaWithId {
    id: Uuid,
    #[serde(flatten)]
    schema: CollectionSchema,
}

async fn handler(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    let CollectionSchemaWithId { id, schema } =
        // Load User collection schema + keys
        serde_json::from_str(include_str!("../users.annotated.json"))?;

    let config = utils::load_config(id, &env);
    if let Err(err) = config {
        return Response::error(err.to_string(), 500);
    }

    let auth = req.headers().get("authorization")?;
    if auth.is_none() {
        return Response::error("Not Authorized", 401);
    }

    Router::with_data((config.unwrap(), auth.unwrap().to_string(), schema))
        .post_async("/", |mut req, ctx| async move {
            let (config, authn, schema) = ctx.data;
            let mut api = api::Api::connect(config, schema, authn);

            let user: User = req.json().await?;

            let record = user.into();

            match api.put(record).await {
                Ok(result) => Response::from_json(&result),
                Err(err) => Response::error(err.to_string(), 400),
            }
        })
        .post_async("/query", |mut req, ctx| async move {
            let (config, authn, schema) = ctx.data;
            let api = api::Api::connect(config, schema, authn);

            let query: UserQuery = req.json().await?;

            let res = api.query(query.into()).await.map_err(|e| e.to_string())?;

            let users: Vec<User> = res
                .records
                .into_iter()
                .map(|x| x.try_into())
                .collect::<std::result::Result<_, _>>()?;

            Response::from_json(&json!({ "users": users }))
        })
        .get_async("/:id", |_req, ctx| async move {
            if let Some(id) = &ctx.param("id").and_then(|str| Uuid::parse_str(str).ok()) {
                let (config, authn, schema) = ctx.data;
                let mut api = api::Api::connect(config, schema, authn);

                return match api.get(*id).await {
                    Ok(record) => Response::from_json::<User>(&record.try_into()?),
                    Err(err) => Response::error(err.to_string(), 500),
                };
            } else {
                return Response::error("Missing or invalid ID parameter", 400);
            }
        })
        .run(req, env)
        .await
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, ctx: worker::Context) -> Result<Response> {
    match handler(req, env, ctx).await {
        Ok(x) => Ok(x),
        Err(e) => Response::error(e.to_string(), 500),
    }
}
