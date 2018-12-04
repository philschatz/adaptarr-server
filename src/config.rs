use rand::RngCore;
use std::{fs, net::{SocketAddr, Ipv4Addr}};
use toml;
use serde::de::{Deserializer, Error, Visitor, Unexpected};

pub fn load() -> crate::Result<Config> {
    let data = fs::read("config.toml")?;
    toml::from_slice(&data).map_err(|e| ConfigurationError(e).into())
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub database: Option<Database>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    /// Address on which to listen.
    #[serde(default = "default_address")]
    pub address: SocketAddr,
    /// Domain (host name) of this server.
    pub domain: String,
    /// Secret key.
    #[serde(default = "random_secret", deserialize_with = "de_binary_base64")]
    pub secret: Vec<u8>,
}

/// Database configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Fail)]
#[fail(display = "Invalid configuration: {}", _0)]
pub struct ConfigurationError(#[fail(cause)] toml::de::Error);

/// Default address (127.0.0.1:80).
fn default_address() -> SocketAddr {
    (Ipv4Addr::LOCALHOST, 80).into()
}

/// Default secret (32 random bytes).
fn random_secret() -> Vec<u8> {
    let mut secret = vec![0; 32];
    rand::thread_rng().fill_bytes(&mut secret);
    secret
}

/// Deserialize a vector of bytes from either binary data or a base64-encoded
/// string.
fn de_binary_base64<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(BinaryBase64Visitor)
}

struct BinaryBase64Visitor;

impl<'de> Visitor<'de> for BinaryBase64Visitor {
    type Value = Vec<u8>;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a binary or a base64-encoded string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Vec<u8>, E>
    where
        E: Error,
    {
        base64::decode(v)
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &"a base64 string"))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Vec<u8>, E> {
        Ok(v.into())
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Vec<u8>, E> {
        Ok(v)
    }
}
