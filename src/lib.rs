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
    // Your logic here
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
pub async fn cbor_encode_addresses(addresses: Vec<u8>) -> JsValue {
    let meta_map = RainMetaDocumentV1Item {
        payload: serde_bytes::ByteBuf::from(addresses),
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
        },
        Err(err) => JsValue::from_str(&format!("CBOR encoding error: {:?}", err)),
    }
}