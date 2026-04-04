use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyType {
    // Standard Base Keys
    SSH_DSS,
    SSH_RSA,
    SSH_ED448,
    SSH_ED25519,
    RSA_SHA2_256,
    RSA_SHA2_512,
    ECDSA_SHA2_NISTP256,
    ECDSA_SHA2_NISTP384,
    ECDSA_SHA2_NISTP521,

    // Hardware Security Keys (SK)
    SK_SSH_ED25519,
    SK_ECDSA_SHA2_NISTP256,

    // OpenSSH Certificates
    SSH_DSS_CERT,
    SSH_RSA_CERT,
    SSH_ED25519_CERT,
    RSA_SHA2_256_CERT,
    RSA_SHA2_512_CERT,
    SK_SSH_ED25519_CERT,
    ECDSA_SHA2_NISTP256_CERT,
    ECDSA_SHA2_NISTP384_CERT,
    ECDSA_SHA2_NISTP521_CERT,
    SK_ECDSA_SHA2_NISTP256_CERT,

    // X.509 and PGP
    PGP_SIGN_RSA,
    PGP_SIGN_DSS,
    X509V3_SSH_RSA,
    X509V3_SSH_DSS,
    X509V3_SIGN_RSA,
    X509V3_SIGN_DSS,
    X509V3_RSA2048_SHA256,
    X509V3_ECDSA_SHA2_NISTP256,
    X509V3_ECDSA_SHA2_NISTP384,
    X509V3_ECDSA_SHA2_NISTP521,

    // Emerging Post-Quantum (ML-DSA)
    SSH_MLDSA_44,
    SSH_MLDSA_65,
    SSH_MLDSA_87,

    // Fallback for unhandled types
    Unknown(String),
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            // Standard Base Keys
            Self::SSH_DSS => "ssh-dss",
            Self::SSH_RSA => "ssh-rsa",
            Self::SSH_ED448 => "ssh-ed448",
            Self::SSH_ED25519 => "ssh-ed25519",
            Self::RSA_SHA2_256 => "rsa-sha2-256",
            Self::RSA_SHA2_512 => "rsa-sha2-512",
            Self::ECDSA_SHA2_NISTP256 => "ecdsa-sha2-nistp256",
            Self::ECDSA_SHA2_NISTP384 => "ecdsa-sha2-nistp384",
            Self::ECDSA_SHA2_NISTP521 => "ecdsa-sha2-nistp521",

            // Hardware Security Keys (SK)
            Self::SK_SSH_ED25519 => "sk-ssh-ed25519@openssh.com",
            Self::SK_ECDSA_SHA2_NISTP256 => "sk-ecdsa-sha2-nistp256@openssh.com",

            // OpenSSH Certificates
            Self::SSH_DSS_CERT => "ssh-dss-cert-v01@openssh.com",
            Self::SSH_RSA_CERT => "ssh-rsa-cert-v01@openssh.com",
            Self::SSH_ED25519_CERT => "ssh-ed25519-cert-v01@openssh.com",
            Self::RSA_SHA2_256_CERT => "rsa-sha2-256-cert-v01@openssh.com",
            Self::RSA_SHA2_512_CERT => "rsa-sha2-512-cert-v01@openssh.com",
            Self::SK_SSH_ED25519_CERT => "sk-ssh-ed25519-cert-v01@openssh.com",
            Self::ECDSA_SHA2_NISTP256_CERT => "ecdsa-sha2-nistp256-cert-v01@openssh.com",
            Self::ECDSA_SHA2_NISTP384_CERT => "ecdsa-sha2-nistp384-cert-v01@openssh.com",
            Self::ECDSA_SHA2_NISTP521_CERT => "ecdsa-sha2-nistp521-cert-v01@openssh.com",
            Self::SK_ECDSA_SHA2_NISTP256_CERT => "sk-ecdsa-sha2-nistp256-cert-v01@openssh.com",

            // X.509 and PGP
            Self::PGP_SIGN_RSA => "pgp-sign-rsa",
            Self::PGP_SIGN_DSS => "pgp-sign-dss",
            Self::X509V3_SSH_RSA => "x509v3-ssh-rsa",
            Self::X509V3_SSH_DSS => "x509v3-ssh-dss",
            Self::X509V3_SIGN_RSA => "x509v3-sign-rsa",
            Self::X509V3_SIGN_DSS => "x509v3-sign-dss",
            Self::X509V3_RSA2048_SHA256 => "x509v3-rsa2048-sha256",
            Self::X509V3_ECDSA_SHA2_NISTP256 => "x509v3-ecdsa-sha2-nistp256",
            Self::X509V3_ECDSA_SHA2_NISTP384 => "x509v3-ecdsa-sha2-nistp384",
            Self::X509V3_ECDSA_SHA2_NISTP521 => "x509v3-ecdsa-sha2-nistp521",

            // Emerging Post-Quantum
            Self::SSH_MLDSA_44 => "ssh-mldsa-44",
            Self::SSH_MLDSA_65 => "ssh-mldsa-65",
            Self::SSH_MLDSA_87 => "ssh-mldsa-87",

            // Fallback
            Self::Unknown(unknown_str) => unknown_str.as_str(),
        };
        write!(f, "{}", s)
    }
}

