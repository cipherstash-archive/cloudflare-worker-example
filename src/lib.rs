
use cipherstash_client::record::Record;
use worker::*;
use ore_rs::{
    ORECipher,  // Main ORE Cipher trait
    OREEncrypt, // Traits for encrypting primitive types (e.g. u64)
    scheme::bit2::OREAES128 // Specific scheme we want to use
};
use hex_literal::hex;
use cipherstash_client::collection;
use cipherstash_client::indexer::*;
use cipherstash_client::indexer::mapping::{Mapping, MappingWithMeta};
// FIXME: The module probably could be named something other than decoder (maybe field or types?)
use cipherstash_client::record::decoder::{
    RecordSchema, DataType, SchemaField
};
mod utils;

struct EncryptError {
    msg: String
}

fn err(msg: &str) -> EncryptError {
    EncryptError { msg: msg.to_string() }
}

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

fn get_input<D>(ctx: &RouteContext<D>) -> std::result::Result<u64, EncryptError> {
    if let Some(input_str) = ctx.param("input") {
        let result: u64 = input_str
            .parse()
            .map_err(|_| err("Invalid plaintext"))?;

        return Ok(result);
    }

    Err(err("No input provided"))
}

fn do_encrypt_get<D>(ctx: &RouteContext<D>) -> std::result::Result<String, EncryptError> {
    let k1: [u8; 16] = hex!("00010203 04050607 08090a0b 0c0d0e0f");
    let k2: [u8; 16] = hex!("00010203 04050607 08090a0b 0c0d0e0f");
    let seed = hex!("00010203 04050607");

    let input = get_input(&ctx)?;
    let ore = OREAES128::init(k1, k2, &seed).map_err(|_| EncryptError { msg: "Cipher Init Failed".to_string() })?;

    let result = input.encrypt(&ore)
        .map_err(|_| err("Encryption Failed"))?
        .to_bytes();

    Ok(hex::encode(result))
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
                index_id: [0;16],
                prf_key: [0;16],
                prp_key: [0;16]
            },
            "runningTime" => MappingWithMeta {
                mapping: Mapping::Range { field: "runningTime".into() },
                index_id: [0;16],
                prf_key: [0;16],
                prp_key: [0;16]
            }
        },
    }
}

fn index_record() {
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
        .get("/:input", |_, ctx| {

            index_record();

            match do_encrypt_get(&ctx) {
                Ok(result) => return Response::ok(format!("OK: {}", result)),
                Err(EncryptError { msg }) => Response::error(msg, 400)
            }
        })
        .run(req, env)
        .await
}
