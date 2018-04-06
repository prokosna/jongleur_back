use std::fs::File;
use std::io::Read;

use config::AppConfig;

lazy_static! {
    static ref PRIVATE_KEY: Vec<u8> = {
        let filename = AppConfig::jwt_private_key();
        let mut file = File::open(filename).unwrap();
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf).unwrap();
        buf
    };
    static ref PUBLIC_KEY: Vec<u8> = {
        let filename = AppConfig::jwt_public_key();
        let mut file = File::open(filename).unwrap();
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf).unwrap();
        buf
    };
    static ref PUBLIC_KEY_PEM: String = {
        let filename = AppConfig::jwt_public_key_pem();
        let mut file = File::open(filename).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    };
}

pub trait KeyService {
    fn jwt_private_key<'a>(&self) -> &'a Vec<u8> {
        &PRIVATE_KEY
    }
    fn jwt_public_key<'a>(&self) -> &'a Vec<u8> {
        &PUBLIC_KEY
    }
    fn jwt_public_key_pem<'a>(&self) -> &'a String {
        &PUBLIC_KEY_PEM
    }
}

pub trait KeyServiceComponent {
    type KeyService: KeyService;
    fn key_service(&self) -> &Self::KeyService;
}

// Implement
// TODO: Implement in the infra module
impl<T> KeyService for T {}