pub struct Key {
    pub length: u32,
    pub keytype: KeyType,
    pub key: String,
    pub comment: String,
}

impl Key {
    pub fn from(buf: Vec<u8>) -> Option<Self> {
        let string_data = String::from_utf8_lossy(&buf).to_string();

        // Split the string into parts. SSH keys typically follow: [type] [base64_data] [optional_comment]
        let mut parts = string_data.split_whitespace();

        let type_str = parts.next()?;
        let key_data = parts.next()?.to_string();

        // Collect any remaining parts as the comment (could contain spaces)
        let comment_data = parts.collect::<Vec<&str>>().join(" ");

        let keytype = match type_str {
            // Standard Base Keys
            "ssh-dss" => KeyType::SSH_DSS,
            "ssh-rsa" => KeyType::SSH_RSA,
            "ssh-ed448" => KeyType::SSH_ED448,
            "ssh-ed25519" => KeyType::SSH_ED25519,
            "rsa-sha2-256" => KeyType::RSA_SHA2_256,
            "rsa-sha2-512" => KeyType::RSA_SHA2_512,
            "ecdsa-sha2-nistp256" => KeyType::ECDSA_SHA2_NISTP256,
            "ecdsa-sha2-nistp384" => KeyType::ECDSA_SHA2_NISTP384,
            "ecdsa-sha2-nistp521" => KeyType::ECDSA_SHA2_NISTP521,

            // Hardware Security Keys (SK)
            "sk-ssh-ed25519@openssh.com" => KeyType::SK_SSH_ED25519,
            "sk-ecdsa-sha2-nistp256@openssh.com" => KeyType::SK_ECDSA_SHA2_NISTP256,

            // OpenSSH Certificates
            "ssh-dss-cert-v01@openssh.com" => KeyType::SSH_DSS_CERT,
            "ssh-rsa-cert-v01@openssh.com" => KeyType::SSH_RSA_CERT,
            "ssh-ed25519-cert-v01@openssh.com" => KeyType::SSH_ED25519_CERT,
            "rsa-sha2-256-cert-v01@openssh.com" => KeyType::RSA_SHA2_256_CERT,
            "rsa-sha2-512-cert-v01@openssh.com" => KeyType::RSA_SHA2_512_CERT,
            "sk-ssh-ed25519-cert-v01@openssh.com" => KeyType::SK_SSH_ED25519_CERT,
            "ecdsa-sha2-nistp256-cert-v01@openssh.com" => KeyType::ECDSA_SHA2_NISTP256_CERT,
            "ecdsa-sha2-nistp384-cert-v01@openssh.com" => KeyType::ECDSA_SHA2_NISTP384_CERT,
            "ecdsa-sha2-nistp521-cert-v01@openssh.com" => KeyType::ECDSA_SHA2_NISTP521_CERT,
            "sk-ecdsa-sha2-nistp256-cert-v01@openssh.com" => KeyType::SK_ECDSA_SHA2_NISTP256_CERT,

            // X.509 and PGP
            "pgp-sign-rsa" => KeyType::PGP_SIGN_RSA,
            "pgp-sign-dss" => KeyType::PGP_SIGN_DSS,
            "x509v3-ssh-rsa" => KeyType::X509V3_SSH_RSA,
            "x509v3-ssh-dss" => KeyType::X509V3_SSH_DSS,
            "x509v3-sign-rsa" => KeyType::X509V3_SIGN_RSA,
            "x509v3-sign-dss" => KeyType::X509V3_SIGN_DSS,
            "x509v3-rsa2048-sha256" => KeyType::X509V3_RSA2048_SHA256,
            "x509v3-ecdsa-sha2-nistp256" => KeyType::X509V3_ECDSA_SHA2_NISTP256,
            "x509v3-ecdsa-sha2-nistp384" => KeyType::X509V3_ECDSA_SHA2_NISTP384,
            "x509v3-ecdsa-sha2-nistp521" => KeyType::X509V3_ECDSA_SHA2_NISTP521,

            // Emerging Post-Quantum
            "ssh-mldsa-44" => KeyType::SSH_MLDSA_44,
            "ssh-mldsa-65" => KeyType::SSH_MLDSA_65,
            "ssh-mldsa-87" => KeyType::SSH_MLDSA_87,

            // Catch-all
            _ => KeyType::Unknown(type_str.to_string()),
        };

        let length = &keytype.to_string().len() + key_data.len() + comment_data.len();

        Some(Self {
            length: length as u32,
            keytype,
            key: key_data,
            comment: comment_data,
        })
    }

    pub fn to_ssh_str(&self) -> Vec<u8> {
        let mut str: Vec<u8> = Vec::new();
        str.extend(self.length.to_be_bytes());
        str.extend(self.keytype.to_string().as_bytes());
        str.extend(self.key.as_bytes());
        str.extend(self.comment.as_bytes());
        str
    }
}
