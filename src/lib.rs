use hex::FromHex;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use rain_metadata::meta::{
                             RainMetaDocumentV1Item, ContentType, ContentEncoding, ContentLanguage,
                             KnownMagic
                         };
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub fn concat_hex_addresses(addresses: Array) -> Vec<u8> {
    let mut concatenated = Vec::new();
    for idx in 0..addresses.length() {
        let address = addresses.get(idx);
        if let Some(js_address) = address.as_string() {
            // Remove the "0x" prefix and convert the address from hex to bytes
            if let Ok(hex_bytes) = Vec::from_hex(&js_address[2..]) {
                concatenated.extend(hex_bytes);
            }
        }
    }
    concatenated
}

#[wasm_bindgen]
pub fn hex_to_bytes(hex_val: &str) -> Option<Vec<u8>> {
    // Remove the '0x' prefix if present
    let cleaned_hex = if hex_val.starts_with("0x") {
        &hex_val[2..]
    } else {
        hex_val
    };

    // Attempt to parse the hex string into bytes
    match hex::decode(cleaned_hex) {
        Ok(bytes) => Some(bytes),
        Err(_) => None,
    }
}

#[wasm_bindgen]
pub async fn cbor_encode_addresses(address: Vec<u8> ) -> JsValue {
    // Concatenate the addresses using concat_hex_addresses function
//     let concatenated_addresses = concat_hex_addresses(addresses);

    let meta_map = RainMetaDocumentV1Item {
        payload: serde_bytes::ByteBuf::from(address),
        magic: KnownMagic::AddressList,
        content_type: ContentType::Cbor,
        content_encoding: ContentEncoding::None,
        content_language: ContentLanguage::None,
    };

    match meta_map.cbor_encode() {
        Ok(cbor_encoded) => {
            if let Ok(value) = to_value(&cbor_encoded) {
                value
            } else {
                JsValue::from_str("Error converting to JsValue")
            }
        }
        Err(err) => JsValue::from_str(&format!("CBOR encoding error: {:?}", err)),
    }
}