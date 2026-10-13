pub use crate::common::VoiceIdsOrCustomVoice as Voice;
pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, TranscriptTextUsageDuration, TranscriptTextUsageTokens, TranscriptionLanguage, VoiceIdsOrCustomVoice, VoiceIdsShared, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`AudioApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AudioApiSchema {
    #[serde(rename = "CreateSpeechError400")]
    pub create_speech_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSpeechError401")]
    pub create_speech_error401: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSpeechError429")]
    pub create_speech_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSpeechError500")]
    pub create_speech_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSpeechError503")]
    pub create_speech_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSpeechRequest")]
    pub create_speech_request: CreateSpeechRequest,
    #[serde(rename = "CreateSpeechStreamResponse")]
    pub create_speech_stream_response: CreateSpeechResponseStreamEvent,
    #[serde(rename = "CreateTranscriptionError400")]
    pub create_transcription_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranscriptionError401")]
    pub create_transcription_error401: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranscriptionError413")]
    pub create_transcription_error413: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranscriptionError429")]
    pub create_transcription_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranscriptionError500")]
    pub create_transcription_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranscriptionError502")]
    pub create_transcription_error502: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranscriptionError503")]
    pub create_transcription_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranscriptionRequest")]
    pub create_transcription_request: CreateTranscriptionRequest,
    #[serde(rename = "CreateTranscriptionResponse")]
    pub create_transcription_response: CreateTranscriptionResponse,
    #[serde(rename = "CreateTranscriptionStreamResponse")]
    pub create_transcription_stream_response: CreateTranscriptionResponseStreamEvent,
    #[serde(rename = "CreateTranslationError400")]
    pub create_translation_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranslationError401")]
    pub create_translation_error401: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranslationError413")]
    pub create_translation_error413: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranslationError429")]
    pub create_translation_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranslationError500")]
    pub create_translation_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranslationError502")]
    pub create_translation_error502: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranslationError503")]
    pub create_translation_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateTranslationRequest")]
    pub create_translation_request: CreateTranslationRequest,
    #[serde(rename = "CreateTranslationResponse")]
    pub create_translation_response: CreateTranslationResponse,
    #[serde(rename = "CreateVoiceConsentError400")]
    pub create_voice_consent_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVoiceConsentError404")]
    pub create_voice_consent_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVoiceConsentRequest")]
    pub create_voice_consent_request: CreateVoiceConsentRequest,
    #[serde(rename = "CreateVoiceConsentResponse")]
    pub create_voice_consent_response: VoiceConsentResource,
    #[serde(rename = "CreateVoiceError400")]
    pub create_voice_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVoiceError404")]
    pub create_voice_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVoiceError500")]
    pub create_voice_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVoiceRequest")]
    pub create_voice_request: CreateVoiceRequest,
    #[serde(rename = "CreateVoiceResponse")]
    pub create_voice_response: VoiceResource,
    #[serde(rename = "DeleteVoiceConsentError400")]
    pub delete_voice_consent_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVoiceConsentError404")]
    pub delete_voice_consent_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVoiceConsentParams")]
    pub delete_voice_consent_params: DeleteVoiceConsentParams,
    #[serde(rename = "DeleteVoiceConsentResponse")]
    pub delete_voice_consent_response: VoiceConsentDeletedResource,
    #[serde(rename = "GetVoiceConsentError400")]
    pub get_voice_consent_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVoiceConsentError404")]
    pub get_voice_consent_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVoiceConsentParams")]
    pub get_voice_consent_params: GetVoiceConsentParams,
    #[serde(rename = "GetVoiceConsentResponse")]
    pub get_voice_consent_response: VoiceConsentResource,
    #[serde(rename = "ListVoiceConsentsError400")]
    pub list_voice_consents_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListVoiceConsentsError404")]
    pub list_voice_consents_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListVoiceConsentsParams")]
    pub list_voice_consents_params: ListVoiceConsentsParams,
    #[serde(rename = "ListVoiceConsentsResponse")]
    pub list_voice_consents_response: VoiceConsentListResource,
    #[serde(rename = "UpdateVoiceConsentError400")]
    pub update_voice_consent_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateVoiceConsentError404")]
    pub update_voice_consent_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateVoiceConsentParams")]
    pub update_voice_consent_params: UpdateVoiceConsentParams,
    #[serde(rename = "UpdateVoiceConsentRequest")]
    pub update_voice_consent_request: UpdateVoiceConsentRequest,
    #[serde(rename = "UpdateVoiceConsentResponse")]
    pub update_voice_consent_response: VoiceConsentResource,
}
impl AudioApiSchema {
    pub fn builder() -> builder::AudioApiSchema {
        Default::default()
    }
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
    pub voice: ::openai_api_types::common::VoiceIdsOrCustomVoice,
}
impl CreateSpeechRequest {
    pub fn builder() -> builder::CreateSpeechRequest {
        Default::default()
    }
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
impl CreateTranscriptionRequest {
    pub fn builder() -> builder::CreateTranscriptionRequest {
        Default::default()
    }
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
impl CreateTranscriptionResponseDiarizedJson {
    pub fn builder() -> builder::CreateTranscriptionResponseDiarizedJson {
        Default::default()
    }
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
    Variant0(::openai_api_types::common::TranscriptTextUsageTokens),
    Variant1(::openai_api_types::common::TranscriptTextUsageDuration),
}
impl ::std::convert::From<::openai_api_types::common::TranscriptTextUsageTokens>
    for CreateTranscriptionResponseDiarizedJsonUsage
{
    fn from(value: ::openai_api_types::common::TranscriptTextUsageTokens) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::TranscriptTextUsageDuration>
    for CreateTranscriptionResponseDiarizedJsonUsage
{
    fn from(value: ::openai_api_types::common::TranscriptTextUsageDuration) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "Represents a transcription response returned by model, based on the provided input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateTranscriptionResponseJson {
    #[doc = "The languages detected in the audio. Returned by `gpt-transcribe`. An empty array indicates that no language could be reliably detected.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<::openai_api_types::common::TranscriptionLanguage>,
    #[doc = "The log probabilities of the tokens in the transcription. Only returned with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub logprobs: ::std::vec::Vec<CreateTranscriptionResponseJsonLogprobsItem>,
    #[doc = "The transcribed text."]
    pub text: ::std::string::String,
    #[doc = "Token usage statistics for the request."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<CreateTranscriptionResponseJsonUsage>,
}
impl CreateTranscriptionResponseJson {
    pub fn builder() -> builder::CreateTranscriptionResponseJson {
        Default::default()
    }
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
impl CreateTranscriptionResponseJsonLogprobsItem {
    pub fn builder() -> builder::CreateTranscriptionResponseJsonLogprobsItem {
        Default::default()
    }
}
#[doc = "Token usage statistics for the request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateTranscriptionResponseJsonUsage {
    Variant0(::openai_api_types::common::TranscriptTextUsageTokens),
    Variant1(::openai_api_types::common::TranscriptTextUsageDuration),
}
impl ::std::convert::From<::openai_api_types::common::TranscriptTextUsageTokens>
    for CreateTranscriptionResponseJsonUsage
{
    fn from(value: ::openai_api_types::common::TranscriptTextUsageTokens) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::TranscriptTextUsageDuration>
    for CreateTranscriptionResponseJsonUsage
{
    fn from(value: ::openai_api_types::common::TranscriptTextUsageDuration) -> Self {
        Self::Variant1(value)
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
    pub usage: ::std::option::Option<::openai_api_types::common::TranscriptTextUsageDuration>,
    #[doc = "Extracted words and their corresponding timestamps."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub words: ::std::vec::Vec<TranscriptionWord>,
}
impl CreateTranscriptionResponseVerboseJson {
    pub fn builder() -> builder::CreateTranscriptionResponseVerboseJson {
        Default::default()
    }
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
impl CreateTranslationRequest {
    pub fn builder() -> builder::CreateTranslationRequest {
        Default::default()
    }
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
impl CreateTranslationResponseJson {
    pub fn builder() -> builder::CreateTranslationResponseJson {
        Default::default()
    }
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
impl CreateTranslationResponseVerboseJson {
    pub fn builder() -> builder::CreateTranslationResponseVerboseJson {
        Default::default()
    }
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
impl CreateVoiceConsentRequest {
    pub fn builder() -> builder::CreateVoiceConsentRequest {
        Default::default()
    }
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
impl CreateVoiceFromConsentRequest {
    pub fn builder() -> builder::CreateVoiceFromConsentRequest {
        Default::default()
    }
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
#[doc = "`DeleteVoiceConsentParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteVoiceConsentParams {
    pub consent_id: ::std::string::String,
}
impl DeleteVoiceConsentParams {
    pub fn builder() -> builder::DeleteVoiceConsentParams {
        Default::default()
    }
}
#[doc = "`GetVoiceConsentParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetVoiceConsentParams {
    pub consent_id: ::std::string::String,
}
impl GetVoiceConsentParams {
    pub fn builder() -> builder::GetVoiceConsentParams {
        Default::default()
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
impl ListVoiceConsentsParams {
    pub fn builder() -> builder::ListVoiceConsentsParams {
        Default::default()
    }
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
impl SpeechAudioDeltaEvent {
    pub fn builder() -> builder::SpeechAudioDeltaEvent {
        Default::default()
    }
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
impl SpeechAudioDoneEvent {
    pub fn builder() -> builder::SpeechAudioDoneEvent {
        Default::default()
    }
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
impl SpeechAudioDoneEventUsage {
    pub fn builder() -> builder::SpeechAudioDoneEventUsage {
        Default::default()
    }
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
impl TranscriptTextDeltaEvent {
    pub fn builder() -> builder::TranscriptTextDeltaEvent {
        Default::default()
    }
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
impl TranscriptTextDeltaEventLogprobsItem {
    pub fn builder() -> builder::TranscriptTextDeltaEventLogprobsItem {
        Default::default()
    }
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
    pub languages: ::std::vec::Vec<::openai_api_types::common::TranscriptionLanguage>,
    #[doc = "The log probabilities of the individual tokens in the transcription. Only included if you [create a transcription](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) with the `include[]` parameter set to `logprobs`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub logprobs: ::std::vec::Vec<TranscriptTextDoneEventLogprobsItem>,
    #[doc = "The text that was transcribed.\n"]
    pub text: ::std::string::String,
    #[doc = "The type of the event. Always `transcript.text.done`.\n"]
    #[serde(rename = "type")]
    pub type_: TranscriptTextDoneEventType,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<::openai_api_types::common::TranscriptTextUsageTokens>,
}
impl TranscriptTextDoneEvent {
    pub fn builder() -> builder::TranscriptTextDoneEvent {
        Default::default()
    }
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
impl TranscriptTextDoneEventLogprobsItem {
    pub fn builder() -> builder::TranscriptTextDoneEventLogprobsItem {
        Default::default()
    }
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
impl TranscriptTextSegmentEvent {
    pub fn builder() -> builder::TranscriptTextSegmentEvent {
        Default::default()
    }
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
impl TranscriptionDiarizedSegment {
    pub fn builder() -> builder::TranscriptionDiarizedSegment {
        Default::default()
    }
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
impl TranscriptionSegment {
    pub fn builder() -> builder::TranscriptionSegment {
        Default::default()
    }
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
impl TranscriptionWord {
    pub fn builder() -> builder::TranscriptionWord {
        Default::default()
    }
}
#[doc = "`UpdateVoiceConsentParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateVoiceConsentParams {
    pub consent_id: ::std::string::String,
}
impl UpdateVoiceConsentParams {
    pub fn builder() -> builder::UpdateVoiceConsentParams {
        Default::default()
    }
}
#[doc = "`UpdateVoiceConsentRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UpdateVoiceConsentRequest {
    #[doc = "The updated label for this consent recording."]
    pub name: ::std::string::String,
}
impl UpdateVoiceConsentRequest {
    pub fn builder() -> builder::UpdateVoiceConsentRequest {
        Default::default()
    }
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
impl VadConfig {
    pub fn builder() -> builder::VadConfig {
        Default::default()
    }
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
#[doc = "`VoiceConsentDeletedResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VoiceConsentDeletedResource {
    pub deleted: bool,
    #[doc = "The consent recording identifier."]
    pub id: ::std::string::String,
    pub object: VoiceConsentDeletedResourceObject,
}
impl VoiceConsentDeletedResource {
    pub fn builder() -> builder::VoiceConsentDeletedResource {
        Default::default()
    }
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
impl VoiceConsentListResource {
    pub fn builder() -> builder::VoiceConsentListResource {
        Default::default()
    }
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
impl VoiceConsentResource {
    pub fn builder() -> builder::VoiceConsentResource {
        Default::default()
    }
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
impl VoiceResource {
    pub fn builder() -> builder::VoiceResource {
        Default::default()
    }
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
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AudioApiSchema {
        create_speech_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_speech_error401:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_speech_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_speech_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_speech_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_speech_request:
            ::std::result::Result<super::CreateSpeechRequest, ::std::string::String>,
        create_speech_stream_response:
            ::std::result::Result<super::CreateSpeechResponseStreamEvent, ::std::string::String>,
        create_transcription_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_transcription_error401:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_transcription_error413:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_transcription_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_transcription_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_transcription_error502:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_transcription_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_transcription_request:
            ::std::result::Result<super::CreateTranscriptionRequest, ::std::string::String>,
        create_transcription_response:
            ::std::result::Result<super::CreateTranscriptionResponse, ::std::string::String>,
        create_transcription_stream_response: ::std::result::Result<
            super::CreateTranscriptionResponseStreamEvent,
            ::std::string::String,
        >,
        create_translation_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_translation_error401:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_translation_error413:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_translation_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_translation_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_translation_error502:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_translation_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_translation_request:
            ::std::result::Result<super::CreateTranslationRequest, ::std::string::String>,
        create_translation_response:
            ::std::result::Result<super::CreateTranslationResponse, ::std::string::String>,
        create_voice_consent_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_voice_consent_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_voice_consent_request:
            ::std::result::Result<super::CreateVoiceConsentRequest, ::std::string::String>,
        create_voice_consent_response:
            ::std::result::Result<super::VoiceConsentResource, ::std::string::String>,
        create_voice_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_voice_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_voice_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_voice_request:
            ::std::result::Result<super::CreateVoiceRequest, ::std::string::String>,
        create_voice_response: ::std::result::Result<super::VoiceResource, ::std::string::String>,
        delete_voice_consent_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_voice_consent_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_voice_consent_params:
            ::std::result::Result<super::DeleteVoiceConsentParams, ::std::string::String>,
        delete_voice_consent_response:
            ::std::result::Result<super::VoiceConsentDeletedResource, ::std::string::String>,
        get_voice_consent_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_voice_consent_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_voice_consent_params:
            ::std::result::Result<super::GetVoiceConsentParams, ::std::string::String>,
        get_voice_consent_response:
            ::std::result::Result<super::VoiceConsentResource, ::std::string::String>,
        list_voice_consents_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_voice_consents_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_voice_consents_params:
            ::std::result::Result<super::ListVoiceConsentsParams, ::std::string::String>,
        list_voice_consents_response:
            ::std::result::Result<super::VoiceConsentListResource, ::std::string::String>,
        update_voice_consent_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_voice_consent_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_voice_consent_params:
            ::std::result::Result<super::UpdateVoiceConsentParams, ::std::string::String>,
        update_voice_consent_request:
            ::std::result::Result<super::UpdateVoiceConsentRequest, ::std::string::String>,
        update_voice_consent_response:
            ::std::result::Result<super::VoiceConsentResource, ::std::string::String>,
    }
    impl ::std::default::Default for AudioApiSchema {
        fn default() -> Self {
            Self {
                create_speech_error400: Err(
                    "no value supplied for create_speech_error400".to_string()
                ),
                create_speech_error401: Err(
                    "no value supplied for create_speech_error401".to_string()
                ),
                create_speech_error429: Err(
                    "no value supplied for create_speech_error429".to_string()
                ),
                create_speech_error500: Err(
                    "no value supplied for create_speech_error500".to_string()
                ),
                create_speech_error503: Err(
                    "no value supplied for create_speech_error503".to_string()
                ),
                create_speech_request: Err(
                    "no value supplied for create_speech_request".to_string()
                ),
                create_speech_stream_response: Err(
                    "no value supplied for create_speech_stream_response".to_string(),
                ),
                create_transcription_error400: Err(
                    "no value supplied for create_transcription_error400".to_string(),
                ),
                create_transcription_error401: Err(
                    "no value supplied for create_transcription_error401".to_string(),
                ),
                create_transcription_error413: Err(
                    "no value supplied for create_transcription_error413".to_string(),
                ),
                create_transcription_error429: Err(
                    "no value supplied for create_transcription_error429".to_string(),
                ),
                create_transcription_error500: Err(
                    "no value supplied for create_transcription_error500".to_string(),
                ),
                create_transcription_error502: Err(
                    "no value supplied for create_transcription_error502".to_string(),
                ),
                create_transcription_error503: Err(
                    "no value supplied for create_transcription_error503".to_string(),
                ),
                create_transcription_request: Err(
                    "no value supplied for create_transcription_request".to_string(),
                ),
                create_transcription_response: Err(
                    "no value supplied for create_transcription_response".to_string(),
                ),
                create_transcription_stream_response: Err(
                    "no value supplied for create_transcription_stream_response".to_string(),
                ),
                create_translation_error400: Err(
                    "no value supplied for create_translation_error400".to_string(),
                ),
                create_translation_error401: Err(
                    "no value supplied for create_translation_error401".to_string(),
                ),
                create_translation_error413: Err(
                    "no value supplied for create_translation_error413".to_string(),
                ),
                create_translation_error429: Err(
                    "no value supplied for create_translation_error429".to_string(),
                ),
                create_translation_error500: Err(
                    "no value supplied for create_translation_error500".to_string(),
                ),
                create_translation_error502: Err(
                    "no value supplied for create_translation_error502".to_string(),
                ),
                create_translation_error503: Err(
                    "no value supplied for create_translation_error503".to_string(),
                ),
                create_translation_request: Err(
                    "no value supplied for create_translation_request".to_string()
                ),
                create_translation_response: Err(
                    "no value supplied for create_translation_response".to_string(),
                ),
                create_voice_consent_error400: Err(
                    "no value supplied for create_voice_consent_error400".to_string(),
                ),
                create_voice_consent_error404: Err(
                    "no value supplied for create_voice_consent_error404".to_string(),
                ),
                create_voice_consent_request: Err(
                    "no value supplied for create_voice_consent_request".to_string(),
                ),
                create_voice_consent_response: Err(
                    "no value supplied for create_voice_consent_response".to_string(),
                ),
                create_voice_error400: Err(
                    "no value supplied for create_voice_error400".to_string()
                ),
                create_voice_error404: Err(
                    "no value supplied for create_voice_error404".to_string()
                ),
                create_voice_error500: Err(
                    "no value supplied for create_voice_error500".to_string()
                ),
                create_voice_request: Err("no value supplied for create_voice_request".to_string()),
                create_voice_response: Err(
                    "no value supplied for create_voice_response".to_string()
                ),
                delete_voice_consent_error400: Err(
                    "no value supplied for delete_voice_consent_error400".to_string(),
                ),
                delete_voice_consent_error404: Err(
                    "no value supplied for delete_voice_consent_error404".to_string(),
                ),
                delete_voice_consent_params: Err(
                    "no value supplied for delete_voice_consent_params".to_string(),
                ),
                delete_voice_consent_response: Err(
                    "no value supplied for delete_voice_consent_response".to_string(),
                ),
                get_voice_consent_error400: Err(
                    "no value supplied for get_voice_consent_error400".to_string()
                ),
                get_voice_consent_error404: Err(
                    "no value supplied for get_voice_consent_error404".to_string()
                ),
                get_voice_consent_params: Err(
                    "no value supplied for get_voice_consent_params".to_string()
                ),
                get_voice_consent_response: Err(
                    "no value supplied for get_voice_consent_response".to_string()
                ),
                list_voice_consents_error400: Err(
                    "no value supplied for list_voice_consents_error400".to_string(),
                ),
                list_voice_consents_error404: Err(
                    "no value supplied for list_voice_consents_error404".to_string(),
                ),
                list_voice_consents_params: Err(
                    "no value supplied for list_voice_consents_params".to_string()
                ),
                list_voice_consents_response: Err(
                    "no value supplied for list_voice_consents_response".to_string(),
                ),
                update_voice_consent_error400: Err(
                    "no value supplied for update_voice_consent_error400".to_string(),
                ),
                update_voice_consent_error404: Err(
                    "no value supplied for update_voice_consent_error404".to_string(),
                ),
                update_voice_consent_params: Err(
                    "no value supplied for update_voice_consent_params".to_string(),
                ),
                update_voice_consent_request: Err(
                    "no value supplied for update_voice_consent_request".to_string(),
                ),
                update_voice_consent_response: Err(
                    "no value supplied for update_voice_consent_response".to_string(),
                ),
            }
        }
    }
    impl AudioApiSchema {
        pub fn create_speech_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_speech_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_speech_error400: {e}")
            });
            self
        }
        pub fn create_speech_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_speech_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_speech_error401: {e}")
            });
            self
        }
        pub fn create_speech_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_speech_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_speech_error429: {e}")
            });
            self
        }
        pub fn create_speech_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_speech_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_speech_error500: {e}")
            });
            self
        }
        pub fn create_speech_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_speech_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_speech_error503: {e}")
            });
            self
        }
        pub fn create_speech_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSpeechRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_speech_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_speech_request: {e}")
            });
            self
        }
        pub fn create_speech_stream_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSpeechResponseStreamEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.create_speech_stream_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_speech_stream_response: {e}")
            });
            self
        }
        pub fn create_transcription_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_transcription_error400: {e}")
            });
            self
        }
        pub fn create_transcription_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_transcription_error401: {e}")
            });
            self
        }
        pub fn create_transcription_error413<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_error413 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_transcription_error413: {e}")
            });
            self
        }
        pub fn create_transcription_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_transcription_error429: {e}")
            });
            self
        }
        pub fn create_transcription_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_transcription_error500: {e}")
            });
            self
        }
        pub fn create_transcription_error502<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_error502 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_transcription_error502: {e}")
            });
            self
        }
        pub fn create_transcription_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_transcription_error503: {e}")
            });
            self
        }
        pub fn create_transcription_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateTranscriptionRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_transcription_request: {e}")
            });
            self
        }
        pub fn create_transcription_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateTranscriptionResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_transcription_response: {e}")
            });
            self
        }
        pub fn create_transcription_stream_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateTranscriptionResponseStreamEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.create_transcription_stream_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_transcription_stream_response: {e}"
                )
            });
            self
        }
        pub fn create_translation_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_translation_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_translation_error400: {e}")
            });
            self
        }
        pub fn create_translation_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_translation_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_translation_error401: {e}")
            });
            self
        }
        pub fn create_translation_error413<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_translation_error413 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_translation_error413: {e}")
            });
            self
        }
        pub fn create_translation_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_translation_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_translation_error429: {e}")
            });
            self
        }
        pub fn create_translation_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_translation_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_translation_error500: {e}")
            });
            self
        }
        pub fn create_translation_error502<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_translation_error502 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_translation_error502: {e}")
            });
            self
        }
        pub fn create_translation_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_translation_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_translation_error503: {e}")
            });
            self
        }
        pub fn create_translation_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateTranslationRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_translation_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_translation_request: {e}")
            });
            self
        }
        pub fn create_translation_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateTranslationResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_translation_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_translation_response: {e}")
            });
            self
        }
        pub fn create_voice_consent_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_voice_consent_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_voice_consent_error400: {e}")
            });
            self
        }
        pub fn create_voice_consent_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_voice_consent_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_voice_consent_error404: {e}")
            });
            self
        }
        pub fn create_voice_consent_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVoiceConsentRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_voice_consent_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_voice_consent_request: {e}")
            });
            self
        }
        pub fn create_voice_consent_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceConsentResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_voice_consent_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_voice_consent_response: {e}")
            });
            self
        }
        pub fn create_voice_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_voice_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_voice_error400: {e}")
            });
            self
        }
        pub fn create_voice_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_voice_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_voice_error404: {e}")
            });
            self
        }
        pub fn create_voice_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_voice_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_voice_error500: {e}")
            });
            self
        }
        pub fn create_voice_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVoiceRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_voice_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_voice_request: {e}")
            });
            self
        }
        pub fn create_voice_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_voice_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_voice_response: {e}")
            });
            self
        }
        pub fn delete_voice_consent_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_voice_consent_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_voice_consent_error400: {e}")
            });
            self
        }
        pub fn delete_voice_consent_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_voice_consent_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_voice_consent_error404: {e}")
            });
            self
        }
        pub fn delete_voice_consent_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVoiceConsentParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_voice_consent_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_voice_consent_params: {e}")
            });
            self
        }
        pub fn delete_voice_consent_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceConsentDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_voice_consent_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_voice_consent_response: {e}")
            });
            self
        }
        pub fn get_voice_consent_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_voice_consent_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_voice_consent_error400: {e}")
            });
            self
        }
        pub fn get_voice_consent_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_voice_consent_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_voice_consent_error404: {e}")
            });
            self
        }
        pub fn get_voice_consent_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetVoiceConsentParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_voice_consent_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_voice_consent_params: {e}")
            });
            self
        }
        pub fn get_voice_consent_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceConsentResource>,
            T::Error: ::std::fmt::Display,
        {
            self.get_voice_consent_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_voice_consent_response: {e}")
            });
            self
        }
        pub fn list_voice_consents_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_voice_consents_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_voice_consents_error400: {e}")
            });
            self
        }
        pub fn list_voice_consents_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_voice_consents_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_voice_consents_error404: {e}")
            });
            self
        }
        pub fn list_voice_consents_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVoiceConsentsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_voice_consents_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_voice_consents_params: {e}")
            });
            self
        }
        pub fn list_voice_consents_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceConsentListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_voice_consents_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_voice_consents_response: {e}")
            });
            self
        }
        pub fn update_voice_consent_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_voice_consent_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_voice_consent_error400: {e}")
            });
            self
        }
        pub fn update_voice_consent_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_voice_consent_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_voice_consent_error404: {e}")
            });
            self
        }
        pub fn update_voice_consent_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateVoiceConsentParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_voice_consent_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_voice_consent_params: {e}")
            });
            self
        }
        pub fn update_voice_consent_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateVoiceConsentRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.update_voice_consent_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_voice_consent_request: {e}")
            });
            self
        }
        pub fn update_voice_consent_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceConsentResource>,
            T::Error: ::std::fmt::Display,
        {
            self.update_voice_consent_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_voice_consent_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioApiSchema> for super::AudioApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_speech_error400: value.create_speech_error400?,
                create_speech_error401: value.create_speech_error401?,
                create_speech_error429: value.create_speech_error429?,
                create_speech_error500: value.create_speech_error500?,
                create_speech_error503: value.create_speech_error503?,
                create_speech_request: value.create_speech_request?,
                create_speech_stream_response: value.create_speech_stream_response?,
                create_transcription_error400: value.create_transcription_error400?,
                create_transcription_error401: value.create_transcription_error401?,
                create_transcription_error413: value.create_transcription_error413?,
                create_transcription_error429: value.create_transcription_error429?,
                create_transcription_error500: value.create_transcription_error500?,
                create_transcription_error502: value.create_transcription_error502?,
                create_transcription_error503: value.create_transcription_error503?,
                create_transcription_request: value.create_transcription_request?,
                create_transcription_response: value.create_transcription_response?,
                create_transcription_stream_response: value.create_transcription_stream_response?,
                create_translation_error400: value.create_translation_error400?,
                create_translation_error401: value.create_translation_error401?,
                create_translation_error413: value.create_translation_error413?,
                create_translation_error429: value.create_translation_error429?,
                create_translation_error500: value.create_translation_error500?,
                create_translation_error502: value.create_translation_error502?,
                create_translation_error503: value.create_translation_error503?,
                create_translation_request: value.create_translation_request?,
                create_translation_response: value.create_translation_response?,
                create_voice_consent_error400: value.create_voice_consent_error400?,
                create_voice_consent_error404: value.create_voice_consent_error404?,
                create_voice_consent_request: value.create_voice_consent_request?,
                create_voice_consent_response: value.create_voice_consent_response?,
                create_voice_error400: value.create_voice_error400?,
                create_voice_error404: value.create_voice_error404?,
                create_voice_error500: value.create_voice_error500?,
                create_voice_request: value.create_voice_request?,
                create_voice_response: value.create_voice_response?,
                delete_voice_consent_error400: value.delete_voice_consent_error400?,
                delete_voice_consent_error404: value.delete_voice_consent_error404?,
                delete_voice_consent_params: value.delete_voice_consent_params?,
                delete_voice_consent_response: value.delete_voice_consent_response?,
                get_voice_consent_error400: value.get_voice_consent_error400?,
                get_voice_consent_error404: value.get_voice_consent_error404?,
                get_voice_consent_params: value.get_voice_consent_params?,
                get_voice_consent_response: value.get_voice_consent_response?,
                list_voice_consents_error400: value.list_voice_consents_error400?,
                list_voice_consents_error404: value.list_voice_consents_error404?,
                list_voice_consents_params: value.list_voice_consents_params?,
                list_voice_consents_response: value.list_voice_consents_response?,
                update_voice_consent_error400: value.update_voice_consent_error400?,
                update_voice_consent_error404: value.update_voice_consent_error404?,
                update_voice_consent_params: value.update_voice_consent_params?,
                update_voice_consent_request: value.update_voice_consent_request?,
                update_voice_consent_response: value.update_voice_consent_response?,
            })
        }
    }
    impl ::std::convert::From<super::AudioApiSchema> for AudioApiSchema {
        fn from(value: super::AudioApiSchema) -> Self {
            Self {
                create_speech_error400: Ok(value.create_speech_error400),
                create_speech_error401: Ok(value.create_speech_error401),
                create_speech_error429: Ok(value.create_speech_error429),
                create_speech_error500: Ok(value.create_speech_error500),
                create_speech_error503: Ok(value.create_speech_error503),
                create_speech_request: Ok(value.create_speech_request),
                create_speech_stream_response: Ok(value.create_speech_stream_response),
                create_transcription_error400: Ok(value.create_transcription_error400),
                create_transcription_error401: Ok(value.create_transcription_error401),
                create_transcription_error413: Ok(value.create_transcription_error413),
                create_transcription_error429: Ok(value.create_transcription_error429),
                create_transcription_error500: Ok(value.create_transcription_error500),
                create_transcription_error502: Ok(value.create_transcription_error502),
                create_transcription_error503: Ok(value.create_transcription_error503),
                create_transcription_request: Ok(value.create_transcription_request),
                create_transcription_response: Ok(value.create_transcription_response),
                create_transcription_stream_response: Ok(value.create_transcription_stream_response),
                create_translation_error400: Ok(value.create_translation_error400),
                create_translation_error401: Ok(value.create_translation_error401),
                create_translation_error413: Ok(value.create_translation_error413),
                create_translation_error429: Ok(value.create_translation_error429),
                create_translation_error500: Ok(value.create_translation_error500),
                create_translation_error502: Ok(value.create_translation_error502),
                create_translation_error503: Ok(value.create_translation_error503),
                create_translation_request: Ok(value.create_translation_request),
                create_translation_response: Ok(value.create_translation_response),
                create_voice_consent_error400: Ok(value.create_voice_consent_error400),
                create_voice_consent_error404: Ok(value.create_voice_consent_error404),
                create_voice_consent_request: Ok(value.create_voice_consent_request),
                create_voice_consent_response: Ok(value.create_voice_consent_response),
                create_voice_error400: Ok(value.create_voice_error400),
                create_voice_error404: Ok(value.create_voice_error404),
                create_voice_error500: Ok(value.create_voice_error500),
                create_voice_request: Ok(value.create_voice_request),
                create_voice_response: Ok(value.create_voice_response),
                delete_voice_consent_error400: Ok(value.delete_voice_consent_error400),
                delete_voice_consent_error404: Ok(value.delete_voice_consent_error404),
                delete_voice_consent_params: Ok(value.delete_voice_consent_params),
                delete_voice_consent_response: Ok(value.delete_voice_consent_response),
                get_voice_consent_error400: Ok(value.get_voice_consent_error400),
                get_voice_consent_error404: Ok(value.get_voice_consent_error404),
                get_voice_consent_params: Ok(value.get_voice_consent_params),
                get_voice_consent_response: Ok(value.get_voice_consent_response),
                list_voice_consents_error400: Ok(value.list_voice_consents_error400),
                list_voice_consents_error404: Ok(value.list_voice_consents_error404),
                list_voice_consents_params: Ok(value.list_voice_consents_params),
                list_voice_consents_response: Ok(value.list_voice_consents_response),
                update_voice_consent_error400: Ok(value.update_voice_consent_error400),
                update_voice_consent_error404: Ok(value.update_voice_consent_error404),
                update_voice_consent_params: Ok(value.update_voice_consent_params),
                update_voice_consent_request: Ok(value.update_voice_consent_request),
                update_voice_consent_response: Ok(value.update_voice_consent_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateSpeechRequest {
        input: ::std::result::Result<super::CreateSpeechRequestInput, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<super::CreateSpeechRequestInstructions>,
            ::std::string::String,
        >,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        response_format: ::std::result::Result<
            ::std::option::Option<super::CreateSpeechRequestResponseFormat>,
            ::std::string::String,
        >,
        speed: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stream_format: ::std::result::Result<
            ::std::option::Option<super::CreateSpeechRequestStreamFormat>,
            ::std::string::String,
        >,
        voice: ::std::result::Result<
            ::openai_api_types::common::VoiceIdsOrCustomVoice,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateSpeechRequest {
        fn default() -> Self {
            Self {
                input: Err("no value supplied for input".to_string()),
                instructions: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                response_format: Ok(Default::default()),
                speed: Ok(Default::default()),
                stream_format: Ok(Default::default()),
                voice: Err("no value supplied for voice".to_string()),
            }
        }
    }
    impl CreateSpeechRequest {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSpeechRequestInput>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateSpeechRequestInstructions>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn response_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateSpeechRequestResponseFormat>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speed: {e}"));
            self
        }
        pub fn stream_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateSpeechRequestStreamFormat>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.stream_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stream_format: {e}"));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::VoiceIdsOrCustomVoice>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateSpeechRequest> for super::CreateSpeechRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateSpeechRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                instructions: value.instructions?,
                model: value.model?,
                response_format: value.response_format?,
                speed: value.speed?,
                stream_format: value.stream_format?,
                voice: value.voice?,
            })
        }
    }
    impl ::std::convert::From<super::CreateSpeechRequest> for CreateSpeechRequest {
        fn from(value: super::CreateSpeechRequest) -> Self {
            Self {
                input: Ok(value.input),
                instructions: Ok(value.instructions),
                model: Ok(value.model),
                response_format: Ok(value.response_format),
                speed: Ok(value.speed),
                stream_format: Ok(value.stream_format),
                voice: Ok(value.voice),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateTranscriptionRequest {
        chunking_strategy:
            ::std::result::Result<::std::option::Option<super::VadConfig>, ::std::string::String>,
        file: ::std::result::Result<::std::string::String, ::std::string::String>,
        include: ::std::result::Result<
            ::std::vec::Vec<super::TranscriptionInclude>,
            ::std::string::String,
        >,
        keywords:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        known_speaker_names:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        known_speaker_references:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        language: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        languages:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        prompt: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        response_format: ::std::result::Result<
            ::std::option::Option<super::AudioResponseFormat>,
            ::std::string::String,
        >,
        stream: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        timestamp_granularities: ::std::result::Result<
            ::std::vec::Vec<super::CreateTranscriptionRequestTimestampGranularitiesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateTranscriptionRequest {
        fn default() -> Self {
            Self {
                chunking_strategy: Ok(Default::default()),
                file: Err("no value supplied for file".to_string()),
                include: Ok(Default::default()),
                keywords: Ok(Default::default()),
                known_speaker_names: Ok(Default::default()),
                known_speaker_references: Ok(Default::default()),
                language: Ok(Default::default()),
                languages: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                prompt: Ok(Default::default()),
                response_format: Ok(Default::default()),
                stream: Ok(Default::default()),
                temperature: Ok(Default::default()),
                timestamp_granularities: Ok(Default::default()),
            }
        }
    }
    impl CreateTranscriptionRequest {
        pub fn chunking_strategy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VadConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.chunking_strategy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunking_strategy: {e}"));
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {e}"));
            self
        }
        pub fn include<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TranscriptionInclude>>,
            T::Error: ::std::fmt::Display,
        {
            self.include = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include: {e}"));
            self
        }
        pub fn keywords<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.keywords = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keywords: {e}"));
            self
        }
        pub fn known_speaker_names<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.known_speaker_names = value.try_into().map_err(|e| {
                format!("error converting supplied value for known_speaker_names: {e}")
            });
            self
        }
        pub fn known_speaker_references<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.known_speaker_references = value.try_into().map_err(|e| {
                format!("error converting supplied value for known_speaker_references: {e}")
            });
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {e}"));
            self
        }
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.languages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for languages: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
        pub fn response_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AudioResponseFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn stream<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.stream = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stream: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn timestamp_granularities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CreateTranscriptionRequestTimestampGranularitiesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp_granularities = value.try_into().map_err(|e| {
                format!("error converting supplied value for timestamp_granularities: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CreateTranscriptionRequest> for super::CreateTranscriptionRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateTranscriptionRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                chunking_strategy: value.chunking_strategy?,
                file: value.file?,
                include: value.include?,
                keywords: value.keywords?,
                known_speaker_names: value.known_speaker_names?,
                known_speaker_references: value.known_speaker_references?,
                language: value.language?,
                languages: value.languages?,
                model: value.model?,
                prompt: value.prompt?,
                response_format: value.response_format?,
                stream: value.stream?,
                temperature: value.temperature?,
                timestamp_granularities: value.timestamp_granularities?,
            })
        }
    }
    impl ::std::convert::From<super::CreateTranscriptionRequest> for CreateTranscriptionRequest {
        fn from(value: super::CreateTranscriptionRequest) -> Self {
            Self {
                chunking_strategy: Ok(value.chunking_strategy),
                file: Ok(value.file),
                include: Ok(value.include),
                keywords: Ok(value.keywords),
                known_speaker_names: Ok(value.known_speaker_names),
                known_speaker_references: Ok(value.known_speaker_references),
                language: Ok(value.language),
                languages: Ok(value.languages),
                model: Ok(value.model),
                prompt: Ok(value.prompt),
                response_format: Ok(value.response_format),
                stream: Ok(value.stream),
                temperature: Ok(value.temperature),
                timestamp_granularities: Ok(value.timestamp_granularities),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateTranscriptionResponseDiarizedJson {
        duration: ::std::result::Result<f64, ::std::string::String>,
        segments: ::std::result::Result<
            ::std::vec::Vec<super::TranscriptionDiarizedSegment>,
            ::std::string::String,
        >,
        task: ::std::result::Result<
            super::CreateTranscriptionResponseDiarizedJsonTask,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        usage: ::std::result::Result<
            ::std::option::Option<super::CreateTranscriptionResponseDiarizedJsonUsage>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateTranscriptionResponseDiarizedJson {
        fn default() -> Self {
            Self {
                duration: Err("no value supplied for duration".to_string()),
                segments: Err("no value supplied for segments".to_string()),
                task: Err("no value supplied for task".to_string()),
                text: Err("no value supplied for text".to_string()),
                usage: Ok(Default::default()),
            }
        }
    }
    impl CreateTranscriptionResponseDiarizedJson {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {e}"));
            self
        }
        pub fn segments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TranscriptionDiarizedSegment>>,
            T::Error: ::std::fmt::Display,
        {
            self.segments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for segments: {e}"));
            self
        }
        pub fn task<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateTranscriptionResponseDiarizedJsonTask>,
            T::Error: ::std::fmt::Display,
        {
            self.task = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateTranscriptionResponseDiarizedJsonUsage>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateTranscriptionResponseDiarizedJson>
        for super::CreateTranscriptionResponseDiarizedJson
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateTranscriptionResponseDiarizedJson,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                segments: value.segments?,
                task: value.task?,
                text: value.text?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::CreateTranscriptionResponseDiarizedJson>
        for CreateTranscriptionResponseDiarizedJson
    {
        fn from(value: super::CreateTranscriptionResponseDiarizedJson) -> Self {
            Self {
                duration: Ok(value.duration),
                segments: Ok(value.segments),
                task: Ok(value.task),
                text: Ok(value.text),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateTranscriptionResponseJson {
        languages: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::TranscriptionLanguage>,
            ::std::string::String,
        >,
        logprobs: ::std::result::Result<
            ::std::vec::Vec<super::CreateTranscriptionResponseJsonLogprobsItem>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        usage: ::std::result::Result<
            ::std::option::Option<super::CreateTranscriptionResponseJsonUsage>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateTranscriptionResponseJson {
        fn default() -> Self {
            Self {
                languages: Ok(Default::default()),
                logprobs: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                usage: Ok(Default::default()),
            }
        }
    }
    impl CreateTranscriptionResponseJson {
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::openai_api_types::common::TranscriptionLanguage>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.languages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for languages: {e}"));
            self
        }
        pub fn logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CreateTranscriptionResponseJsonLogprobsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprobs: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateTranscriptionResponseJsonUsage>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateTranscriptionResponseJson>
        for super::CreateTranscriptionResponseJson
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateTranscriptionResponseJson,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                languages: value.languages?,
                logprobs: value.logprobs?,
                text: value.text?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::CreateTranscriptionResponseJson>
        for CreateTranscriptionResponseJson
    {
        fn from(value: super::CreateTranscriptionResponseJson) -> Self {
            Self {
                languages: Ok(value.languages),
                logprobs: Ok(value.logprobs),
                text: Ok(value.text),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateTranscriptionResponseJsonLogprobsItem {
        bytes: ::std::result::Result<::std::vec::Vec<f64>, ::std::string::String>,
        logprob: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateTranscriptionResponseJsonLogprobsItem {
        fn default() -> Self {
            Self {
                bytes: Ok(Default::default()),
                logprob: Ok(Default::default()),
                token: Ok(Default::default()),
            }
        }
    }
    impl CreateTranscriptionResponseJsonLogprobsItem {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {e}"));
            self
        }
        pub fn logprob<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.logprob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprob: {e}"));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateTranscriptionResponseJsonLogprobsItem>
        for super::CreateTranscriptionResponseJsonLogprobsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateTranscriptionResponseJsonLogprobsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                logprob: value.logprob?,
                token: value.token?,
            })
        }
    }
    impl ::std::convert::From<super::CreateTranscriptionResponseJsonLogprobsItem>
        for CreateTranscriptionResponseJsonLogprobsItem
    {
        fn from(value: super::CreateTranscriptionResponseJsonLogprobsItem) -> Self {
            Self {
                bytes: Ok(value.bytes),
                logprob: Ok(value.logprob),
                token: Ok(value.token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateTranscriptionResponseVerboseJson {
        duration: ::std::result::Result<f64, ::std::string::String>,
        language: ::std::result::Result<::std::string::String, ::std::string::String>,
        segments: ::std::result::Result<
            ::std::vec::Vec<super::TranscriptionSegment>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        usage: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::TranscriptTextUsageDuration>,
            ::std::string::String,
        >,
        words:
            ::std::result::Result<::std::vec::Vec<super::TranscriptionWord>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateTranscriptionResponseVerboseJson {
        fn default() -> Self {
            Self {
                duration: Err("no value supplied for duration".to_string()),
                language: Err("no value supplied for language".to_string()),
                segments: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                usage: Ok(Default::default()),
                words: Ok(Default::default()),
            }
        }
    }
    impl CreateTranscriptionResponseVerboseJson {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {e}"));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {e}"));
            self
        }
        pub fn segments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TranscriptionSegment>>,
            T::Error: ::std::fmt::Display,
        {
            self.segments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for segments: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::TranscriptTextUsageDuration>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
        pub fn words<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TranscriptionWord>>,
            T::Error: ::std::fmt::Display,
        {
            self.words = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for words: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateTranscriptionResponseVerboseJson>
        for super::CreateTranscriptionResponseVerboseJson
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateTranscriptionResponseVerboseJson,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                language: value.language?,
                segments: value.segments?,
                text: value.text?,
                usage: value.usage?,
                words: value.words?,
            })
        }
    }
    impl ::std::convert::From<super::CreateTranscriptionResponseVerboseJson>
        for CreateTranscriptionResponseVerboseJson
    {
        fn from(value: super::CreateTranscriptionResponseVerboseJson) -> Self {
            Self {
                duration: Ok(value.duration),
                language: Ok(value.language),
                segments: Ok(value.segments),
                text: Ok(value.text),
                usage: Ok(value.usage),
                words: Ok(value.words),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateTranslationRequest {
        file: ::std::result::Result<::std::string::String, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        prompt: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        response_format: ::std::result::Result<
            ::std::option::Option<super::CreateTranslationRequestResponseFormat>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateTranslationRequest {
        fn default() -> Self {
            Self {
                file: Err("no value supplied for file".to_string()),
                model: Err("no value supplied for model".to_string()),
                prompt: Ok(Default::default()),
                response_format: Ok(Default::default()),
                temperature: Ok(Default::default()),
            }
        }
    }
    impl CreateTranslationRequest {
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
        pub fn response_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateTranslationRequestResponseFormat>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateTranslationRequest> for super::CreateTranslationRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateTranslationRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file: value.file?,
                model: value.model?,
                prompt: value.prompt?,
                response_format: value.response_format?,
                temperature: value.temperature?,
            })
        }
    }
    impl ::std::convert::From<super::CreateTranslationRequest> for CreateTranslationRequest {
        fn from(value: super::CreateTranslationRequest) -> Self {
            Self {
                file: Ok(value.file),
                model: Ok(value.model),
                prompt: Ok(value.prompt),
                response_format: Ok(value.response_format),
                temperature: Ok(value.temperature),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateTranslationResponseJson {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateTranslationResponseJson {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl CreateTranslationResponseJson {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateTranslationResponseJson>
        for super::CreateTranslationResponseJson
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateTranslationResponseJson,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { text: value.text? })
        }
    }
    impl ::std::convert::From<super::CreateTranslationResponseJson> for CreateTranslationResponseJson {
        fn from(value: super::CreateTranslationResponseJson) -> Self {
            Self {
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateTranslationResponseVerboseJson {
        duration: ::std::result::Result<f64, ::std::string::String>,
        language: ::std::result::Result<::std::string::String, ::std::string::String>,
        segments: ::std::result::Result<
            ::std::vec::Vec<super::TranscriptionSegment>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateTranslationResponseVerboseJson {
        fn default() -> Self {
            Self {
                duration: Err("no value supplied for duration".to_string()),
                language: Err("no value supplied for language".to_string()),
                segments: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl CreateTranslationResponseVerboseJson {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {e}"));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {e}"));
            self
        }
        pub fn segments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TranscriptionSegment>>,
            T::Error: ::std::fmt::Display,
        {
            self.segments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for segments: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateTranslationResponseVerboseJson>
        for super::CreateTranslationResponseVerboseJson
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateTranslationResponseVerboseJson,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                language: value.language?,
                segments: value.segments?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::CreateTranslationResponseVerboseJson>
        for CreateTranslationResponseVerboseJson
    {
        fn from(value: super::CreateTranslationResponseVerboseJson) -> Self {
            Self {
                duration: Ok(value.duration),
                language: Ok(value.language),
                segments: Ok(value.segments),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVoiceConsentRequest {
        language: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        recording: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVoiceConsentRequest {
        fn default() -> Self {
            Self {
                language: Err("no value supplied for language".to_string()),
                name: Err("no value supplied for name".to_string()),
                recording: Err("no value supplied for recording".to_string()),
            }
        }
    }
    impl CreateVoiceConsentRequest {
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn recording<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recording = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recording: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVoiceConsentRequest> for super::CreateVoiceConsentRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVoiceConsentRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                language: value.language?,
                name: value.name?,
                recording: value.recording?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVoiceConsentRequest> for CreateVoiceConsentRequest {
        fn from(value: super::CreateVoiceConsentRequest) -> Self {
            Self {
                language: Ok(value.language),
                name: Ok(value.name),
                recording: Ok(value.recording),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVoiceFromConsentRequest {
        audio_sample: ::std::result::Result<::std::string::String, ::std::string::String>,
        consent: ::std::result::Result<::std::string::String, ::std::string::String>,
        name:
            ::std::result::Result<super::CreateVoiceFromConsentRequestName, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::CreateVoiceFromConsentRequestType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateVoiceFromConsentRequest {
        fn default() -> Self {
            Self {
                audio_sample: Err("no value supplied for audio_sample".to_string()),
                consent: Err("no value supplied for consent".to_string()),
                name: Err("no value supplied for name".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl CreateVoiceFromConsentRequest {
        pub fn audio_sample<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_sample = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio_sample: {e}"));
            self
        }
        pub fn consent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.consent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for consent: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVoiceFromConsentRequestName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateVoiceFromConsentRequestType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVoiceFromConsentRequest>
        for super::CreateVoiceFromConsentRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVoiceFromConsentRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_sample: value.audio_sample?,
                consent: value.consent?,
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVoiceFromConsentRequest> for CreateVoiceFromConsentRequest {
        fn from(value: super::CreateVoiceFromConsentRequest) -> Self {
            Self {
                audio_sample: Ok(value.audio_sample),
                consent: Ok(value.consent),
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteVoiceConsentParams {
        consent_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteVoiceConsentParams {
        fn default() -> Self {
            Self {
                consent_id: Err("no value supplied for consent_id".to_string()),
            }
        }
    }
    impl DeleteVoiceConsentParams {
        pub fn consent_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.consent_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for consent_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteVoiceConsentParams> for super::DeleteVoiceConsentParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteVoiceConsentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                consent_id: value.consent_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteVoiceConsentParams> for DeleteVoiceConsentParams {
        fn from(value: super::DeleteVoiceConsentParams) -> Self {
            Self {
                consent_id: Ok(value.consent_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetVoiceConsentParams {
        consent_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetVoiceConsentParams {
        fn default() -> Self {
            Self {
                consent_id: Err("no value supplied for consent_id".to_string()),
            }
        }
    }
    impl GetVoiceConsentParams {
        pub fn consent_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.consent_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for consent_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetVoiceConsentParams> for super::GetVoiceConsentParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetVoiceConsentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                consent_id: value.consent_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetVoiceConsentParams> for GetVoiceConsentParams {
        fn from(value: super::GetVoiceConsentParams) -> Self {
            Self {
                consent_id: Ok(value.consent_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListVoiceConsentsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ListVoiceConsentsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
            }
        }
    }
    impl ListVoiceConsentsParams {
        pub fn after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for after: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListVoiceConsentsParams> for super::ListVoiceConsentsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListVoiceConsentsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
            })
        }
    }
    impl ::std::convert::From<super::ListVoiceConsentsParams> for ListVoiceConsentsParams {
        fn from(value: super::ListVoiceConsentsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpeechAudioDeltaEvent {
        audio: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::SpeechAudioDeltaEventType, ::std::string::String>,
    }
    impl ::std::default::Default for SpeechAudioDeltaEvent {
        fn default() -> Self {
            Self {
                audio: Err("no value supplied for audio".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SpeechAudioDeltaEvent {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpeechAudioDeltaEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SpeechAudioDeltaEvent> for super::SpeechAudioDeltaEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpeechAudioDeltaEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SpeechAudioDeltaEvent> for SpeechAudioDeltaEvent {
        fn from(value: super::SpeechAudioDeltaEvent) -> Self {
            Self {
                audio: Ok(value.audio),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpeechAudioDoneEvent {
        type_: ::std::result::Result<super::SpeechAudioDoneEventType, ::std::string::String>,
        usage: ::std::result::Result<super::SpeechAudioDoneEventUsage, ::std::string::String>,
    }
    impl ::std::default::Default for SpeechAudioDoneEvent {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                usage: Err("no value supplied for usage".to_string()),
            }
        }
    }
    impl SpeechAudioDoneEvent {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpeechAudioDoneEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpeechAudioDoneEventUsage>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SpeechAudioDoneEvent> for super::SpeechAudioDoneEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpeechAudioDoneEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::SpeechAudioDoneEvent> for SpeechAudioDoneEvent {
        fn from(value: super::SpeechAudioDoneEvent) -> Self {
            Self {
                type_: Ok(value.type_),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpeechAudioDoneEventUsage {
        input_tokens: ::std::result::Result<i64, ::std::string::String>,
        output_tokens: ::std::result::Result<i64, ::std::string::String>,
        total_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for SpeechAudioDoneEventUsage {
        fn default() -> Self {
            Self {
                input_tokens: Err("no value supplied for input_tokens".to_string()),
                output_tokens: Err("no value supplied for output_tokens".to_string()),
                total_tokens: Err("no value supplied for total_tokens".to_string()),
            }
        }
    }
    impl SpeechAudioDoneEventUsage {
        pub fn input_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.input_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_tokens: {e}"));
            self
        }
        pub fn output_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.output_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_tokens: {e}"));
            self
        }
        pub fn total_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.total_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SpeechAudioDoneEventUsage> for super::SpeechAudioDoneEventUsage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpeechAudioDoneEventUsage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_tokens: value.input_tokens?,
                output_tokens: value.output_tokens?,
                total_tokens: value.total_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::SpeechAudioDoneEventUsage> for SpeechAudioDoneEventUsage {
        fn from(value: super::SpeechAudioDoneEventUsage) -> Self {
            Self {
                input_tokens: Ok(value.input_tokens),
                output_tokens: Ok(value.output_tokens),
                total_tokens: Ok(value.total_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptTextDeltaEvent {
        delta: ::std::result::Result<::std::string::String, ::std::string::String>,
        logprobs: ::std::result::Result<
            ::std::vec::Vec<super::TranscriptTextDeltaEventLogprobsItem>,
            ::std::string::String,
        >,
        segment_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::TranscriptTextDeltaEventType, ::std::string::String>,
    }
    impl ::std::default::Default for TranscriptTextDeltaEvent {
        fn default() -> Self {
            Self {
                delta: Err("no value supplied for delta".to_string()),
                logprobs: Ok(Default::default()),
                segment_id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TranscriptTextDeltaEvent {
        pub fn delta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.delta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delta: {e}"));
            self
        }
        pub fn logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::TranscriptTextDeltaEventLogprobsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprobs: {e}"));
            self
        }
        pub fn segment_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.segment_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for segment_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptTextDeltaEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptTextDeltaEvent> for super::TranscriptTextDeltaEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptTextDeltaEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                delta: value.delta?,
                logprobs: value.logprobs?,
                segment_id: value.segment_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptTextDeltaEvent> for TranscriptTextDeltaEvent {
        fn from(value: super::TranscriptTextDeltaEvent) -> Self {
            Self {
                delta: Ok(value.delta),
                logprobs: Ok(value.logprobs),
                segment_id: Ok(value.segment_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptTextDeltaEventLogprobsItem {
        bytes: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
        logprob: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranscriptTextDeltaEventLogprobsItem {
        fn default() -> Self {
            Self {
                bytes: Ok(Default::default()),
                logprob: Ok(Default::default()),
                token: Ok(Default::default()),
            }
        }
    }
    impl TranscriptTextDeltaEventLogprobsItem {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {e}"));
            self
        }
        pub fn logprob<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.logprob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprob: {e}"));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptTextDeltaEventLogprobsItem>
        for super::TranscriptTextDeltaEventLogprobsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptTextDeltaEventLogprobsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                logprob: value.logprob?,
                token: value.token?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptTextDeltaEventLogprobsItem>
        for TranscriptTextDeltaEventLogprobsItem
    {
        fn from(value: super::TranscriptTextDeltaEventLogprobsItem) -> Self {
            Self {
                bytes: Ok(value.bytes),
                logprob: Ok(value.logprob),
                token: Ok(value.token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptTextDoneEvent {
        languages: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::TranscriptionLanguage>,
            ::std::string::String,
        >,
        logprobs: ::std::result::Result<
            ::std::vec::Vec<super::TranscriptTextDoneEventLogprobsItem>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::TranscriptTextDoneEventType, ::std::string::String>,
        usage: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::TranscriptTextUsageTokens>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranscriptTextDoneEvent {
        fn default() -> Self {
            Self {
                languages: Ok(Default::default()),
                logprobs: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                usage: Ok(Default::default()),
            }
        }
    }
    impl TranscriptTextDoneEvent {
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::openai_api_types::common::TranscriptionLanguage>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.languages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for languages: {e}"));
            self
        }
        pub fn logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TranscriptTextDoneEventLogprobsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprobs: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptTextDoneEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::TranscriptTextUsageTokens>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptTextDoneEvent> for super::TranscriptTextDoneEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptTextDoneEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                languages: value.languages?,
                logprobs: value.logprobs?,
                text: value.text?,
                type_: value.type_?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptTextDoneEvent> for TranscriptTextDoneEvent {
        fn from(value: super::TranscriptTextDoneEvent) -> Self {
            Self {
                languages: Ok(value.languages),
                logprobs: Ok(value.logprobs),
                text: Ok(value.text),
                type_: Ok(value.type_),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptTextDoneEventLogprobsItem {
        bytes: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
        logprob: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranscriptTextDoneEventLogprobsItem {
        fn default() -> Self {
            Self {
                bytes: Ok(Default::default()),
                logprob: Ok(Default::default()),
                token: Ok(Default::default()),
            }
        }
    }
    impl TranscriptTextDoneEventLogprobsItem {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {e}"));
            self
        }
        pub fn logprob<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.logprob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprob: {e}"));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptTextDoneEventLogprobsItem>
        for super::TranscriptTextDoneEventLogprobsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptTextDoneEventLogprobsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                logprob: value.logprob?,
                token: value.token?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptTextDoneEventLogprobsItem>
        for TranscriptTextDoneEventLogprobsItem
    {
        fn from(value: super::TranscriptTextDoneEventLogprobsItem) -> Self {
            Self {
                bytes: Ok(value.bytes),
                logprob: Ok(value.logprob),
                token: Ok(value.token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptTextSegmentEvent {
        end: ::std::result::Result<f64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        speaker: ::std::result::Result<::std::string::String, ::std::string::String>,
        start: ::std::result::Result<f64, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::TranscriptTextSegmentEventType, ::std::string::String>,
    }
    impl ::std::default::Default for TranscriptTextSegmentEvent {
        fn default() -> Self {
            Self {
                end: Err("no value supplied for end".to_string()),
                id: Err("no value supplied for id".to_string()),
                speaker: Err("no value supplied for speaker".to_string()),
                start: Err("no value supplied for start".to_string()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TranscriptTextSegmentEvent {
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn speaker<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.speaker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speaker: {e}"));
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptTextSegmentEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptTextSegmentEvent> for super::TranscriptTextSegmentEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptTextSegmentEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end: value.end?,
                id: value.id?,
                speaker: value.speaker?,
                start: value.start?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptTextSegmentEvent> for TranscriptTextSegmentEvent {
        fn from(value: super::TranscriptTextSegmentEvent) -> Self {
            Self {
                end: Ok(value.end),
                id: Ok(value.id),
                speaker: Ok(value.speaker),
                start: Ok(value.start),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptionDiarizedSegment {
        end: ::std::result::Result<f64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        speaker: ::std::result::Result<::std::string::String, ::std::string::String>,
        start: ::std::result::Result<f64, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_:
            ::std::result::Result<super::TranscriptionDiarizedSegmentType, ::std::string::String>,
    }
    impl ::std::default::Default for TranscriptionDiarizedSegment {
        fn default() -> Self {
            Self {
                end: Err("no value supplied for end".to_string()),
                id: Err("no value supplied for id".to_string()),
                speaker: Err("no value supplied for speaker".to_string()),
                start: Err("no value supplied for start".to_string()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TranscriptionDiarizedSegment {
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn speaker<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.speaker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speaker: {e}"));
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptionDiarizedSegmentType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptionDiarizedSegment> for super::TranscriptionDiarizedSegment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptionDiarizedSegment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end: value.end?,
                id: value.id?,
                speaker: value.speaker?,
                start: value.start?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptionDiarizedSegment> for TranscriptionDiarizedSegment {
        fn from(value: super::TranscriptionDiarizedSegment) -> Self {
            Self {
                end: Ok(value.end),
                id: Ok(value.id),
                speaker: Ok(value.speaker),
                start: Ok(value.start),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptionSegment {
        avg_logprob: ::std::result::Result<f32, ::std::string::String>,
        compression_ratio: ::std::result::Result<f32, ::std::string::String>,
        end: ::std::result::Result<f64, ::std::string::String>,
        id: ::std::result::Result<i64, ::std::string::String>,
        no_speech_prob: ::std::result::Result<f32, ::std::string::String>,
        seek: ::std::result::Result<i64, ::std::string::String>,
        start: ::std::result::Result<f64, ::std::string::String>,
        temperature: ::std::result::Result<f32, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        tokens: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for TranscriptionSegment {
        fn default() -> Self {
            Self {
                avg_logprob: Err("no value supplied for avg_logprob".to_string()),
                compression_ratio: Err("no value supplied for compression_ratio".to_string()),
                end: Err("no value supplied for end".to_string()),
                id: Err("no value supplied for id".to_string()),
                no_speech_prob: Err("no value supplied for no_speech_prob".to_string()),
                seek: Err("no value supplied for seek".to_string()),
                start: Err("no value supplied for start".to_string()),
                temperature: Err("no value supplied for temperature".to_string()),
                text: Err("no value supplied for text".to_string()),
                tokens: Err("no value supplied for tokens".to_string()),
            }
        }
    }
    impl TranscriptionSegment {
        pub fn avg_logprob<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f32>,
            T::Error: ::std::fmt::Display,
        {
            self.avg_logprob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for avg_logprob: {e}"));
            self
        }
        pub fn compression_ratio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f32>,
            T::Error: ::std::fmt::Display,
        {
            self.compression_ratio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for compression_ratio: {e}"));
            self
        }
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn no_speech_prob<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f32>,
            T::Error: ::std::fmt::Display,
        {
            self.no_speech_prob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for no_speech_prob: {e}"));
            self
        }
        pub fn seek<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.seek = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seek: {e}"));
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f32>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptionSegment> for super::TranscriptionSegment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptionSegment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                avg_logprob: value.avg_logprob?,
                compression_ratio: value.compression_ratio?,
                end: value.end?,
                id: value.id?,
                no_speech_prob: value.no_speech_prob?,
                seek: value.seek?,
                start: value.start?,
                temperature: value.temperature?,
                text: value.text?,
                tokens: value.tokens?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptionSegment> for TranscriptionSegment {
        fn from(value: super::TranscriptionSegment) -> Self {
            Self {
                avg_logprob: Ok(value.avg_logprob),
                compression_ratio: Ok(value.compression_ratio),
                end: Ok(value.end),
                id: Ok(value.id),
                no_speech_prob: Ok(value.no_speech_prob),
                seek: Ok(value.seek),
                start: Ok(value.start),
                temperature: Ok(value.temperature),
                text: Ok(value.text),
                tokens: Ok(value.tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptionWord {
        end: ::std::result::Result<f64, ::std::string::String>,
        start: ::std::result::Result<f64, ::std::string::String>,
        word: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TranscriptionWord {
        fn default() -> Self {
            Self {
                end: Err("no value supplied for end".to_string()),
                start: Err("no value supplied for start".to_string()),
                word: Err("no value supplied for word".to_string()),
            }
        }
    }
    impl TranscriptionWord {
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {e}"));
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {e}"));
            self
        }
        pub fn word<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.word = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for word: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptionWord> for super::TranscriptionWord {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptionWord,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end: value.end?,
                start: value.start?,
                word: value.word?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptionWord> for TranscriptionWord {
        fn from(value: super::TranscriptionWord) -> Self {
            Self {
                end: Ok(value.end),
                start: Ok(value.start),
                word: Ok(value.word),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateVoiceConsentParams {
        consent_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateVoiceConsentParams {
        fn default() -> Self {
            Self {
                consent_id: Err("no value supplied for consent_id".to_string()),
            }
        }
    }
    impl UpdateVoiceConsentParams {
        pub fn consent_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.consent_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for consent_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateVoiceConsentParams> for super::UpdateVoiceConsentParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateVoiceConsentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                consent_id: value.consent_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateVoiceConsentParams> for UpdateVoiceConsentParams {
        fn from(value: super::UpdateVoiceConsentParams) -> Self {
            Self {
                consent_id: Ok(value.consent_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateVoiceConsentRequest {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateVoiceConsentRequest {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl UpdateVoiceConsentRequest {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateVoiceConsentRequest> for super::UpdateVoiceConsentRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateVoiceConsentRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::UpdateVoiceConsentRequest> for UpdateVoiceConsentRequest {
        fn from(value: super::UpdateVoiceConsentRequest) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VadConfig {
        prefix_padding_ms: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        silence_duration_ms:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        threshold: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        type_: ::std::result::Result<super::VadConfigType, ::std::string::String>,
    }
    impl ::std::default::Default for VadConfig {
        fn default() -> Self {
            Self {
                prefix_padding_ms: Ok(Default::default()),
                silence_duration_ms: Ok(Default::default()),
                threshold: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VadConfig {
        pub fn prefix_padding_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.prefix_padding_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prefix_padding_ms: {e}"));
            self
        }
        pub fn silence_duration_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.silence_duration_ms = value.try_into().map_err(|e| {
                format!("error converting supplied value for silence_duration_ms: {e}")
            });
            self
        }
        pub fn threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VadConfigType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VadConfig> for super::VadConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VadConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prefix_padding_ms: value.prefix_padding_ms?,
                silence_duration_ms: value.silence_duration_ms?,
                threshold: value.threshold?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VadConfig> for VadConfig {
        fn from(value: super::VadConfig) -> Self {
            Self {
                prefix_padding_ms: Ok(value.prefix_padding_ms),
                silence_duration_ms: Ok(value.silence_duration_ms),
                threshold: Ok(value.threshold),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VoiceConsentDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::VoiceConsentDeletedResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for VoiceConsentDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl VoiceConsentDeletedResource {
        pub fn deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deleted: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceConsentDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VoiceConsentDeletedResource> for super::VoiceConsentDeletedResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VoiceConsentDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::VoiceConsentDeletedResource> for VoiceConsentDeletedResource {
        fn from(value: super::VoiceConsentDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VoiceConsentListResource {
        data: ::std::result::Result<
            ::std::vec::Vec<super::VoiceConsentResource>,
            ::std::string::String,
        >,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::VoiceConsentListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for VoiceConsentListResource {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                first_id: Ok(Default::default()),
                has_more: Err("no value supplied for has_more".to_string()),
                last_id: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl VoiceConsentListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VoiceConsentResource>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn first_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.first_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for first_id: {e}"));
            self
        }
        pub fn has_more<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.has_more = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for has_more: {e}"));
            self
        }
        pub fn last_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_id: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceConsentListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VoiceConsentListResource> for super::VoiceConsentListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VoiceConsentListResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                first_id: value.first_id?,
                has_more: value.has_more?,
                last_id: value.last_id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::VoiceConsentListResource> for VoiceConsentListResource {
        fn from(value: super::VoiceConsentListResource) -> Self {
            Self {
                data: Ok(value.data),
                first_id: Ok(value.first_id),
                has_more: Ok(value.has_more),
                last_id: Ok(value.last_id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VoiceConsentResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        language: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::VoiceConsentResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for VoiceConsentResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                language: Err("no value supplied for language".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl VoiceConsentResource {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceConsentResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VoiceConsentResource> for super::VoiceConsentResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VoiceConsentResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                language: value.language?,
                name: value.name?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::VoiceConsentResource> for VoiceConsentResource {
        fn from(value: super::VoiceConsentResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                language: Ok(value.language),
                name: Ok(value.name),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VoiceResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::VoiceResourceObject, ::std::string::String>,
        type_: ::std::result::Result<super::VoiceResourceType, ::std::string::String>,
    }
    impl ::std::default::Default for VoiceResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VoiceResource {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceResourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VoiceResource> for super::VoiceResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VoiceResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VoiceResource> for VoiceResource {
        fn from(value: super::VoiceResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                type_: Ok(value.type_),
            }
        }
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
