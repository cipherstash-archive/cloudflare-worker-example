use cipherstash_client::api::{Config, SourceKey};
use hex::FromHex;
use uuid::Uuid;
use worker::RouteContext;
use cfg_if::cfg_if;
use crate::RequestError;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub(crate) fn load_api_config<D>(ctx: &RouteContext<D>) -> Result<Config, RequestError> {
    let collection_id = Uuid::parse_str(
        &ctx.env
            .var("COLLECTION_ID")
            .map_err(|_| RequestError::MissingCollectionId)?
            .to_string(),
    )
    .map_err(|_| RequestError::MissingCollectionId)?;

    let host = &ctx
        .env
        .var("CIPHERSTASH_HOST")
        .map_err(|_| RequestError::ConfigError("Missing Host".into()))?
        .to_string();

    let key_string = &ctx
        .env
        .secret("CIPHERSTASH_KEY")
        .map_err(|_| RequestError::ConfigError("Missing Key".into()))?
        .to_string();

    let key = SourceKey::from_hex(key_string)
        .map_err(|_| RequestError::ConfigError("Invalid source key".to_string()))?;

    Ok(Config::init(host.to_string(), collection_id, key))
}


