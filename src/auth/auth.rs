// auth document https://btsecom.github.io/docs/spot/en/#authentication
//
// API Key (request-api)
//
// Parameter Name: request-api, in: header. API key is obtained from BTSE platform as a string
// API Key (request-nonce)
//
// Parameter Name: request-nonce, in: header. Representation of current timestamp in long format
// API Key (request-sign)
//
// Parameter Name: request-sign, in: header. A composite signature produced based on the following algorithm: Signature=HMAC.Sha384 (secretkey, (urlpath + request-nonce + bodyStr)) (note: bodyStr = '' when no data):

use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha384;

#[derive(Debug)]
pub struct AuthSign {
    pub key: String,
    pub nonce: String,
    pub sign: String,
}

pub fn generate_auth(url_path: &str, api_key: &str, api_secret: &str, body: &str) -> AuthSign {
    let now = Utc::now();
    let timestamp_millis = now.timestamp_millis().to_string();

    let message = format!("{}{}{}", url_path, timestamp_millis, body);

    let mut mac = Hmac::<Sha384>::new_from_slice(api_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(message.as_bytes());
    let result = mac.finalize().into_bytes();

    let signature = general_purpose::STANDARD.encode(result);
    format!("Signature=HMAC.Sha384({})", signature);

    AuthSign {
        key: String::from(api_key),
        nonce: timestamp_millis,
        sign: signature,
    }
}
