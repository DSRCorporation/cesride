use cesride_core::Result as CesrResult;
use js_sys::Error as JsError;

// Allows conversion of CesrError to javascript error
pub(crate) trait JsResult<T> {
    #[allow(clippy::wrong_self_convention)]
    fn as_js(self) -> Result<T, JsError>;
}

impl<T> JsResult<T> for CesrResult<T> {
    fn as_js(self) -> Result<T, JsError> {
        self.map_err(|e| JsError::new(&e.to_string()))
    }
}
