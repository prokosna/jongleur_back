use std::fs::File;
use std::io::Read;

use util::DomainConfig;

lazy_static! {
  static ref PRIVATE_KEY: Vec<u8> = {
    let filename = DomainConfig::jwt_private_key();
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf).unwrap();
    buf
  };

  static ref PUBLIC_KEY: Vec<u8> = {
    let filename = DomainConfig::jwt_public_key();
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf).unwrap();
    buf
  };

  static ref PUBLIC_KEY_PEM: String = {
    let filename = DomainConfig::jwt_public_key_pem();
    let mut file = File::open(filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
  };
}

pub struct KeyStore {}

impl KeyStore {
    pub fn jwt_private_key<'a>() -> &'a Vec<u8> {
        &PRIVATE_KEY
    }
    pub fn jwt_public_key<'a>() -> &'a Vec<u8> {
        &PUBLIC_KEY
    }
    pub fn jwt_public_key_pem<'a>() -> &'a String {
        &PUBLIC_KEY_PEM
    }
}
