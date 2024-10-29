use druid::{Data, Lens};

#[derive(Clone, Data, Lens, Default)]
pub struct AppState {
    pub selected_file: String,
    pub transcribe_result: String,
    pub is_processing: bool,
}
