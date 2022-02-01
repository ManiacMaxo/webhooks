use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(
    payload: &[u8],
    value: &str,
    algorithm: &str,
    secret: Option<&str>,
) -> bool {
    match algorithm {
        "hmacSha256" => {
            if secret.is_none() {
                return false;
            }
            let mut mac = HmacSha256::new_from_slice(secret.unwrap().as_bytes()).unwrap();

            mac.update(payload);
            let result = mac.finalize();
            let code_bytes = result.into_bytes();

            let mut mac = HmacSha256::new_from_slice(secret.unwrap().as_bytes()).unwrap();

            mac.update(value.as_bytes());
            println!("{:?}", mac.verify(&code_bytes).unwrap());
            return true;
        }
        _ => return false,
    }
}
