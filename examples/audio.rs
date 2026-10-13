use bytes::Bytes;
use openai_rs::{AudioFile, AudioSpeechResponse, AudioTranscriptionResponse, OpenAIClient, types};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;

    let speech = types::audio::CreateSpeechRequest {
        input: "Audio streams are read incrementally.".parse()?,
        instructions: None,
        model: "gpt-4o-mini-tts".to_owned(),
        response_format: None,
        speed: None,
        stream_format: None,
        voice: types::audio::Voice::VoiceIdsShared(types::audio::VoiceIdsShared(
            "marin".to_owned(),
        )),
    };
    match client.audio().create_speech(&speech).await? {
        AudioSpeechResponse::Audio(mut audio) => {
            let mut contents = Vec::new();
            while let Some(chunk) = audio.next_chunk().await {
                contents.extend_from_slice(&chunk?);
            }
            std::fs::write("speech.mp3", contents)?;
        }
        AudioSpeechResponse::Events(mut events) => {
            while let Some(event) = events.next_event().await {
                println!("{}", event?.event_type);
            }
        }
    }

    let audio_path = std::env::var("AUDIO_FILE")?;
    let filename = Path::new(&audio_path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("AUDIO_FILE must include a valid filename")?;
    let contents = Bytes::from(std::fs::read(&audio_path)?);
    let request: types::audio::CreateTranscriptionRequest =
        serde_json::from_value(serde_json::json!({
            "file": filename,
            "model": "whisper-1",
            "response_format": "json"
        }))?;
    let transcription = client
        .audio()
        .transcribe(&request, AudioFile::new(filename, "audio/wav", contents))
        .await?;
    match transcription {
        AudioTranscriptionResponse::Json(response) => match response.data {
            types::audio::CreateTranscriptionResponse::Json(result) => println!("{}", result.text),
            types::audio::CreateTranscriptionResponse::DiarizedJson(result) => {
                println!("{}", result.text)
            }
            types::audio::CreateTranscriptionResponse::VerboseJson(result) => {
                println!("{}", result.text)
            }
        },
        AudioTranscriptionResponse::Text(response) => println!("{}", response.text),
        AudioTranscriptionResponse::Events(mut events) => {
            while let Some(event) = events.next_event().await {
                println!("{}", event?.event_type);
            }
        }
    }
    Ok(())
}
