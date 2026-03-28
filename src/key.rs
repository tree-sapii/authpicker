#[allow(non_camel_case_types)]
enum KeyType {
    ECDSA,
    ECDSA_SK,
    ED25519,
    ED25519_SK,
    RSA,
}

struct Key {
    keytype: KeyType,
    key: String,
    comment: String,
}

impl Key {
    pub fn from(buf: Vec<u8>) { // TODO: Change the param name to something better in the future
    }
}
