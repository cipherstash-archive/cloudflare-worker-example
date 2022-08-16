use std::convert::TryInto;
use cipherstash_client::collection;
use cipherstash_client::indexer::CollectionSchema;
use cipherstash_client::api;
use cipherstash_client::indexer::mapping::{Mapping, MappingWithMeta};
use uuid::Uuid;
use worker::*;
// FIXME: The module probably could be named something other than decoder (maybe field or types?)
use cipherstash_client::record::decoder::{DataType, RecordSchema, SchemaField};


mod utils;
mod errors;
mod movie;
use self::movie::Movie;
use self::utils::*;
use self::errors::RequestError;


fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

fn get_record_id<D>(ctx: &RouteContext<D>) -> std::result::Result<Uuid, RequestError> {
    if let Some(input_str) = ctx.param("id") {
        return Ok(Uuid::parse_str(input_str).map_err(|_| RequestError::InvalidID)?);
    }
    return Err(RequestError::InvalidID);
}

fn collection_schema() -> CollectionSchema {
    CollectionSchema {
        schema: RecordSchema {
            map: collection! {
                "title" => SchemaField::DataType(DataType::String),
                "runningTime" => SchemaField::DataType(DataType::Uint64)
            },
        },
        indexes: collection! {
            "exactTitle" => MappingWithMeta {
                mapping: Mapping::Exact { field: "title".into() },
                index_id: *Uuid::parse_str("65e381f5-18dc-41af-8a89-d0641b09accc").unwrap().as_bytes(),
                prf_key: [0;16],
                prp_key: [0;16]
            },
            "runningTime" => MappingWithMeta {
                mapping: Mapping::Range { field: "runningTime".into() },
                index_id: *Uuid::parse_str("f83f2cf4-4fc9-4147-9c39-7b99eafa74df").unwrap().as_bytes(),
                prf_key: [0;16],
                prp_key: [0;16]
            }
        },
    }
}

fn get_token(req: &Request) -> std::result::Result<String, RequestError> {
    if let Some(auth) = req
        .headers()
        .get("authorization")
        .map_err(|_| RequestError::Unauthorized)?
    {
        return Ok(auth);
    }

    return Err(RequestError::Unauthorized);
}


#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // TODO: Set data on router
    // TODO: Unwraps and router errors

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .post_async("/", |mut req, ctx| async move {
            let schema = collection_schema();
            let jwt = get_token(&req).unwrap();
            let config = load_api_config(&ctx)?;
            let mut api = api::Api::connect(config, schema, jwt);

            let movie: Movie = req.json().await?;
            let record = movie.into();

            let created = api
                .put(record)
                .await
                .unwrap();
                //.map_err(|e| Response::error(e.to_string(), 400))?;

            return Response::from_json(&created);
        })
        .get_async("/:id", |req, ctx| async move {
            let schema = collection_schema();
            let jwt = get_token(&req).unwrap();
            let config = load_api_config(&ctx)?;
            let mut api = api::Api::connect(config, schema, jwt);

            return match get_record_id(&ctx) {
                Ok(result) => {
                    let movie: Movie =
                        api
                        .get(result)
                        .await
                        .unwrap()
                        .try_into()
                        .unwrap();

                    Response::from_json(&movie)
                }
                Err(e) => Response::error(format!("Error: {}", e.to_string()), 400),
            };
        })
        .run(req, env)
        .await
}
