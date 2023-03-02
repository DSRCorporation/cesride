use crate::error::*;
use cesride_core::{Matter, Verfer};
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Verfer)]
pub struct VerferWrapper(pub(crate) Verfer);

#[wasm_bindgen(js_class = Verfer)]
impl VerferWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new_with_raw(raw: &[u8], code: Option<String>) -> Result<VerferWrapper, JsValue> {
        Ok(VerferWrapper(Verfer::new_with_raw(raw, code.as_deref()).as_js()?))
    }

    #[wasm_bindgen(constructor)]
    pub fn new_with_qb64(qb64: &str) -> Result<VerferWrapper, JsValue> {
        Ok(VerferWrapper(Verfer::new_with_qb64(qb64).as_js()?))
    }

    #[wasm_bindgen(constructor)]
    pub fn new_with_qb64b(qb64b: &[u8]) -> Result<VerferWrapper, JsValue> {
        Ok(VerferWrapper(Verfer::new_with_qb64b(qb64b).as_js()?))
    }

    #[wasm_bindgen(constructor)]
    pub fn new_with_qb2(qb2: &[u8]) -> Result<VerferWrapper, JsValue> {
        Ok(VerferWrapper(Verfer::new_with_qb2(qb2).as_js()?))
    }

    pub fn verify(&self, sig: &[u8], ser: &[u8]) -> Result<bool, JsValue> {
        Ok(self.0.verify(sig, ser).as_js()?)
    }

    pub fn code(&self) -> String {
        self.0.code()
    }

    pub fn size(&self) -> u32 {
        self.0.size()
    }

    pub fn raw(&self) -> Vec<u8> {
        self.0.raw()
    }

    pub fn qb64(&self) -> Result<String, JsValue> {
        Ok(self.0.qb64().as_js()?)
    }

    pub fn qb64b(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.0.qb64b().as_js()?)
    }

    pub fn qb2(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.0.qb2().as_js()?)
    }
}

impl Deref for VerferWrapper {
    type Target = Verfer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
