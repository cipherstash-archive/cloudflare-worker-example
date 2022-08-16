use cfg_if::cfg_if;
use cipherstash_client::api::{Config, SourceKey};
use hex::FromHex;
use uuid::Uuid;
use worker::{console_log, Date, Env, Request};

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

pub(crate) fn load_config<D>(env: &Env) -> Result<Config, String> {
    let collection_id = Uuid::parse_str(
        &env.var("COLLECTION_ID")
            .map_err(|_| "Missing Collection ID".to_string())?
            .to_string(),
    )
    .map_err(|_| "Missing Collection ID".to_string())?;

    let host = &env
        .var("CIPHERSTASH_HOST")
        .map_err(|_| "Missing Host".to_string())?
        .to_string();

    let key_string = &env
        .secret("CIPHERSTASH_KEY")
        .map_err(|_| "Missing Key".to_string())?
        .to_string();

    let key = SourceKey::from_hex(key_string).map_err(|_| "Invalid source key".to_string())?;

    Ok(Config::init(host.to_string(), collection_id, key))
}

pub(crate) fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}
