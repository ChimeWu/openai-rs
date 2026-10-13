#[doc = "`AudioApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AudioApiSchema {
    #[serde(rename = "CreateSpeechRequest")]
    pub create_speech_request: CreateSpeechRequest,
    #[serde(rename = "CreateSpeechStreamResponse")]
    pub create_speech_stream_response: CreateSpeechResponseStreamEvent,
    #[serde(rename = "CreateTranscriptionRequest")]
    pub create_transcription_request: CreateTranscriptionRequest,
    #[serde(rename = "CreateTranscriptionResponse")]
    pub create_transcription_response: CreateTranscriptionResponse,
    #[serde(rename = "CreateTranscriptionStreamResponse")]
    pub create_transcription_stream_response: CreateTranscriptionResponseStreamEvent,
    #[serde(rename = "CreateTranslationRequest")]
    pub create_translation_request: CreateTranslationRequest,
    #[serde(rename = "CreateTranslationResponse")]
    pub create_translation_response: CreateTranslationResponse,
    #[serde(rename = "CreateVoiceConsentRequest")]
    pub create_voice_consent_request: CreateVoiceConsentRequest,
    #[serde(rename = "CreateVoiceConsentResponse")]
    pub create_voice_consent_response: VoiceConsentResource,
    #[serde(rename = "CreateVoiceRequest")]
    pub create_voice_request: CreateVoiceRequest,
    #[serde(rename = "CreateVoiceResponse")]
    pub create_voice_response: VoiceResource,
    #[serde(rename = "DeleteVoiceConsentResponse")]
    pub delete_voice_consent_response: VoiceConsentDeletedResource,
    #[serde(rename = "GetVoiceConsentResponse")]
    pub get_voice_consent_response: VoiceConsentResource,
    #[serde(rename = "ListVoiceConsentsParams")]
    pub list_voice_consents_params: ListVoiceConsentsParams,
    #[serde(rename = "ListVoiceConsentsResponse")]
    pub list_voice_consents_response: VoiceConsentListResource,
    #[serde(rename = "UpdateVoiceConsentRequest")]
    pub update_voice_consent_request: UpdateVoiceConsentRequest,
    #[serde(rename = "UpdateVoiceConsentResponse")]
    pub update_voice_consent_response: VoiceConsentResource,
}
#[doc = "The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, `vtt`, or `diarized_json`. For `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`, the only supported format is `json`. For `gpt-4o-transcribe-diarize`, the supported formats are `json`, `text`, and `diarized_json`, with `diarized_json` required to receive speaker annotations.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AudioResponseFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "srt")]
    Srt,
    #[serde(rename = "verbose_json")]
    VerboseJson,
    #[serde(rename = "vtt")]
    Vtt,
    #[serde(rename = "diarized_json")]
    DiarizedJson,
}
impl ::std::fmt::Display for AudioResponseFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Json => f.write_str("json"),
            Self::Text => f.write_str("text"),
            Self::Srt => f.write_str("srt"),
            Self::VerboseJson => f.write_str("verbose_json"),
            Self::Vtt => f.write_str("vtt"),
            Self::DiarizedJson => f.write_str("diarized_json"),
        }
    }
}
impl ::std::str::FromStr for AudioResponseFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "json" => Ok(Self::Json),
            "text" => Ok(Self::Text),
            "srt" => Ok(Self::Srt),
            "verbose_json" => Ok(Self::VerboseJson),
            "vtt" => Ok(Self::Vtt),
            "diarized_json" => Ok(Self::DiarizedJson),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AudioResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AudioResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateSpeechRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateSpeechRequest {
    #[doc = "The text to generate audio for. The maximum length is 4096 characters."]
    pub input: CreateSpeechRequestInput,
    #[doc = "Control the voice of your generated audio with additional instructions. Does not work with `tts-1` or `tts-1-hd`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<CreateSpeechRequestInstructions>,
    #[doc = "One of the available [TTS models](https://developers.openai.com/api/docs/guides/text-to-speech): `tts-1`, `tts-1-hd`, `gpt-4o-mini-tts`, or `gpt-4o-mini-tts-2025-12-15`.\n"]
    pub model: ::std::string::String,
    #[doc = "The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<CreateSpeechRequestResponseFormat>,
    #[doc = "The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub speed: ::std::option::Option<f64>,
    #[doc = "The format to stream the audio in. Supported formats are `sse` and `audio`. `sse` is not supported for `tts-1` or `tts-1-hd`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream_format: ::std::option::Option<CreateSpeechRequestStreamFormat>,
    #[doc = "The voice to use when generating the audio. Supported built-in voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, `verse`, `marin`, and `cedar`. You may also provide a custom voice object with an `id`, for example `{ \"id\": \"voice_1234\" }`. Previews of the voices are available in the [Text to speech guide](https://developers.openai.com/api/docs/guides/text-to-speech#voice-options). Custom voices must be created from audio samples."]
    pub voice: Voice,
}
#[doc = "The text to generate audio for. The maximum length is 4096 characters."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateSpeechRequestInput(::std::string::String);
impl ::std::ops::Deref for CreateSpeechRequestInput {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateSpeechRequestInput> for ::std::string::String {
    fn from(value: CreateSpeechRequestInput) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateSpeechRequestInput {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4096usize {
            return Err("longer than 4096 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateSpeechRequestInput {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateSpeechRequestInput {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateSpeechRequestInput {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Control the voice of your generated audio with additional instructions. Does not work with `tts-1` or `tts-1-hd`."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateSpeechRequestInstructions(::std::string::String);
impl ::std::ops::Deref for CreateSpeechRequestInstructions {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateSpeechRequestInstructions> for ::std::string::String {
    fn from(value: CreateSpeechRequestInstructions) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateSpeechRequestInstructions {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4096usize {
            return Err("longer than 4096 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateSpeechRequestInstructions {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateSpeechRequestInstructions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateSpeechRequestInstructions {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateSpeechRequestResponseFormat {
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "opus")]
    Opus,
    #[serde(rename = "aac")]
    Aac,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "pcm")]
    Pcm,
}
impl ::std::fmt::Display for CreateSpeechRequestResponseFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mp3 => f.write_str("mp3"),
            Self::Opus => f.write_str("opus"),
            Self::Aac => f.write_str("aac"),
            Self::Flac => f.write_str("flac"),
            Self::Wav => f.write_str("wav"),
            Self::Pcm => f.write_str("pcm"),
        }
    }
}
impl ::std::str::FromStr for CreateSpeechRequestResponseFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mp3" => Ok(Self::Mp3),
            "opus" => Ok(Self::Opus),
            "aac" => Ok(Self::Aac),
            "flac" => Ok(Self::Flac),
            "wav" => Ok(Self::Wav),
            "pcm" => Ok(Self::Pcm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateSpeechRequestResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateSpeechRequestResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The format to stream the audio in. Supported formats are `sse` and `audio`. `sse` is not supported for `tts-1` or `tts-1-hd`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateSpeechRequestStreamFormat {
    #[serde(rename = "sse")]
    Sse,
    #[serde(rename = "audio")]
    Audio,
}
impl ::std::fmt::Display for CreateSpeechRequestStreamFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sse => f.write_str("sse"),
            Self::Audio => f.write_str("audio"),
        }
    }
}
impl ::std::str::FromStr for CreateSpeechRequestStreamFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "sse" => Ok(Self::Sse),
            "audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateSpeechRequestStreamFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateSpeechRequestStreamFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateSpeechResponseStreamEvent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateSpeechResponseStreamEvent {
    DeltaEvent(SpeechAudioDeltaEvent),
    DoneEvent(SpeechAudioDoneEvent),
}
impl ::std::convert::From<SpeechAudioDeltaEvent> for CreateSpeechResponseStreamEvent {
    fn from(value: SpeechAudioDeltaEvent) -> Self {
        Self::DeltaEvent(value)
    }
}
impl ::std::convert::From<SpeechAudioDoneEvent> for CreateSpeechResponseStreamEvent {
    fn from(value: SpeechAudioDoneEvent) -> Self {
        Self::DoneEvent(value)
    }
}
#[doc = "`CreateTranscriptionRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateTranscriptionRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub chunking_strategy: ::std::option::Option<VadConfig>,
    #[doc = "The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.\nThe request must include enough format metadata for the file to be identified. We recommend an extension-bearing filename and an appropriate content type.\n"]
    pub file: ::std::string::String,
    #[doc = "Additional information to include in the transcription response.\n`logprobs` will return the log probabilities of the tokens in the\nresponse to understand the model's confidence in the transcription.\n`logprobs` only works with response_format set to `json` and only with\nthe models `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `gpt-4o-mini-transcribe-2025-12-15`. This field is not supported when using `gpt-4o-transcribe-diarize`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<TranscriptionInclude>,
    #[doc = "Words or phrases to guide transcription of the input audio. Supported by `gpt-transcribe`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub keywords: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional list of speaker names that correspond to the audio samples provided in `known_speaker_references[]`. Each entry should be a short identifier (for example `customer` or `agent`). Up to 4 speakers are supported.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub known_speaker_names: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional list of audio samples (as [data URLs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs)) that contain known speaker references matching `known_speaker_names[]`. Each sample must be between 2 and 10 seconds, and can use any of the same input audio formats supported by `file`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub known_speaker_references: ::std::vec::Vec<::std::string::String>,
    #[doc = "The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[doc = "Possible languages of the input audio, in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format. Supported by `gpt-transcribe`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<::std::string::String>,
    #[doc = "ID of the model to use. The options are `gpt-transcribe`, `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, `gpt-4o-mini-transcribe-2025-12-15`, `whisper-1` (which is powered by our open source Whisper V2 model), and `gpt-4o-transcribe-diarize`.\n"]
    pub model: ::std::string::String,
    #[doc = "An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://developers.openai.com/api/docs/guides/speech-to-text#prompting) should match the audio language. This field is not supported when using `gpt-4o-transcribe-diarize`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<AudioResponseFormat>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream: ::std::option::Option<bool>,
    #[doc = "The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[doc = "The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency.\nThis option is not available for `gpt-4o-transcribe-diarize`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub timestamp_granularities:
        ::std::vec::Vec<CreateTranscriptionRequestTimestampGranularitiesItem>,
}
#[doc = "`CreateTranscriptionRequestTimestampGranularitiesItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateTranscriptionRequestTimestampGranularitiesItem {
    #[serde(rename = "word")]
    Word,
    #[serde(rename = "segment")]
    Segment,
}
impl ::std::fmt::Display for CreateTranscriptionRequestTimestampGranularitiesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Word => f.write_str("word"),
            Self::Segment => f.write_str("segment"),
        }
    }
}
impl ::std::str::FromStr for CreateTranscriptionRequestTimestampGranularitiesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "word" => Ok(Self::Word),
            "segment" => Ok(Self::Segment),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateTranscriptionRequestTimestampGranularitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateTranscriptionRequestTimestampGranularitiesItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateTranscriptionResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateTranscriptionResponse {
    Json(CreateTranscriptionResponseJson),
    DiarizedJson(CreateTranscriptionResponseDiarizedJson),
    VerboseJson(CreateTranscriptionResponseVerboseJson),
}
impl ::std::convert::From<CreateTranscriptionResponseJson> for CreateTranscriptionResponse {
    fn from(value: CreateTranscriptionResponseJson) -> Self {
        Self::Json(value)
    }
}
impl ::std::convert::From<CreateTranscriptionResponseDiarizedJson> for CreateTranscriptionResponse {
    fn from(value: CreateTranscriptionResponseDiarizedJson) -> Self {
        Self::DiarizedJson(value)
    }
}
impl ::std::convert::From<CreateTranscriptionResponseVerboseJson> for CreateTranscriptionResponse {
    fn from(value: CreateTranscriptionResponseVerboseJson) -> Self {
        Self::VerboseJson(value)
    }
}
#[doc = "Represents a diarized transcription response returned by the model, including the combined transcript and speaker-segment annotations.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateTranscriptionResponseDiarizedJson {
    #[doc = "Duration of the input audio in seconds."]
    pub duration: f64,
    #[doc = "Segments of the transcript annotated with timestamps and speaker labels."]
    pub segments: ::std::vec::Vec<TranscriptionDiarizedSegment>,
    #[doc = "The type of task that was run. Always `transcribe`."]
    pub task: CreateTranscriptionResponseDiarizedJsonTask,
    #[doc = "The concatenated transcript text for the entire audio input."]
    pub text: ::std::string::String,
    #[doc = "Token or duration usage statistics for the request."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<CreateTranscriptionResponseDiarizedJsonUsage>,
}
#[doc = "The type of task that was run. Always `transcribe`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateTranscriptionResponseDiarizedJsonTask {
    #[serde(rename = "transcribe")]
    Transcribe,
}
impl ::std::fmt::Display for CreateTranscriptionResponseDiarizedJsonTask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transcribe => f.write_str("transcribe"),
        }
    }
}
impl ::std::str::FromStr for CreateTranscriptionResponseDiarizedJsonTask {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcribe" => Ok(Self::Transcribe),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateTranscriptionResponseDiarizedJsonTask {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateTranscriptionResponseDiarizedJsonTask
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Token or duration usage statistics for the request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateTranscriptionResponseDiarizedJsonUsage {
    Tokens(TranscriptTextUsageTokens),
    Duration(TranscriptTextUsageDuration),
}
impl ::std::convert::From<TranscriptTextUsageTokens>
    for CreateTranscriptionResponseDiarizedJsonUsage
{
    fn from(value: TranscriptTextUsageTokens) -> Self {
        Self::Tokens(value)
    }
}
impl ::std::convert::From<TranscriptTextUsageDuration>
    for CreateTranscriptionResponseDiarizedJsonUsage
{
    fn from(value: TranscriptTextUsageDuration) -> Self {
        Self::Duration(value)
    }
}
#[doc = "Represents a transcription response returned by model, based on the provided input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateTranscriptionResponseJson {
    #[doc = "The languages detected in the audio. Returned by `gpt-transcribe`. An empty array indicates that no language could be reliably detected.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<TranscriptionLanguage>,
    #[doc = "The log probabilities of the tokens in the transcription. Only returned with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub logprobs: ::std::vec::Vec<CreateTranscriptionResponseJsonLogprobsItem>,
    #[doc = "The transcribed text."]
    pub text: ::std::string::String,
    #[doc = "Token usage statistics for the request."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<CreateTranscriptionResponseJsonUsage>,
}
#[doc = "`CreateTranscriptionResponseJsonLogprobsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateTranscriptionResponseJsonLogprobsItem {
    #[doc = "The bytes of the token."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub bytes: ::std::vec::Vec<f64>,
    #[doc = "The log probability of the token."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logprob: ::std::option::Option<f64>,
    #[doc = "The token in the transcription."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
}
#[doc = "Token usage statistics for the request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateTranscriptionResponseJsonUsage {
    Tokens(TranscriptTextUsageTokens),
    Duration(TranscriptTextUsageDuration),
}
impl ::std::convert::From<TranscriptTextUsageTokens> for CreateTranscriptionResponseJsonUsage {
    fn from(value: TranscriptTextUsageTokens) -> Self {
        Self::Tokens(value)
    }
}
impl ::std::convert::From<TranscriptTextUsageDuration> for CreateTranscriptionResponseJsonUsage {
    fn from(value: TranscriptTextUsageDuration) -> Self {
        Self::Duration(value)
    }
}
#[doc = "`CreateTranscriptionResponseStreamEvent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateTranscriptionResponseStreamEvent {
    SegmentEvent(TranscriptTextSegmentEvent),
    DeltaEvent(TranscriptTextDeltaEvent),
    DoneEvent(TranscriptTextDoneEvent),
}
impl ::std::convert::From<TranscriptTextSegmentEvent> for CreateTranscriptionResponseStreamEvent {
    fn from(value: TranscriptTextSegmentEvent) -> Self {
        Self::SegmentEvent(value)
    }
}
impl ::std::convert::From<TranscriptTextDeltaEvent> for CreateTranscriptionResponseStreamEvent {
    fn from(value: TranscriptTextDeltaEvent) -> Self {
        Self::DeltaEvent(value)
    }
}
impl ::std::convert::From<TranscriptTextDoneEvent> for CreateTranscriptionResponseStreamEvent {
    fn from(value: TranscriptTextDoneEvent) -> Self {
        Self::DoneEvent(value)
    }
}
#[doc = "Represents a verbose json transcription response returned by model, based on the provided input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateTranscriptionResponseVerboseJson {
    #[doc = "The duration of the input audio."]
    pub duration: f64,
    #[doc = "The language of the input audio."]
    pub language: ::std::string::String,
    #[doc = "Segments of the transcribed text and their corresponding details."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub segments: ::std::vec::Vec<TranscriptionSegment>,
    #[doc = "The transcribed text."]
    pub text: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<TranscriptTextUsageDuration>,
    #[doc = "Extracted words and their corresponding timestamps."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub words: ::std::vec::Vec<TranscriptionWord>,
}
#[doc = "`CreateTranslationRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateTranslationRequest {
    #[doc = "The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm. The request must include enough format metadata for the file to be identified. We recommend an extension-bearing filename and an appropriate content type.\n"]
    pub file: ::std::string::String,
    #[doc = "ID of the model to use. Only `whisper-1` (which is powered by our open source Whisper V2 model) is currently available.\n"]
    pub model: ::std::string::String,
    #[doc = "An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://developers.openai.com/api/docs/guides/speech-to-text#prompting) should be in English.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<::std::string::String>,
    #[doc = "The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<CreateTranslationRequestResponseFormat>,
    #[doc = "The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
}
#[doc = "The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateTranslationRequestResponseFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "srt")]
    Srt,
    #[serde(rename = "verbose_json")]
    VerboseJson,
    #[serde(rename = "vtt")]
    Vtt,
}
impl ::std::fmt::Display for CreateTranslationRequestResponseFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Json => f.write_str("json"),
            Self::Text => f.write_str("text"),
            Self::Srt => f.write_str("srt"),
            Self::VerboseJson => f.write_str("verbose_json"),
            Self::Vtt => f.write_str("vtt"),
        }
    }
}
impl ::std::str::FromStr for CreateTranslationRequestResponseFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "json" => Ok(Self::Json),
            "text" => Ok(Self::Text),
            "srt" => Ok(Self::Srt),
            "verbose_json" => Ok(Self::VerboseJson),
            "vtt" => Ok(Self::Vtt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateTranslationRequestResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateTranslationRequestResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateTranslationResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateTranslationResponse {
    Json(CreateTranslationResponseJson),
    VerboseJson(CreateTranslationResponseVerboseJson),
}
impl ::std::convert::From<CreateTranslationResponseJson> for CreateTranslationResponse {
    fn from(value: CreateTranslationResponseJson) -> Self {
        Self::Json(value)
    }
}
impl ::std::convert::From<CreateTranslationResponseVerboseJson> for CreateTranslationResponse {
    fn from(value: CreateTranslationResponseVerboseJson) -> Self {
        Self::VerboseJson(value)
    }
}
#[doc = "`CreateTranslationResponseJson`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateTranslationResponseJson {
    pub text: ::std::string::String,
}
#[doc = "`CreateTranslationResponseVerboseJson`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateTranslationResponseVerboseJson {
    #[doc = "The duration of the input audio."]
    pub duration: f64,
    #[doc = "The language of the output translation (always `english`)."]
    pub language: ::std::string::String,
    #[doc = "Segments of the translated text and their corresponding details."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub segments: ::std::vec::Vec<TranscriptionSegment>,
    #[doc = "The translated text."]
    pub text: ::std::string::String,
}
#[doc = "`CreateVoiceConsentRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateVoiceConsentRequest {
    #[doc = "The BCP 47 language tag for the consent phrase (for example, `en-US`)."]
    pub language: ::std::string::String,
    #[doc = "The label to use for this consent recording."]
    pub name: ::std::string::String,
    #[doc = "The consent audio recording file. Maximum size is 10 MiB.\n\nSupported MIME types:\n`audio/mpeg`, `audio/wav`, `audio/x-wav`, `audio/ogg`, `audio/aac`, `audio/flac`, `audio/webm`, `audio/mp4`.\n"]
    pub recording: ::std::string::String,
}
#[doc = "Creates a voice from a consent recording and an audio sample. Requires multipart/form-data."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateVoiceFromConsentRequest {
    #[doc = "The sample audio recording file. Maximum size is 10 MiB.\n\nSupported MIME types:\n`audio/mpeg`, `audio/wav`, `audio/x-wav`, `audio/ogg`, `audio/aac`, `audio/flac`, `audio/webm`, `audio/mp4`.\n"]
    pub audio_sample: ::std::string::String,
    #[doc = "The consent recording ID (for example, `cons_1234`)."]
    pub consent: ::std::string::String,
    #[doc = "The name of the new voice."]
    pub name: CreateVoiceFromConsentRequestName,
    #[doc = "The voice creation method. Defaults to `audio_sample` when omitted."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<CreateVoiceFromConsentRequestType>,
}
#[doc = "The name of the new voice."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVoiceFromConsentRequestName(::std::string::String);
impl ::std::ops::Deref for CreateVoiceFromConsentRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVoiceFromConsentRequestName> for ::std::string::String {
    fn from(value: CreateVoiceFromConsentRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVoiceFromConsentRequestName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVoiceFromConsentRequestName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVoiceFromConsentRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVoiceFromConsentRequestName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The voice creation method. Defaults to `audio_sample` when omitted."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateVoiceFromConsentRequestType {
    #[serde(rename = "audio_sample")]
    AudioSample,
}
impl ::std::fmt::Display for CreateVoiceFromConsentRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioSample => f.write_str("audio_sample"),
        }
    }
}
impl ::std::str::FromStr for CreateVoiceFromConsentRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio_sample" => Ok(Self::AudioSample),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateVoiceFromConsentRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVoiceFromConsentRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateVoiceRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct CreateVoiceRequest(pub CreateVoiceFromConsentRequest);
impl ::std::ops::Deref for CreateVoiceRequest {
    type Target = CreateVoiceFromConsentRequest;
    fn deref(&self) -> &CreateVoiceFromConsentRequest {
        &self.0
    }
}
impl ::std::convert::From<CreateVoiceRequest> for CreateVoiceFromConsentRequest {
    fn from(value: CreateVoiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<CreateVoiceFromConsentRequest> for CreateVoiceRequest {
    fn from(value: CreateVoiceFromConsentRequest) -> Self {
        Self(value)
    }
}
#[doc = "`ListVoiceConsentsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListVoiceConsentsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
}
#[doc = "Emitted for each chunk of audio data generated during speech synthesis."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SpeechAudioDeltaEvent {
    #[doc = "A chunk of Base64-encoded audio data.\n"]
    pub audio: ::std::string::String,
    #[doc = "The type of the event. Always `speech.audio.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: SpeechAudioDeltaEventType,
}
#[doc = "The type of the event. Always `speech.audio.delta`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SpeechAudioDeltaEventType {
    #[serde(rename = "speech.audio.delta")]
    SpeechAudioDelta,
}
impl ::std::fmt::Display for SpeechAudioDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SpeechAudioDelta => f.write_str("speech.audio.delta"),
        }
    }
}
impl ::std::str::FromStr for SpeechAudioDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "speech.audio.delta" => Ok(Self::SpeechAudioDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SpeechAudioDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpeechAudioDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the speech synthesis is complete and all audio has been streamed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SpeechAudioDoneEvent {
    #[doc = "The type of the event. Always `speech.audio.done`.\n"]
    #[serde(rename = "type")]
    pub type_: SpeechAudioDoneEventType,
    pub usage: SpeechAudioDoneEventUsage,
}
#[doc = "The type of the event. Always `speech.audio.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SpeechAudioDoneEventType {
    #[serde(rename = "speech.audio.done")]
    SpeechAudioDone,
}
impl ::std::fmt::Display for SpeechAudioDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SpeechAudioDone => f.write_str("speech.audio.done"),
        }
    }
}
impl ::std::str::FromStr for SpeechAudioDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "speech.audio.done" => Ok(Self::SpeechAudioDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SpeechAudioDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpeechAudioDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Token usage statistics for the request.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SpeechAudioDoneEventUsage {
    #[doc = "Number of input tokens in the prompt."]
    pub input_tokens: i64,
    #[doc = "Number of output tokens generated."]
    pub output_tokens: i64,
    #[doc = "Total number of tokens used (input + output)."]
    pub total_tokens: i64,
}
#[doc = "Emitted when there is an additional text delta. This is also the first event emitted when the transcription starts. Only emitted when you [create a transcription](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) with the `Stream` parameter set to `true`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptTextDeltaEvent {
    #[doc = "The text delta that was additionally transcribed.\n"]
    pub delta: ::std::string::String,
    #[doc = "The log probabilities of the delta. Only included if you [create a transcription](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) with the `include[]` parameter set to `logprobs`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub logprobs: ::std::vec::Vec<TranscriptTextDeltaEventLogprobsItem>,
    #[doc = "Identifier of the diarized segment that this delta belongs to. Only present when using `gpt-4o-transcribe-diarize`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub segment_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the event. Always `transcript.text.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: TranscriptTextDeltaEventType,
}
#[doc = "`TranscriptTextDeltaEventLogprobsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct TranscriptTextDeltaEventLogprobsItem {
    #[doc = "The bytes that were used to generate the log probability.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub bytes: ::std::vec::Vec<i64>,
    #[doc = "The log probability of the token.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logprob: ::std::option::Option<f64>,
    #[doc = "The token that was used to generate the log probability.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
}
#[doc = "The type of the event. Always `transcript.text.delta`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TranscriptTextDeltaEventType {
    #[serde(rename = "transcript.text.delta")]
    TranscriptTextDelta,
}
impl ::std::fmt::Display for TranscriptTextDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TranscriptTextDelta => f.write_str("transcript.text.delta"),
        }
    }
}
impl ::std::str::FromStr for TranscriptTextDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcript.text.delta" => Ok(Self::TranscriptTextDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptTextDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptTextDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the transcription is complete. Contains the complete transcription text. Only emitted when you [create a transcription](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) with the `Stream` parameter set to `true`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptTextDoneEvent {
    #[doc = "The languages detected in the audio. Returned by `gpt-transcribe`. An empty array indicates that no language could be reliably detected.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<TranscriptionLanguage>,
    #[doc = "The log probabilities of the individual tokens in the transcription. Only included if you [create a transcription](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) with the `include[]` parameter set to `logprobs`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub logprobs: ::std::vec::Vec<TranscriptTextDoneEventLogprobsItem>,
    #[doc = "The text that was transcribed.\n"]
    pub text: ::std::string::String,
    #[doc = "The type of the event. Always `transcript.text.done`.\n"]
    #[serde(rename = "type")]
    pub type_: TranscriptTextDoneEventType,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<TranscriptTextUsageTokens>,
}
#[doc = "`TranscriptTextDoneEventLogprobsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct TranscriptTextDoneEventLogprobsItem {
    #[doc = "The bytes that were used to generate the log probability.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub bytes: ::std::vec::Vec<i64>,
    #[doc = "The log probability of the token.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logprob: ::std::option::Option<f64>,
    #[doc = "The token that was used to generate the log probability.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
}
#[doc = "The type of the event. Always `transcript.text.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TranscriptTextDoneEventType {
    #[serde(rename = "transcript.text.done")]
    TranscriptTextDone,
}
impl ::std::fmt::Display for TranscriptTextDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TranscriptTextDone => f.write_str("transcript.text.done"),
        }
    }
}
impl ::std::str::FromStr for TranscriptTextDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcript.text.done" => Ok(Self::TranscriptTextDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptTextDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptTextDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a diarized transcription returns a completed segment with speaker information. Only emitted when you [create a transcription](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) with `stream` set to `true` and `response_format` set to `diarized_json`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptTextSegmentEvent {
    #[doc = "End timestamp of the segment in seconds."]
    pub end: f64,
    #[doc = "Unique identifier for the segment."]
    pub id: ::std::string::String,
    #[doc = "Speaker label for this segment."]
    pub speaker: ::std::string::String,
    #[doc = "Start timestamp of the segment in seconds."]
    pub start: f64,
    #[doc = "Transcript text for this segment."]
    pub text: ::std::string::String,
    #[doc = "The type of the event. Always `transcript.text.segment`."]
    #[serde(rename = "type")]
    pub type_: TranscriptTextSegmentEventType,
}
#[doc = "The type of the event. Always `transcript.text.segment`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TranscriptTextSegmentEventType {
    #[serde(rename = "transcript.text.segment")]
    TranscriptTextSegment,
}
impl ::std::fmt::Display for TranscriptTextSegmentEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TranscriptTextSegment => f.write_str("transcript.text.segment"),
        }
    }
}
impl ::std::str::FromStr for TranscriptTextSegmentEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcript.text.segment" => Ok(Self::TranscriptTextSegment),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptTextSegmentEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptTextSegmentEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Usage statistics for models billed by audio input duration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptTextUsageDuration {
    #[doc = "Duration of the input audio in seconds."]
    pub seconds: f64,
    #[doc = "The type of the usage object. Always `duration` for this variant."]
    #[serde(rename = "type")]
    pub type_: TranscriptTextUsageDurationType,
}
#[doc = "The type of the usage object. Always `duration` for this variant."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TranscriptTextUsageDurationType {
    #[serde(rename = "duration")]
    Duration,
}
impl ::std::fmt::Display for TranscriptTextUsageDurationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Duration => f.write_str("duration"),
        }
    }
}
impl ::std::str::FromStr for TranscriptTextUsageDurationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "duration" => Ok(Self::Duration),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptTextUsageDurationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptTextUsageDurationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Usage statistics for models billed by token usage."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptTextUsageTokens {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_token_details: ::std::option::Option<TranscriptTextUsageTokensInputTokenDetails>,
    #[doc = "Number of input tokens billed for this request."]
    pub input_tokens: i64,
    #[doc = "Number of output tokens generated."]
    pub output_tokens: i64,
    #[doc = "Total number of tokens used (input + output)."]
    pub total_tokens: i64,
    #[doc = "The type of the usage object. Always `tokens` for this variant."]
    #[serde(rename = "type")]
    pub type_: TranscriptTextUsageTokensType,
}
#[doc = "Details about the input tokens billed for this request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct TranscriptTextUsageTokensInputTokenDetails {
    #[doc = "Number of audio tokens billed for this request."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_tokens: ::std::option::Option<i64>,
    #[doc = "Number of text tokens billed for this request."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text_tokens: ::std::option::Option<i64>,
}
#[doc = "The type of the usage object. Always `tokens` for this variant."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TranscriptTextUsageTokensType {
    #[serde(rename = "tokens")]
    Tokens,
}
impl ::std::fmt::Display for TranscriptTextUsageTokensType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tokens => f.write_str("tokens"),
        }
    }
}
impl ::std::str::FromStr for TranscriptTextUsageTokensType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tokens" => Ok(Self::Tokens),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptTextUsageTokensType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptTextUsageTokensType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A segment of diarized transcript text with speaker metadata."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptionDiarizedSegment {
    #[doc = "End timestamp of the segment in seconds."]
    pub end: f64,
    #[doc = "Unique identifier for the segment."]
    pub id: ::std::string::String,
    #[doc = "Speaker label for this segment. When known speakers are provided, the label matches `known_speaker_names[]`. Otherwise speakers are labeled sequentially using capital letters (`A`, `B`, ...).\n"]
    pub speaker: ::std::string::String,
    #[doc = "Start timestamp of the segment in seconds."]
    pub start: f64,
    #[doc = "Transcript text for this segment."]
    pub text: ::std::string::String,
    #[doc = "The type of the segment. Always `transcript.text.segment`.\n"]
    #[serde(rename = "type")]
    pub type_: TranscriptionDiarizedSegmentType,
}
#[doc = "The type of the segment. Always `transcript.text.segment`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TranscriptionDiarizedSegmentType {
    #[serde(rename = "transcript.text.segment")]
    TranscriptTextSegment,
}
impl ::std::fmt::Display for TranscriptionDiarizedSegmentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TranscriptTextSegment => f.write_str("transcript.text.segment"),
        }
    }
}
impl ::std::str::FromStr for TranscriptionDiarizedSegmentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcript.text.segment" => Ok(Self::TranscriptTextSegment),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptionDiarizedSegmentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptionDiarizedSegmentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`TranscriptionInclude`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TranscriptionInclude {
    #[serde(rename = "logprobs")]
    Logprobs,
}
impl ::std::fmt::Display for TranscriptionInclude {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Logprobs => f.write_str("logprobs"),
        }
    }
}
impl ::std::str::FromStr for TranscriptionInclude {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "logprobs" => Ok(Self::Logprobs),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptionInclude {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptionInclude {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A language detected in transcribed audio."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptionLanguage {
    #[doc = "The code of a language detected in the audio."]
    pub code: ::std::string::String,
}
#[doc = "`TranscriptionSegment`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptionSegment {
    #[doc = "Average logprob of the segment. If the value is lower than -1, consider the logprobs failed."]
    pub avg_logprob: f32,
    #[doc = "Compression ratio of the segment. If the value is greater than 2.4, consider the compression failed."]
    pub compression_ratio: f32,
    #[doc = "End time of the segment in seconds."]
    pub end: f64,
    #[doc = "Unique identifier of the segment."]
    pub id: i64,
    #[doc = "Probability of no speech in the segment. If the value is higher than 1.0 and the `avg_logprob` is below -1, consider this segment silent."]
    pub no_speech_prob: f32,
    #[doc = "Seek offset of the segment."]
    pub seek: i64,
    #[doc = "Start time of the segment in seconds."]
    pub start: f64,
    #[doc = "Temperature parameter used for generating the segment."]
    pub temperature: f32,
    #[doc = "Text content of the segment."]
    pub text: ::std::string::String,
    #[doc = "Array of token IDs for the text content."]
    pub tokens: ::std::vec::Vec<i64>,
}
#[doc = "`TranscriptionWord`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptionWord {
    #[doc = "End time of the word in seconds."]
    pub end: f64,
    #[doc = "Start time of the word in seconds."]
    pub start: f64,
    #[doc = "The text content of the word."]
    pub word: ::std::string::String,
}
#[doc = "`UpdateVoiceConsentRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UpdateVoiceConsentRequest {
    #[doc = "The updated label for this consent recording."]
    pub name: ::std::string::String,
}
#[doc = "`VadConfig`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VadConfig {
    #[doc = "Amount of audio to include before the VAD detected speech (in \nmilliseconds).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prefix_padding_ms: ::std::option::Option<i64>,
    #[doc = "Duration of silence to detect speech stop (in milliseconds).\nWith shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub silence_duration_ms: ::std::option::Option<i64>,
    #[doc = "Sensitivity threshold (0.0 to 1.0) for voice activity detection. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub threshold: ::std::option::Option<f64>,
    #[doc = "Must be set to `server_vad` to enable manual chunking using server side VAD."]
    #[serde(rename = "type")]
    pub type_: VadConfigType,
}
#[doc = "Must be set to `server_vad` to enable manual chunking using server side VAD."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VadConfigType {
    #[serde(rename = "server_vad")]
    ServerVad,
}
impl ::std::fmt::Display for VadConfigType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ServerVad => f.write_str("server_vad"),
        }
    }
}
impl ::std::str::FromStr for VadConfigType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "server_vad" => Ok(Self::ServerVad),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VadConfigType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VadConfigType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The voice to use when generating the audio. Supported built-in voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, `verse`, `marin`, and `cedar`. You may also provide a custom voice object with an `id`, for example `{ \"id\": \"voice_1234\" }`. Previews of the voices are available in the [Text to speech guide](https://developers.openai.com/api/docs/guides/text-to-speech#voice-options). Custom voices must be created from audio samples."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged, deny_unknown_fields)]
pub enum Voice {
    VoiceIdsShared(VoiceIdsShared),
    Object {
        #[doc = "The custom voice ID, e.g. `voice_1234`."]
        id: ::std::string::String,
    },
}
impl ::std::convert::From<VoiceIdsShared> for Voice {
    fn from(value: VoiceIdsShared) -> Self {
        Self::VoiceIdsShared(value)
    }
}
#[doc = "`VoiceConsentDeletedResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VoiceConsentDeletedResource {
    pub deleted: bool,
    #[doc = "The consent recording identifier."]
    pub id: ::std::string::String,
    pub object: VoiceConsentDeletedResourceObject,
}
#[doc = "`VoiceConsentDeletedResourceObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VoiceConsentDeletedResourceObject {
    #[serde(rename = "audio.voice_consent")]
    AudioVoiceConsent,
}
impl ::std::fmt::Display for VoiceConsentDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioVoiceConsent => f.write_str("audio.voice_consent"),
        }
    }
}
impl ::std::str::FromStr for VoiceConsentDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio.voice_consent" => Ok(Self::AudioVoiceConsent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VoiceConsentDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceConsentDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VoiceConsentListResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VoiceConsentListResource {
    pub data: ::std::vec::Vec<VoiceConsentResource>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: VoiceConsentListResourceObject,
}
#[doc = "`VoiceConsentListResourceObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VoiceConsentListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for VoiceConsentListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for VoiceConsentListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VoiceConsentListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceConsentListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A consent recording used to authorize creation of a custom voice."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VoiceConsentResource {
    #[doc = "The Unix timestamp (in seconds) for when the consent recording was created."]
    pub created_at: i64,
    #[doc = "The consent recording identifier."]
    pub id: ::std::string::String,
    #[doc = "The BCP 47 language tag for the consent phrase (for example, `en-US`)."]
    pub language: ::std::string::String,
    #[doc = "The label provided when the consent recording was uploaded."]
    pub name: ::std::string::String,
    #[doc = "The object type, which is always `audio.voice_consent`."]
    pub object: VoiceConsentResourceObject,
}
#[doc = "The object type, which is always `audio.voice_consent`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VoiceConsentResourceObject {
    #[serde(rename = "audio.voice_consent")]
    AudioVoiceConsent,
}
impl ::std::fmt::Display for VoiceConsentResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioVoiceConsent => f.write_str("audio.voice_consent"),
        }
    }
}
impl ::std::str::FromStr for VoiceConsentResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio.voice_consent" => Ok(Self::AudioVoiceConsent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VoiceConsentResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceConsentResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VoiceIdsShared`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct VoiceIdsShared(pub ::std::string::String);
impl ::std::ops::Deref for VoiceIdsShared {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceIdsShared> for ::std::string::String {
    fn from(value: VoiceIdsShared) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for VoiceIdsShared {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for VoiceIdsShared {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for VoiceIdsShared {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "A custom voice that can be used for audio output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VoiceResource {
    #[doc = "The Unix timestamp (in seconds) for when the voice was created."]
    pub created_at: i64,
    #[doc = "The voice identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The name of the voice."]
    pub name: ::std::string::String,
    #[doc = "The object type, which is always `audio.voice`."]
    pub object: VoiceResourceObject,
    #[doc = "How the voice was created."]
    #[serde(rename = "type")]
    pub type_: VoiceResourceType,
}
#[doc = "The object type, which is always `audio.voice`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VoiceResourceObject {
    #[serde(rename = "audio.voice")]
    AudioVoice,
}
impl ::std::fmt::Display for VoiceResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioVoice => f.write_str("audio.voice"),
        }
    }
}
impl ::std::str::FromStr for VoiceResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio.voice" => Ok(Self::AudioVoice),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VoiceResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "How the voice was created."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VoiceResourceType {
    #[serde(rename = "audio_sample")]
    AudioSample,
}
impl ::std::fmt::Display for VoiceResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioSample => f.write_str("audio_sample"),
        }
    }
}
impl ::std::str::FromStr for VoiceResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio_sample" => Ok(Self::AudioSample),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VoiceResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = " Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
