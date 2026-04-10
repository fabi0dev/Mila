use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use std::path::Path;

pub struct SttEngine {
    ctx: WhisperContext,
}

impl SttEngine {
    pub fn new(model_path: &str) -> anyhow::Result<Self> {
        if !Path::new(model_path).exists() {
            return Err(anyhow::anyhow!("Whisper model not found at {}", model_path));
        }

        let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
            .map_err(|_| anyhow::anyhow!("Failed to load Whisper model"))?;

        Ok(Self { ctx })
    }

    pub fn transcribe(&self, audio_data: &[f32]) -> anyhow::Result<String> {
        let mut state = self.ctx.create_state().map_err(|_| anyhow::anyhow!("Failed to create Whisper state"))?;
        
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_n_threads(4);
        params.set_language(Some("pt")); // Configured for Portuguese as requested by default or project context
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        state.full(params, audio_data).map_err(|_| anyhow::anyhow!("Whisper inference failed"))?;

        let mut result = String::new();
        for segment in state.as_iter() {
            result.push_str(&segment.to_string());
        }

        Ok(result.trim().to_string())
    }
}
