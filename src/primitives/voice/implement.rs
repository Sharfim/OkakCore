// okak_core/src/api/voice.rs
pub trait Voice {
    fn speak(&self, text: &str);
    fn listen(&self) -> String;
}
