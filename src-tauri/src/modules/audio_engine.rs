use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use anyhow::Result;
use tauri::AppHandle;
use crate::modules::audio::{emit_state, AppState};
use crate::modules::wake_word::WakeWordDetector;
use crate::modules::stt_engine::SttEngine;
use std::sync::mpsc::{channel, Sender, Receiver};

#[derive(Clone, Copy, PartialEq)]
pub enum EngineMode {
    WakeWord,
    Recording,
    Idle,
}

pub struct AudioEngine {
    app_handle: AppHandle,
    is_running: Arc<Mutex<bool>>,
}

impl AudioEngine {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) -> Result<()> {
        let mut running = self.is_running.lock().unwrap();
        if *running {
            return Ok(());
        }
        *running = true;

        let app_handle = self.app_handle.clone();
        let is_running = self.is_running.clone();

        std::thread::spawn(move || {
            if let Err(e) = run_capture_loop(app_handle, is_running) {
                eprintln!("Audio Engine Error: {:?}", e);
            }
        });

        Ok(())
    }
}

fn run_capture_loop(
    app_handle: AppHandle, 
    is_running: Arc<Mutex<bool>>
) -> Result<()> {
    let vosk_path = "models/vosk/model-small";
    let whisper_path = "models/whisper/ggml-base.en.bin";

    // Initialize engines
    let mut wake_detector = WakeWordDetector::new(vosk_path, 16000.0)?;
    let stt_engine = match SttEngine::new(whisper_path) {
        Ok(engine) => Some(engine),
        Err(e) => {
            eprintln!("STT Engine disabled: {:?}", e);
            None
        }
    };

    let host = cpal::default_host();
    let device = host.default_input_device().ok_or_else(|| anyhow::anyhow!("No input device found"))?;
    let config: cpal::StreamConfig = device.default_input_config()?.into();
    let sample_rate = config.sample_rate;
    let channels = config.channels;

    let (tx, rx): (Sender<Vec<f32>>, Receiver<Vec<f32>>) = channel();

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let _ = tx.send(data.to_vec());
        },
        |err| eprintln!("Stream error: {:?}", err),
        None,
    )?;

    stream.play()?;

    let mut mode = EngineMode::WakeWord;
    let mut recording_buffer: Vec<f32> = Vec::new();
    let mut silence_frames = 0;
    let silence_threshold = 0.01;
    let silence_limit = 50;

    println!("Mila Engine Online. Listening for 'Mila'...");

    while *is_running.lock().unwrap() {
        if let Ok(data) = rx.recv() {
            let mono_16khz = resample_and_mono(data, sample_rate, channels);
            
            match mode {
                EngineMode::WakeWord => {
                    let i16_samples: Vec<i16> = mono_16khz.iter().map(|&s| (s * 32767.0) as i16).collect();
                    if wake_detector.process(&i16_samples) {
                        println!("Mila detected!");
                        emit_state(&app_handle, AppState::WakeWordDetected);
                        mode = EngineMode::Recording;
                        recording_buffer.clear();
                        silence_frames = 0;
                    }
                }
                EngineMode::Recording => {
                    recording_buffer.extend_from_slice(&mono_16khz);
                    let energy = mono_16khz.iter().map(|&s| s * s).sum::<f32>() / mono_16khz.len() as f32;
                    if energy < silence_threshold {
                        silence_frames += 1;
                    } else {
                        silence_frames = 0;
                    }

                    if silence_frames > silence_limit && recording_buffer.len() > 16000 {
                        mode = EngineMode::Idle;
                        if let Some(stt) = &stt_engine {
                            println!("Processing transcription...");
                            if let Ok(text) = stt.transcribe(&recording_buffer) {
                                println!("Transcription: {}", text);
                                emit_state(&app_handle, AppState::SttResult(text));
                            }
                        }
                        mode = EngineMode::WakeWord;
                    }
                }
                EngineMode::Idle => {}
            }
        }
    }

    Ok(())
}

fn resample_and_mono(data: Vec<f32>, source_rate: u32, channels: u16) -> Vec<f32> {
    let mut mono = Vec::with_capacity(data.len() / channels as usize);
    for chunk in data.chunks_exact(channels as usize) {
        let sum: f32 = chunk.iter().sum();
        mono.push(sum / channels as f32);
    }
    if source_rate == 16000 { return mono; }
    let ratio = source_rate as f32 / 16000.0;
    let mut resampled = Vec::new();
    let mut i = 0.0;
    while i < mono.len() as f32 {
        let index = i as usize;
        if index < mono.len() { resampled.push(mono[index]); }
        i += ratio;
    }
    resampled
}
