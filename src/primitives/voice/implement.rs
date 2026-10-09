// okak_core/src/api/voice.rs
pub trait Voice {
    fn speak(&self, text: &str) -> Result<(), String>;
    fn listen(&self) -> Result<String, String>;
}
