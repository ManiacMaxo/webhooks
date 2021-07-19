use sha2::Sha256;

pub fn verify_signature(payload: String, value: String, algorithm: &str = "sha256") -> bool {
    let mut hasher = match algorithm {
        "sha256" => Sha256::new(),
    };

    return false;
}
