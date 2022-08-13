
use cipherstash_client::collection;
use cipherstash_client::grpc::tonic::codegen::http::request;
use serde_json::to_string;
use thiserror::Error;
use cipherstash_client::grpc::client::Client;
use cipherstash_client::grpc::tonic::metadata::AsciiMetadataKey;
use cipherstash_client::grpc::tonic::metadata::MetadataValue;
use cipherstash_client::record::Record;

use uuid::Uuid;
use worker::*;
use ore_rs::{
    ORECipher,  // Main ORE Cipher trait
    OREEncrypt, // Traits for encrypting primitive types (e.g. u64)
    scheme::bit2::OREAES128 // Specific scheme we want to use
};
use hex_literal::hex;
use cipherstash_client::api;
use cipherstash_client::indexer::*;
use cipherstash_client::indexer::mapping::{Mapping, MappingWithMeta};
// FIXME: The module probably could be named something other than decoder (maybe field or types?)
use cipherstash_client::record::decoder::{
    RecordSchema, DataType, SchemaField
};

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

#[derive(Debug, Error)]
enum RequestError {
    #[error("ID missing or invalid")]
    InvalidID,
    #[error("No such collection")]
    NoSuchCollection,
    #[error("No such record")]
    NoSuchRecord,
    #[error("Unauthorized")]
    Unauthorized
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

/*fn index_record() {
    let schema = collection_schema();
    let mut indexer = RecordIndexer::from_collection_schema(schema).unwrap();

    let vectors = indexer
    .encrypt(&Record {
        id: [163,  98, 105, 100, 216, 64,
            80, 117, 189,  53, 179, 82,
            84,  65,   2, 178],
        fields: collection! {
            "title" => "Hello!",
            "runningTime" => 230
        },
    })
    .unwrap();

    console_debug!("VECTORS: {:?}", vectors);
}*/

/*async fn send_request(id: Uuid) -> Result<User> {

    let user = client.get(id)?;

    let client = Client::new("https://ap-southeast-2.aws.stashdata.net".to_string());

    let mut request = tonic::Request::new(api::documents::GetRequest{
        collection_id: [
            31, 234, 109, 160, 130,
           107,  65, 203, 166, 255,
            58,  19, 217, 161, 246,
           229
         ].into(),
        id: [
            97, 244, 195,   5, 19, 153,
            71, 186, 133,  15, 88, 138,
           129,  64, 251, 179
         ].into()
    });


    let jwt = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IkEwOVdodlFoV3h1emRjS2ZwVHFUdSJ9.eyJodHRwczovL2F3cy5hbWF6b24uY29tL3RhZ3MiOnsicHJpbmNpcGFsX3RhZ3MiOnsid29ya3NwYWNlIjpbIndzOkNaUlFKWk5CV0xTQzNaSSJdfX0sImlzcyI6Imh0dHBzOi8vYXV0aC5jaXBoZXJzdGFzaC5jb20vIiwic3ViIjoiYXV0aDB8NjE1MjZhMDRhODllOGIwMDY4ZTQ2ODQ2IiwiYXVkIjoiYXAtc291dGhlYXN0LTIuYXdzLnN0YXNoZGF0YS5uZXQiLCJpYXQiOjE2NjAwMzA5MTIsImV4cCI6MTY2MDExNzMxMiwiYXpwIjoiQ3RZOUROR29uZ29TdlphQXdiYjZzdzBIcjdHbDdwZzciLCJzY29wZSI6ImNvbGxlY3Rpb24uY3JlYXRlIGNvbGxlY3Rpb24uZGVsZXRlIGNvbGxlY3Rpb24uaW5mbyBjb2xsZWN0aW9uLmxpc3QgZG9jdW1lbnQucHV0IGRvY3VtZW50LmRlbGV0ZSBkb2N1bWVudC5nZXQgZG9jdW1lbnQucXVlcnkgd3M6Q1pSUUpaTkJXTFNDM1pJIG9mZmxpbmVfYWNjZXNzIn0.vNrMkGB19Wg5_nfqDPmX88JbdBbWumZb7MFhed3Jg88I2wiIqSBmZqdCfIKulRvFqXnVtjkF8O8VzjcylZGXPtHCuVsDwYyq0JUQ0bC5Kc0lXvIMSJ17ipTXuCG5jIynyb0GL4Jcr9HhAeReNL6D7OKHBxkCKEfuvISGd4oauMUOt8GQ9XyfpEHgVpEx1R3hrMrvHjqla9zjKpx5n-LZknrF7BBzOi4sK1x7bDJT4_kKOZejS09cfoX2WZ5onf_eEKiPIcKLfPwrMBWvk-SFyWcV4wS3tLvE6yYfc2WCGADtNJmCJeSdy15-FP4uyaxzcFfW4MOtgptWOPOkgzm5lQ";
    request
        .metadata_mut()
        .insert(AsciiMetadataKey::from_static("authorization"), format!("Bearer {}", jwt).parse().unwrap());

    let mut stash =  api::api_client::ApiClient::new(client);

    //client.get(request).await.unwrap();
    match stash.get(request).await {
        Ok(ret) => console_debug!("RESPONSE OK: {:?}", ret.into_inner()),
        Err(e) => console_error!("ERROR: {:?}", e)
    }
}*/

fn get_token(req: &Request) -> std::result::Result<String, RequestError> {

    if let Some(auth) = req
        .headers()
        .get("authorization")
        .map_err(|_| RequestError::Unauthorized)? {
            return Ok(auth);
        }
    
    return Err(RequestError::Unauthorized);
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

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
        .post_async("/", |req, ctx| async move {
            let schema = collection_schema();
            let jwt = get_token(&req).unwrap();

            let collection_id = Uuid::parse_str(
                &ctx.env
                    .var("COLLECTION_ID")?
                    .to_string()
                ).map_err(|_| Error::Internal("Bad or missing collection ID".into()))?;
        
            console_debug!("Using Collection ID {:?}", collection_id);

            let api = api::Api::connect(
                schema,
                collection_id,
                "https://ap-southeast-2.aws.stashdata.net".to_string(),
                jwt.to_string()
            );

            let vectors = api.put(Record {
                id: [163,  98, 105, 100, 216, 64,
                    80, 117, 189,  53, 179, 82,
                    84,  65,   2, 178],
                fields: collection! {
                    "title" => "Hello!",
                    "runningTime" => 230
                }
            }).await.unwrap();

            console_debug!("RETURN: {:?}", vectors);

            return Response::ok("SAVED");
        })
        .get_async("/:id", |req, ctx| async move {
            let schema = collection_schema();
            let jwt = get_token(&req).unwrap();

            let collection_id = Uuid::parse_str(
                &ctx
                    .env
                    .var("COLLECTION_ID")?
                    .to_string()
                ).map_err(|_| Error::Internal("Bad or missing collection ID".into()))?;
        
            console_debug!("Using Collection ID {:?}", collection_id);

            // TODO: Don't unwrap
            let api = api::Api::connect(
                schema,
                collection_id,
                "https://ap-southeast-2.aws.stashdata.net".to_string(),
                jwt.to_string()
            );

            //send_request().await;
            return match get_record_id(&ctx) {
                Ok(result) => {
                    let raw = api.get(*result.as_bytes()).await.unwrap();
                    Response::ok(format!("OK: {:?}", raw))
                },
                Err(e) => Response::error(format!("Error: {}", e), 400)
            };
        })
        .run(req, env)
        .await
}
