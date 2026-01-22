use ctes_models::{
    Ciphertext,
    CiphertextMetadata,
    EncodingMetadata,
    Encoding
};

fn main() {
    let enc_metadata: EncodingMetadata = EncodingMetadata{
        encoding: Encoding::Utf8 as i32,
        base: 0,
    };
    
    let ct_metadata: CiphertextMetadata = CiphertextMetadata{
        encoding: Some(enc_metadata),
        r#type: "text".to_string()
    };

    let ciphertext: Ciphertext = Ciphertext{
        bytes: "Hello, world!".as_bytes().to_vec(),
        metadata: Some(ct_metadata)
    };

    println!("Ciphertext: {:?}", ciphertext);
}
