use hex::{encode, ToHex};
use ring::{
    rand,
    signature::{self, KeyPair, Signature}, aead::Tag,
};
use sha2::{Digest, Sha256, Sha512};

pub fn hash_str(s: &str) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(&s);
    let result = hasher.finalize();
    return result.to_vec();
}

pub fn round(x: f64) -> f64 {
    (x * 100.0).floor() / 100.0 //rounds to down to 2 decimal places, TODO: Limit transactions below 0.01 NIX for safety!
}

pub fn keys() {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
    let peer_public_key_bytes = key_pair.public_key().as_ref();
    println!("Pkcs8: {:x?}", encode(pkcs8_bytes.as_ref()));
    println!("Public key: {:x?}", encode(&peer_public_key_bytes));
    println!("PB KEY {:?}", peer_public_key_bytes);
    let msg = b"hello world!";
    let fmsg = b"not the hello world!";
    let sig= key_pair.sign(msg);
    println!("\nSignature: {:?}", &sig.as_ref());
    let peer_public_key =
        signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
    let vef = peer_public_key.verify(fmsg, sig.as_ref()).is_ok();
    println!("{:x?}", vef)
}

pub fn bytes_to_ascii(byt: Vec<u8>) -> String {
    let data = std::str::from_utf8(&byt).expect("invalid utf-8 sequence");
    data.to_owned()
}

pub fn verify(pub_key:&[u8], sig:&[u8], msg:&str) {
    let peer_public_key_bytes = pub_key.as_ref();
    let peer_public_key =
        signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
    let vef = peer_public_key.verify(&msg.as_bytes(), &sig).is_ok();
    println!("{:?}",vef);
}
