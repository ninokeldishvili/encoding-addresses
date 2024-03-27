use hex::FromHex;
use js_sys::Array;
use wasm_bindgen::prelude::*;

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