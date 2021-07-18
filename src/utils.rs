use sha2::Sha256;

pub fn verify_signature(payload: String, value: String, algorithm: &str ="sha256") -> bool {
    let mut hasher

    let r = match algorithm {
        "sha256" => {
            hasher = Sha256::new().update(value.as_bytes());
            let result = hasher.finish();

            return result == payload.as_bytes();
        },
        _ => false;
    }

    return r;
}
