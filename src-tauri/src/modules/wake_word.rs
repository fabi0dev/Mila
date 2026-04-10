use vosk::{Model, Recognizer};
use std::path::Path;

pub struct WakeWordDetector {
    recognizer: Recognizer,
}

impl WakeWordDetector {
    pub fn new(model_path: &str, sample_rate: f32) -> anyhow::Result<Self> {
        if !Path::new(model_path).exists() {
            // Log warning but allow creation (it will fail later if models aren't downloaded)
            eprintln!("Warning: Vosk model not found at {}", model_path);
        }

        let model = Model::new(model_path).ok_or_else(|| anyhow::anyhow!("Failed to load Vosk model"))?;
        
        // Grammar restricted to "mila" and "[unk]" for noise
        let grammar = ["mila", "[unk]"];
        let recognizer = Recognizer::new_with_grammar(&model, sample_rate, &grammar)
            .ok_or_else(|| anyhow::anyhow!("Failed to create Vosk recognizer"))?;

        Ok(Self { recognizer })
    }

    pub fn process(&mut self, samples: &[i16]) -> bool {
        self.recognizer.accept_waveform(samples);
        let partial = self.recognizer.partial_result();
        let text = partial.partial;
        
        if text.contains("mila") {
            self.recognizer.reset(); // Correct way to reset state in Vosk 0.3.1
            return true;
        }
        
        false
    }
}
