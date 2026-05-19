/// ML fallback stub — to be populated once the ONNX model is trained.
///
/// The model activates only when oz-core's confidence drops below the
/// configured threshold (default 0.6). It handles OOV words: foreign
/// borrowings, neologisms, and social media forms.

pub struct MlFallback;

impl MlFallback {
    pub fn new(_model_path: &str) -> Result<Self, String> {
        // TODO: load ONNX model via `ort`
        Err("ML fallback not yet implemented — train and export model first".into())
    }

    pub fn stem(&self, _word: &str) -> (String, f32) {
        unimplemented!("oz-ml is a stub in Phase 1")
    }
}
