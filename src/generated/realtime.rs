#[doc = "`AudioTranscription`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AudioTranscription {
    #[doc = "Controls how long the model waits before emitting transcription text.\nHigher values can improve transcription accuracy at the cost of latency.\nOnly supported with `gpt-realtime-whisper` in GA Realtime sessions.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delay: ::std::option::Option<AudioTranscriptionDelay>,
    #[doc = "Words or phrases to guide transcription of the input audio. Supported by `gpt-transcribe` and `gpt-live-transcribe`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub keywords: ::std::vec::Vec<::std::string::String>,
    #[doc = "The language of the input audio. Supplying the input language in\n[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format\nwill improve accuracy and latency.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[doc = "Possible languages of the input audio, in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format. Supported by `gpt-transcribe` and `gpt-live-transcribe`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<::std::string::String>,
    #[doc = "The model to use for transcription. Current options are `whisper-1`, `gpt-transcribe`, `gpt-live-transcribe`, `gpt-4o-mini-transcribe`, `gpt-4o-mini-transcribe-2025-12-15`, `gpt-4o-transcribe`, `gpt-4o-transcribe-diarize`, and `gpt-realtime-whisper`. Use `gpt-4o-transcribe-diarize` when you need diarization with speaker labels.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "An optional text to guide the model's style or continue a previous audio\nsegment.\nFor `whisper-1`, the [prompt is a list of keywords](https://developers.openai.com/api/docs/guides/speech-to-text#prompting).\nFor `gpt-4o-transcribe` models (excluding `gpt-4o-transcribe-diarize`), the prompt is a free text string, for example \"expect words related to technology\".\nPrompt is not supported with `gpt-realtime-whisper` in GA Realtime sessions.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<::std::string::String>,
}
#[doc = "Controls how long the model waits before emitting transcription text.\nHigher values can improve transcription accuracy at the cost of latency.\nOnly supported with `gpt-realtime-whisper` in GA Realtime sessions.\n"]
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
pub enum AudioTranscriptionDelay {
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
}
impl ::std::fmt::Display for AudioTranscriptionDelay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Minimal => f.write_str("minimal"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
        }
    }
}
impl ::std::str::FromStr for AudioTranscriptionDelay {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "minimal" => Ok(Self::Minimal),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AudioTranscriptionDelay {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AudioTranscriptionDelay {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AudioTranscriptionResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AudioTranscriptionResponse {
    #[doc = "The language of the input audio.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[doc = "The possible input audio languages configured for transcription, in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<::std::string::String>,
    #[doc = "The model used for transcription. Current options are `whisper-1`, `gpt-transcribe`, `gpt-live-transcribe`, `gpt-4o-mini-transcribe`, `gpt-4o-mini-transcribe-2025-12-15`, `gpt-4o-transcribe`, `gpt-4o-transcribe-diarize`, and `gpt-realtime-whisper`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The prompt configured for input audio transcription, when present.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<::std::string::String>,
}
#[doc = "Enables tracing and sets default values for tracing configuration options. Always `auto`.\n"]
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
pub enum Auto {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for Auto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for Auto {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Auto {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Auto {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CallableToolAllowedCaller`"]
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
pub enum CallableToolAllowedCaller {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "programmatic")]
    Programmatic,
}
impl ::std::fmt::Display for CallableToolAllowedCaller {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Direct => f.write_str("direct"),
            Self::Programmatic => f.write_str("programmatic"),
        }
    }
}
impl ::std::str::FromStr for CallableToolAllowedCaller {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "direct" => Ok(Self::Direct),
            "programmatic" => Ok(Self::Programmatic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallableToolAllowedCaller {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CallableToolAllowedCaller {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for the client secret expiration. Expiration refers to the time after which\na client secret will no longer be valid for creating sessions. The session itself may\ncontinue after that time once started. A secret can be used to create multiple sessions\nuntil it expires.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ClientSecretExpiration {
    #[doc = "The anchor point for the client secret expiration, meaning that `seconds` will be added to the `created_at` time of the client secret to produce an expiration timestamp. Only `created_at` is currently supported.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub anchor: ::std::option::Option<ClientSecretExpirationAnchor>,
    #[doc = "The number of seconds from the anchor point to the expiration. Select a value between `10` and `7200` (2 hours). This default to 600 seconds (10 minutes) if not specified.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub seconds: ::std::option::Option<i64>,
}
#[doc = "The anchor point for the client secret expiration, meaning that `seconds` will be added to the `created_at` time of the client secret to produce an expiration timestamp. Only `created_at` is currently supported.\n"]
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
pub enum ClientSecretExpirationAnchor {
    #[serde(rename = "created_at")]
    CreatedAt,
}
impl ::std::fmt::Display for ClientSecretExpirationAnchor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CreatedAt => f.write_str("created_at"),
        }
    }
}
impl ::std::str::FromStr for ClientSecretExpirationAnchor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "created_at" => Ok(Self::CreatedAt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClientSecretExpirationAnchor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientSecretExpirationAnchor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FileInputDetail`"]
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
pub enum FileInputDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for FileInputDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for FileInputDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileInputDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileInputDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ImageDetail`"]
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
pub enum ImageDetail {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "original")]
    Original,
}
impl ::std::fmt::Display for ImageDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
            Self::Auto => f.write_str("auto"),
            Self::Original => f.write_str("original"),
        }
    }
}
impl ::std::str::FromStr for ImageDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            "auto" => Ok(Self::Auto),
            "original" => Ok(Self::Original),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A file input to the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputFileContent {
    #[doc = "The detail level of the file to be sent to the model. Use `auto` to let the system select the detail level; for GPT-5.6 and later models, `auto` uses high-quality rendering, which may increase input token usage. Use `low` for lower-cost rendering, or `high` to render the file at higher quality. Defaults to `auto`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<FileInputDetail>,
    #[doc = "The content of the file to be sent to the model.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_data: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "The URL of the file to be sent to the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_url: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the file to be sent to the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filename: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointConfig>,
    #[doc = "The type of the input item. Always `input_file`."]
    #[serde(rename = "type")]
    pub type_: InputFileContentType,
}
#[doc = "The type of the input item. Always `input_file`."]
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
pub enum InputFileContentType {
    #[serde(rename = "input_file")]
    InputFile,
}
impl ::std::fmt::Display for InputFileContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputFile => f.write_str("input_file"),
        }
    }
}
impl ::std::str::FromStr for InputFileContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_file" => Ok(Self::InputFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputFileContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputFileContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An image input to the model. Learn about [image inputs](https://developers.openai.com/api/docs/guides/images-vision)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputImageContent {
    #[doc = "The detail level of the image to be sent to the model. One of `high`, `low`, `auto`, or `original`. Defaults to `auto`."]
    pub detail: ImageDetail,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointConfig>,
    #[doc = "The type of the input item. Always `input_image`."]
    #[serde(rename = "type")]
    pub type_: InputImageContentType,
}
#[doc = "The type of the input item. Always `input_image`."]
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
pub enum InputImageContentType {
    #[serde(rename = "input_image")]
    InputImage,
}
impl ::std::fmt::Display for InputImageContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputImage => f.write_str("input_image"),
        }
    }
}
impl ::std::str::FromStr for InputImageContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_image" => Ok(Self::InputImage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputImageContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputImageContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A text input to the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputTextContent {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointConfig>,
    #[doc = "The text input to the model."]
    pub text: ::std::string::String,
    #[doc = "The type of the input item. Always `input_text`."]
    #[serde(rename = "type")]
    pub type_: InputTextContentType,
}
#[doc = "The type of the input item. Always `input_text`."]
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
pub enum InputTextContentType {
    #[serde(rename = "input_text")]
    InputText,
}
impl ::std::fmt::Display for InputTextContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputText => f.write_str("input_text"),
        }
    }
}
impl ::std::str::FromStr for InputTextContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_text" => Ok(Self::InputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A log probability object.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LogProbProperties {
    #[doc = "The bytes that were used to generate the log probability.\n"]
    pub bytes: ::std::vec::Vec<i64>,
    #[doc = "The log probability of the token.\n"]
    pub logprob: f64,
    #[doc = "The token that was used to generate the log probability.\n"]
    pub token: ::std::string::String,
}
#[doc = "A tool available on an MCP server.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpListToolsTool {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The JSON schema describing the tool's input.\n"]
    pub input_schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The name of the tool.\n"]
    pub name: ::std::string::String,
}
#[doc = "Give the model access to additional tools via remote Model Context Protocol\n(MCP) servers. [Learn more about MCP](https://developers.openai.com/api/docs/guides/tools-connectors-mcp).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpTool {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_callers: ::std::option::Option<::std::vec::Vec<CallableToolAllowedCaller>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_tools: ::std::option::Option<McpToolAllowedTools>,
    #[doc = "An OAuth access token that can be used with a remote MCP server, either\nwith a custom MCP server URL or a service connector. Your application\nmust handle the OAuth authorization flow and provide the token here.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub authorization: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for service connectors, like those available in ChatGPT. One of\n`server_url`, `connector_id`, or `tunnel_id` must be provided. Learn more\nabout service connectors [here](https://developers.openai.com/api/docs/guides/tools-connectors-mcp#connectors).\n\nThis field is deprecated for models released after September 1, 2026.\nUse `server_url` to connect to a remote MCP server, or `tunnel_id` to\nconnect through a Secure MCP Tunnel.\n\nCurrently supported `connector_id` values are:\n\n- Dropbox: `connector_dropbox`\n- Gmail: `connector_gmail`\n- Google Calendar: `connector_googlecalendar`\n- Google Drive: `connector_googledrive`\n- Microsoft Teams: `connector_microsoftteams`\n- Outlook Calendar: `connector_outlookcalendar`\n- Outlook Email: `connector_outlookemail`\n- SharePoint: `connector_sharepoint`\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub connector_id: ::std::option::Option<McpToolConnectorId>,
    #[doc = "Whether this MCP tool is deferred and discovered via tool search.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub defer_loading: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub headers: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub require_approval: ::std::option::Option<McpToolRequireApproval>,
    #[doc = "Optional description of the MCP server, used to provide more context.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub server_description: ::std::option::Option<::std::string::String>,
    #[doc = "A label for this MCP server, used to identify it in tool calls.\n"]
    pub server_label: ::std::string::String,
    #[doc = "The URL for the MCP server. One of `server_url`, `connector_id`, or\n`tunnel_id` must be provided.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub server_url: ::std::option::Option<::std::string::String>,
    #[doc = "The Secure MCP Tunnel ID to use instead of a direct server URL. One of\n`server_url`, `connector_id`, or `tunnel_id` must be provided.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tunnel_id: ::std::option::Option<McpToolTunnelId>,
    #[doc = "The type of the MCP tool. Always `mcp`."]
    #[serde(rename = "type")]
    pub type_: McpToolType,
}
#[doc = "List of allowed tool names or a filter object.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum McpToolAllowedTools {
    AllowedTools(::std::vec::Vec<::std::string::String>),
    ToolFilter(McpToolFilter),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for McpToolAllowedTools {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::AllowedTools(value)
    }
}
impl ::std::convert::From<McpToolFilter> for McpToolAllowedTools {
    fn from(value: McpToolFilter) -> Self {
        Self::ToolFilter(value)
    }
}
#[doc = "Specify a single approval policy for all tools. One of `always` or\n`never`. When set to `always`, all tools will require approval. When\nset to `never`, all tools will not require approval.\n"]
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
pub enum McpToolApprovalSetting {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "never")]
    Never,
}
impl ::std::fmt::Display for McpToolApprovalSetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Always => f.write_str("always"),
            Self::Never => f.write_str("never"),
        }
    }
}
impl ::std::str::FromStr for McpToolApprovalSetting {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolApprovalSetting {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolApprovalSetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier for service connectors, like those available in ChatGPT. One of\n`server_url`, `connector_id`, or `tunnel_id` must be provided. Learn more\nabout service connectors [here](https://developers.openai.com/api/docs/guides/tools-connectors-mcp#connectors).\n\nThis field is deprecated for models released after September 1, 2026.\nUse `server_url` to connect to a remote MCP server, or `tunnel_id` to\nconnect through a Secure MCP Tunnel.\n\nCurrently supported `connector_id` values are:\n\n- Dropbox: `connector_dropbox`\n- Gmail: `connector_gmail`\n- Google Calendar: `connector_googlecalendar`\n- Google Drive: `connector_googledrive`\n- Microsoft Teams: `connector_microsoftteams`\n- Outlook Calendar: `connector_outlookcalendar`\n- Outlook Email: `connector_outlookemail`\n- SharePoint: `connector_sharepoint`\n"]
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
pub enum McpToolConnectorId {
    #[serde(rename = "connector_dropbox")]
    ConnectorDropbox,
    #[serde(rename = "connector_gmail")]
    ConnectorGmail,
    #[serde(rename = "connector_googlecalendar")]
    ConnectorGooglecalendar,
    #[serde(rename = "connector_googledrive")]
    ConnectorGoogledrive,
    #[serde(rename = "connector_microsoftteams")]
    ConnectorMicrosoftteams,
    #[serde(rename = "connector_outlookcalendar")]
    ConnectorOutlookcalendar,
    #[serde(rename = "connector_outlookemail")]
    ConnectorOutlookemail,
    #[serde(rename = "connector_sharepoint")]
    ConnectorSharepoint,
}
impl ::std::fmt::Display for McpToolConnectorId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConnectorDropbox => f.write_str("connector_dropbox"),
            Self::ConnectorGmail => f.write_str("connector_gmail"),
            Self::ConnectorGooglecalendar => f.write_str("connector_googlecalendar"),
            Self::ConnectorGoogledrive => f.write_str("connector_googledrive"),
            Self::ConnectorMicrosoftteams => f.write_str("connector_microsoftteams"),
            Self::ConnectorOutlookcalendar => f.write_str("connector_outlookcalendar"),
            Self::ConnectorOutlookemail => f.write_str("connector_outlookemail"),
            Self::ConnectorSharepoint => f.write_str("connector_sharepoint"),
        }
    }
}
impl ::std::str::FromStr for McpToolConnectorId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "connector_dropbox" => Ok(Self::ConnectorDropbox),
            "connector_gmail" => Ok(Self::ConnectorGmail),
            "connector_googlecalendar" => Ok(Self::ConnectorGooglecalendar),
            "connector_googledrive" => Ok(Self::ConnectorGoogledrive),
            "connector_microsoftteams" => Ok(Self::ConnectorMicrosoftteams),
            "connector_outlookcalendar" => Ok(Self::ConnectorOutlookcalendar),
            "connector_outlookemail" => Ok(Self::ConnectorOutlookemail),
            "connector_sharepoint" => Ok(Self::ConnectorSharepoint),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolConnectorId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolConnectorId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A filter object to specify which tools are allowed.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct McpToolFilter {
    #[doc = "Indicates whether or not a tool modifies data or is read-only. If an\nMCP server is [annotated with `readOnlyHint`](https://modelcontextprotocol.io/specification/2025-06-18/schema#toolannotations-readonlyhint),\nit will match this filter.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub read_only: ::std::option::Option<bool>,
    #[doc = "List of allowed tool names."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tool_names: ::std::vec::Vec<::std::string::String>,
}
#[doc = "Specify which of the MCP server's tools require approval."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged, deny_unknown_fields)]
pub enum McpToolRequireApproval {
    Filter {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        always: ::std::option::Option<McpToolFilter>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        never: ::std::option::Option<McpToolFilter>,
    },
    Setting(McpToolApprovalSetting),
}
impl ::std::convert::From<McpToolApprovalSetting> for McpToolRequireApproval {
    fn from(value: McpToolApprovalSetting) -> Self {
        Self::Setting(value)
    }
}
#[doc = "The Secure MCP Tunnel ID to use instead of a direct server URL. One of\n`server_url`, `connector_id`, or `tunnel_id` must be provided.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct McpToolTunnelId(::std::string::String);
impl ::std::ops::Deref for McpToolTunnelId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<McpToolTunnelId> for ::std::string::String {
    fn from(value: McpToolTunnelId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for McpToolTunnelId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^tunnel_[a-z0-9]{32}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^tunnel_[a-z0-9]{32}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for McpToolTunnelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolTunnelId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for McpToolTunnelId {
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
#[doc = "The type of the MCP tool. Always `mcp`."]
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
pub enum McpToolType {
    #[serde(rename = "mcp")]
    Mcp,
}
impl ::std::fmt::Display for McpToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mcp => f.write_str("mcp"),
        }
    }
}
impl ::std::str::FromStr for McpToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp" => Ok(Self::Mcp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Metadata`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Metadata(pub ::std::option::Option<MetadataValue>);
impl ::std::ops::Deref for Metadata {
    type Target = ::std::option::Option<MetadataValue>;
    fn deref(&self) -> &::std::option::Option<MetadataValue> {
        &self.0
    }
}
impl ::std::convert::From<Metadata> for ::std::option::Option<MetadataValue> {
    fn from(value: Metadata) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<MetadataValue>> for Metadata {
    fn from(value: ::std::option::Option<MetadataValue>) -> Self {
        Self(value)
    }
}
#[doc = "Set of 16 key-value pairs that can be attached to an object. This can be\nuseful for storing additional information about the object in a structured\nformat, and querying for objects via API or the dashboard.\n\nKeys are strings with a maximum length of 64 characters. Values are strings\nwith a maximum length of 512 characters.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct MetadataValue(
    pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
);
impl ::std::ops::Deref for MetadataValue {
    type Target = ::std::collections::HashMap<::std::string::String, ::std::string::String>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<MetadataValue>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String>
{
    fn from(value: MetadataValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, ::std::string::String>>
    for MetadataValue
{
    fn from(
        value: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    ) -> Self {
        Self(value)
    }
}
#[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
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
pub enum NoiseReductionType {
    #[serde(rename = "near_field")]
    NearField,
    #[serde(rename = "far_field")]
    FarField,
}
impl ::std::fmt::Display for NoiseReductionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NearField => f.write_str("near_field"),
            Self::FarField => f.write_str("far_field"),
        }
    }
}
impl ::std::str::FromStr for NoiseReductionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "near_field" => Ok(Self::NearField),
            "far_field" => Ok(Self::FarField),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NoiseReductionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NoiseReductionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The PCM audio format. Only a 24kHz sample rate is supported."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct PcmAudioFormat {
    #[doc = "The sample rate of the audio. Always `24000`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rate: ::std::option::Option<PcmAudioFormatRate>,
    #[doc = "The audio format. Always `audio/pcm`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<PcmAudioFormatType>,
}
#[doc = "The sample rate of the audio. Always `24000`."]
#[derive(:: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PcmAudioFormatRate(i64);
impl ::std::ops::Deref for PcmAudioFormatRate {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<PcmAudioFormatRate> for i64 {
    fn from(value: PcmAudioFormatRate) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for PcmAudioFormatRate {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![24000_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for PcmAudioFormatRate {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "The audio format. Always `audio/pcm`."]
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
pub enum PcmAudioFormatType {
    #[serde(rename = "audio/pcm")]
    AudioPcm,
}
impl ::std::fmt::Display for PcmAudioFormatType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioPcm => f.write_str("audio/pcm"),
        }
    }
}
impl ::std::str::FromStr for PcmAudioFormatType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio/pcm" => Ok(Self::AudioPcm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PcmAudioFormatType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PcmAudioFormatType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The G.711 A-law format."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct PcmaAudioFormat {
    #[doc = "The audio format. Always `audio/pcma`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<PcmaAudioFormatType>,
}
#[doc = "The audio format. Always `audio/pcma`."]
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
pub enum PcmaAudioFormatType {
    #[serde(rename = "audio/pcma")]
    AudioPcma,
}
impl ::std::fmt::Display for PcmaAudioFormatType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioPcma => f.write_str("audio/pcma"),
        }
    }
}
impl ::std::str::FromStr for PcmaAudioFormatType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio/pcma" => Ok(Self::AudioPcma),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PcmaAudioFormatType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PcmaAudioFormatType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The G.711 μ-law format."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct PcmuAudioFormat {
    #[doc = "The audio format. Always `audio/pcmu`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<PcmuAudioFormatType>,
}
#[doc = "The audio format. Always `audio/pcmu`."]
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
pub enum PcmuAudioFormatType {
    #[serde(rename = "audio/pcmu")]
    AudioPcmu,
}
impl ::std::fmt::Display for PcmuAudioFormatType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioPcmu => f.write_str("audio/pcmu"),
        }
    }
}
impl ::std::str::FromStr for PcmuAudioFormatType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio/pcmu" => Ok(Self::AudioPcmu),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PcmuAudioFormatType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PcmuAudioFormatType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Prompt`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Prompt(pub ::std::option::Option<PromptValue>);
impl ::std::ops::Deref for Prompt {
    type Target = ::std::option::Option<PromptValue>;
    fn deref(&self) -> &::std::option::Option<PromptValue> {
        &self.0
    }
}
impl ::std::convert::From<Prompt> for ::std::option::Option<PromptValue> {
    fn from(value: Prompt) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<PromptValue>> for Prompt {
    fn from(value: ::std::option::Option<PromptValue>) -> Self {
        Self(value)
    }
}
#[doc = "Marks the exact end of a reusable prompt prefix. The breakpoint inherits its TTL from the request's `prompt_cache_options.ttl`; the boundary is not rounded to a token block."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptCacheBreakpointConfig {
    #[doc = "The breakpoint mode. Always `explicit`."]
    pub mode: PromptCacheBreakpointConfigMode,
}
#[doc = "The breakpoint mode. Always `explicit`."]
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
pub enum PromptCacheBreakpointConfigMode {
    #[serde(rename = "explicit")]
    Explicit,
}
impl ::std::fmt::Display for PromptCacheBreakpointConfigMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Explicit => f.write_str("explicit"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheBreakpointConfigMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "explicit" => Ok(Self::Explicit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheBreakpointConfigMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptCacheBreakpointConfigMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Reference to a prompt template and its variables.\n[Learn more](https://developers.openai.com/api/docs/guides/text?api-mode=responses#version-prompts-in-code).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptValue {
    #[doc = "The unique identifier of the prompt template to use."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub variables: ::std::option::Option<ResponsePromptVariables>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
#[doc = "`RealtimeApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeApiSchema {
    #[serde(rename = "AcceptRealtimeCallRequest")]
    pub accept_realtime_call_request: RealtimeSessionCreateRequestGa,
    #[serde(rename = "CreateRealtimeCallRequest")]
    pub create_realtime_call_request: RealtimeCallCreateRequest,
    #[serde(rename = "CreateRealtimeClientSecretRequest")]
    pub create_realtime_client_secret_request: RealtimeCreateClientSecretRequest,
    #[serde(rename = "CreateRealtimeClientSecretResponse")]
    pub create_realtime_client_secret_response: RealtimeCreateClientSecretResponse,
    #[serde(rename = "CreateRealtimeSessionRequest")]
    pub create_realtime_session_request: RealtimeSessionCreateRequest,
    #[serde(rename = "CreateRealtimeSessionResponse")]
    pub create_realtime_session_response: RealtimeSessionCreateResponse,
    #[serde(rename = "CreateRealtimeTranscriptionSessionRequest")]
    pub create_realtime_transcription_session_request: RealtimeTranscriptionSessionCreateRequest,
    #[serde(rename = "CreateRealtimeTranscriptionSessionResponse")]
    pub create_realtime_transcription_session_response: RealtimeTranscriptionSessionCreateResponse,
    #[serde(rename = "CreateRealtimeTranslationClientSecretRequest")]
    pub create_realtime_translation_client_secret_request:
        RealtimeTranslationClientSecretCreateRequest,
    #[serde(rename = "CreateRealtimeTranslationClientSecretResponse")]
    pub create_realtime_translation_client_secret_response:
        RealtimeTranslationClientSecretCreateResponse,
    #[serde(rename = "RealtimeClientEvent")]
    pub realtime_client_event: RealtimeClientEvent,
    #[serde(rename = "RealtimeServerEvent")]
    pub realtime_server_event: RealtimeServerEvent,
    #[serde(rename = "RealtimeTranslationClientEvent")]
    pub realtime_translation_client_event: RealtimeTranslationClientEvent,
    #[serde(rename = "RealtimeTranslationServerEvent")]
    pub realtime_translation_server_event: RealtimeTranslationServerEvent,
    #[serde(rename = "ReferRealtimeCallRequest")]
    pub refer_realtime_call_request: RealtimeCallReferRequest,
    #[serde(rename = "RejectRealtimeCallRequest")]
    pub reject_realtime_call_request: RealtimeCallRejectRequest,
}
#[doc = "`RealtimeAudioFormats`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeAudioFormats {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<PcmAudioFormat>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<PcmuAudioFormat>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<PcmaAudioFormat>,
}
#[doc = "Parameters required to initiate a realtime call and receive the SDP answer\nneeded to complete a WebRTC peer connection. Provide an SDP offer generated\nby your client and optionally configure the session that will answer the call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RealtimeCallCreateRequest {
    #[doc = "WebRTC Session Description Protocol (SDP) offer generated by the caller."]
    pub sdp: ::std::string::String,
    #[doc = "Optional session configuration to apply before the realtime session is\ncreated. Use the same parameters you would send in a [`create client secret`](https://developers.openai.com/api/reference/resources/realtime/subresources/client_secrets/methods/create)\nrequest."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub session: ::std::option::Option<RealtimeSessionCreateRequestGa>,
}
#[doc = "Parameters required to transfer a SIP call to a new destination using the\nRealtime API."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RealtimeCallReferRequest {
    #[doc = "URI that should appear in the SIP Refer-To header. Supports values like\n`tel:+14155550123` or `sip:agent@example.com`."]
    pub target_uri: ::std::string::String,
}
#[doc = "Parameters used to decline an incoming SIP call handled by the Realtime API."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RealtimeCallRejectRequest {
    #[doc = "SIP response code to send back to the caller. Defaults to `603` (Decline)\nwhen omitted."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status_code: ::std::option::Option<i64>,
}
#[doc = "A realtime client event.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeClientEvent {
    ConversationItemCreate(RealtimeClientEventConversationItemCreate),
    ConversationItemDelete(RealtimeClientEventConversationItemDelete),
    ConversationItemRetrieve(RealtimeClientEventConversationItemRetrieve),
    ConversationItemTruncate(RealtimeClientEventConversationItemTruncate),
    InputAudioBufferAppend(RealtimeClientEventInputAudioBufferAppend),
    InputAudioBufferClear(RealtimeClientEventInputAudioBufferClear),
    OutputAudioBufferClear(RealtimeClientEventOutputAudioBufferClear),
    InputAudioBufferCommit(RealtimeClientEventInputAudioBufferCommit),
    ResponseCancel(RealtimeClientEventResponseCancel),
    ResponseCreate(RealtimeClientEventResponseCreate),
    SessionUpdate(RealtimeClientEventSessionUpdate),
}
impl ::std::convert::From<RealtimeClientEventConversationItemCreate> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventConversationItemCreate) -> Self {
        Self::ConversationItemCreate(value)
    }
}
impl ::std::convert::From<RealtimeClientEventConversationItemDelete> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventConversationItemDelete) -> Self {
        Self::ConversationItemDelete(value)
    }
}
impl ::std::convert::From<RealtimeClientEventConversationItemRetrieve> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventConversationItemRetrieve) -> Self {
        Self::ConversationItemRetrieve(value)
    }
}
impl ::std::convert::From<RealtimeClientEventConversationItemTruncate> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventConversationItemTruncate) -> Self {
        Self::ConversationItemTruncate(value)
    }
}
impl ::std::convert::From<RealtimeClientEventInputAudioBufferAppend> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventInputAudioBufferAppend) -> Self {
        Self::InputAudioBufferAppend(value)
    }
}
impl ::std::convert::From<RealtimeClientEventInputAudioBufferClear> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventInputAudioBufferClear) -> Self {
        Self::InputAudioBufferClear(value)
    }
}
impl ::std::convert::From<RealtimeClientEventOutputAudioBufferClear> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventOutputAudioBufferClear) -> Self {
        Self::OutputAudioBufferClear(value)
    }
}
impl ::std::convert::From<RealtimeClientEventInputAudioBufferCommit> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventInputAudioBufferCommit) -> Self {
        Self::InputAudioBufferCommit(value)
    }
}
impl ::std::convert::From<RealtimeClientEventResponseCancel> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventResponseCancel) -> Self {
        Self::ResponseCancel(value)
    }
}
impl ::std::convert::From<RealtimeClientEventResponseCreate> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventResponseCreate) -> Self {
        Self::ResponseCreate(value)
    }
}
impl ::std::convert::From<RealtimeClientEventSessionUpdate> for RealtimeClientEvent {
    fn from(value: RealtimeClientEventSessionUpdate) -> Self {
        Self::SessionUpdate(value)
    }
}
#[doc = "Add a new Item to the Conversation's context, including messages, function \ncalls, and function call responses. This event can be used both to populate a \n\"history\" of the conversation and to add new items mid-stream, but has the \ncurrent limitation that it cannot populate assistant audio messages.\n\nIf successful, the server will emit a `conversation.item.added` event and,\nwhen the item is finalized, a `conversation.item.done` event. Otherwise, an\n`error` event will be sent.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventConversationItemCreate {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventConversationItemCreateEventId>,
    pub item: RealtimeConversationItem,
    #[doc = "The ID of the preceding item after which the new item will be inserted. If not set, the new item will be appended to the end of the conversation.\n\nIf set to `root`, the new item will be added to the beginning of the conversation.\n\nIf set to an existing ID, it allows an item to be inserted mid-conversation. If the ID cannot be found, an error will be returned and the item will not be added.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_item_id: ::std::option::Option<::std::string::String>,
    #[doc = "The event type, must be `conversation.item.create`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventConversationItemCreateType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventConversationItemCreateEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventConversationItemCreateEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventConversationItemCreateEventId>
    for ::std::string::String
{
    fn from(value: RealtimeClientEventConversationItemCreateEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventConversationItemCreateEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventConversationItemCreateEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventConversationItemCreateEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventConversationItemCreateEventId {
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
#[doc = "The event type, must be `conversation.item.create`."]
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
pub enum RealtimeClientEventConversationItemCreateType {
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate,
}
impl ::std::fmt::Display for RealtimeClientEventConversationItemCreateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemCreate => f.write_str("conversation.item.create"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventConversationItemCreateType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.create" => Ok(Self::ConversationItemCreate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventConversationItemCreateType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventConversationItemCreateType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Send this event when you want to remove any item from the conversation \nhistory. The server will respond with a `conversation.item.deleted` event, \nunless the item does not exist in the conversation history, in which case the \nserver will respond with an error.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventConversationItemDelete {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventConversationItemDeleteEventId>,
    #[doc = "The ID of the item to delete."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `conversation.item.delete`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventConversationItemDeleteType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventConversationItemDeleteEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventConversationItemDeleteEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventConversationItemDeleteEventId>
    for ::std::string::String
{
    fn from(value: RealtimeClientEventConversationItemDeleteEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventConversationItemDeleteEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventConversationItemDeleteEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventConversationItemDeleteEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventConversationItemDeleteEventId {
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
#[doc = "The event type, must be `conversation.item.delete`."]
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
pub enum RealtimeClientEventConversationItemDeleteType {
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete,
}
impl ::std::fmt::Display for RealtimeClientEventConversationItemDeleteType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemDelete => f.write_str("conversation.item.delete"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventConversationItemDeleteType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.delete" => Ok(Self::ConversationItemDelete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventConversationItemDeleteType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventConversationItemDeleteType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Send this event when you want to retrieve the server's representation of a specific item in the conversation history. This is useful, for example, to inspect user audio after noise cancellation and VAD.\nThe server will respond with a `conversation.item.retrieved` event, \nunless the item does not exist in the conversation history, in which case the \nserver will respond with an error.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventConversationItemRetrieve {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventConversationItemRetrieveEventId>,
    #[doc = "The ID of the item to retrieve."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `conversation.item.retrieve`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventConversationItemRetrieveType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventConversationItemRetrieveEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventConversationItemRetrieveEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventConversationItemRetrieveEventId>
    for ::std::string::String
{
    fn from(value: RealtimeClientEventConversationItemRetrieveEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventConversationItemRetrieveEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventConversationItemRetrieveEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventConversationItemRetrieveEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventConversationItemRetrieveEventId {
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
#[doc = "The event type, must be `conversation.item.retrieve`."]
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
pub enum RealtimeClientEventConversationItemRetrieveType {
    #[serde(rename = "conversation.item.retrieve")]
    ConversationItemRetrieve,
}
impl ::std::fmt::Display for RealtimeClientEventConversationItemRetrieveType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemRetrieve => f.write_str("conversation.item.retrieve"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventConversationItemRetrieveType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.retrieve" => Ok(Self::ConversationItemRetrieve),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventConversationItemRetrieveType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventConversationItemRetrieveType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Send this event to truncate a previous assistant message’s audio. The server \nwill produce audio faster than realtime, so this event is useful when the user \ninterrupts to truncate audio that has already been sent to the client but not \nyet played. This will synchronize the server's understanding of the audio with \nthe client's playback.\n\nTruncating audio will delete the server-side text transcript to ensure there \nis not text in the context that hasn't been heard by the user.\n\nIf successful, the server will respond with a `conversation.item.truncated` \nevent. \n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventConversationItemTruncate {
    #[doc = "Inclusive duration up to which audio is truncated, in milliseconds. If \nthe audio_end_ms is greater than the actual audio duration, the server \nwill respond with an error.\n"]
    pub audio_end_ms: i64,
    #[doc = "The index of the content part to truncate. Set this to `0`."]
    pub content_index: i64,
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventConversationItemTruncateEventId>,
    #[doc = "The ID of the assistant message item to truncate. Only assistant message \nitems can be truncated.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `conversation.item.truncate`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventConversationItemTruncateType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventConversationItemTruncateEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventConversationItemTruncateEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventConversationItemTruncateEventId>
    for ::std::string::String
{
    fn from(value: RealtimeClientEventConversationItemTruncateEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventConversationItemTruncateEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventConversationItemTruncateEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventConversationItemTruncateEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventConversationItemTruncateEventId {
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
#[doc = "The event type, must be `conversation.item.truncate`."]
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
pub enum RealtimeClientEventConversationItemTruncateType {
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate,
}
impl ::std::fmt::Display for RealtimeClientEventConversationItemTruncateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemTruncate => f.write_str("conversation.item.truncate"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventConversationItemTruncateType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.truncate" => Ok(Self::ConversationItemTruncate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventConversationItemTruncateType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventConversationItemTruncateType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Send this event to append audio bytes to the input audio buffer. The audio \nbuffer is temporary storage you can write to and later commit. A \"commit\" will create a new\nuser message item in the conversation history from the buffer content and clear the buffer.\nInput audio transcription (if enabled) will be generated when the buffer is committed.\n\nIf VAD is enabled the audio buffer is used to detect speech and the server will decide \nwhen to commit. When Server VAD is disabled, you must commit the audio buffer\nmanually. Input audio noise reduction operates on writes to the audio buffer.\n\nThe client may choose how much audio to place in each event up to a maximum \nof 15 MiB, for example streaming smaller chunks from the client may allow the \nVAD to be more responsive. Unlike most other client events, the server will \nnot send a confirmation response to this event.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventInputAudioBufferAppend {
    #[doc = "Base64-encoded audio bytes. This must be in the format specified by the \n`input_audio_format` field in the session configuration.\n"]
    pub audio: ::std::string::String,
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventInputAudioBufferAppendEventId>,
    #[doc = "The event type, must be `input_audio_buffer.append`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventInputAudioBufferAppendType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventInputAudioBufferAppendEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventInputAudioBufferAppendEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventInputAudioBufferAppendEventId>
    for ::std::string::String
{
    fn from(value: RealtimeClientEventInputAudioBufferAppendEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventInputAudioBufferAppendEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventInputAudioBufferAppendEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventInputAudioBufferAppendEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventInputAudioBufferAppendEventId {
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
#[doc = "The event type, must be `input_audio_buffer.append`."]
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
pub enum RealtimeClientEventInputAudioBufferAppendType {
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend,
}
impl ::std::fmt::Display for RealtimeClientEventInputAudioBufferAppendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudioBufferAppend => f.write_str("input_audio_buffer.append"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventInputAudioBufferAppendType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio_buffer.append" => Ok(Self::InputAudioBufferAppend),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventInputAudioBufferAppendType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventInputAudioBufferAppendType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Send this event to clear the audio bytes in the buffer. The server will \nrespond with an `input_audio_buffer.cleared` event.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventInputAudioBufferClear {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventInputAudioBufferClearEventId>,
    #[doc = "The event type, must be `input_audio_buffer.clear`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventInputAudioBufferClearType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventInputAudioBufferClearEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventInputAudioBufferClearEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventInputAudioBufferClearEventId>
    for ::std::string::String
{
    fn from(value: RealtimeClientEventInputAudioBufferClearEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventInputAudioBufferClearEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventInputAudioBufferClearEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventInputAudioBufferClearEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventInputAudioBufferClearEventId {
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
#[doc = "The event type, must be `input_audio_buffer.clear`."]
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
pub enum RealtimeClientEventInputAudioBufferClearType {
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear,
}
impl ::std::fmt::Display for RealtimeClientEventInputAudioBufferClearType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudioBufferClear => f.write_str("input_audio_buffer.clear"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventInputAudioBufferClearType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio_buffer.clear" => Ok(Self::InputAudioBufferClear),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventInputAudioBufferClearType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventInputAudioBufferClearType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Send this event to commit the user input audio buffer, which will create a  new user message item in the conversation. This event will produce an error  if the input audio buffer is empty. When in Server VAD mode, the client does  not need to send this event, the server will commit the audio buffer  automatically.\n\nCommitting the input audio buffer will trigger input audio transcription  (if enabled in session configuration), but it will not create a response  from the model. The server will respond with an `input_audio_buffer.committed` event.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventInputAudioBufferCommit {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventInputAudioBufferCommitEventId>,
    #[doc = "The event type, must be `input_audio_buffer.commit`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventInputAudioBufferCommitType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventInputAudioBufferCommitEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventInputAudioBufferCommitEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventInputAudioBufferCommitEventId>
    for ::std::string::String
{
    fn from(value: RealtimeClientEventInputAudioBufferCommitEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventInputAudioBufferCommitEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventInputAudioBufferCommitEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventInputAudioBufferCommitEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventInputAudioBufferCommitEventId {
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
#[doc = "The event type, must be `input_audio_buffer.commit`."]
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
pub enum RealtimeClientEventInputAudioBufferCommitType {
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit,
}
impl ::std::fmt::Display for RealtimeClientEventInputAudioBufferCommitType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudioBufferCommit => f.write_str("input_audio_buffer.commit"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventInputAudioBufferCommitType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio_buffer.commit" => Ok(Self::InputAudioBufferCommit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventInputAudioBufferCommitType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventInputAudioBufferCommitType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "**WebRTC/SIP Only:** Emit to cut off the current audio response. This will trigger the server to\nstop generating audio and emit a `output_audio_buffer.cleared` event. This\nevent should be preceded by a `response.cancel` client event to stop the\ngeneration of the current response.\n[Learn more](https://developers.openai.com/api/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventOutputAudioBufferClear {
    #[doc = "The unique ID of the client event used for error handling."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The event type, must be `output_audio_buffer.clear`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventOutputAudioBufferClearType,
}
#[doc = "The event type, must be `output_audio_buffer.clear`."]
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
pub enum RealtimeClientEventOutputAudioBufferClearType {
    #[serde(rename = "output_audio_buffer.clear")]
    OutputAudioBufferClear,
}
impl ::std::fmt::Display for RealtimeClientEventOutputAudioBufferClearType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutputAudioBufferClear => f.write_str("output_audio_buffer.clear"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventOutputAudioBufferClearType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "output_audio_buffer.clear" => Ok(Self::OutputAudioBufferClear),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventOutputAudioBufferClearType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeClientEventOutputAudioBufferClearType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Send this event to cancel an in-progress response. The server will respond \nwith a `response.done` event with a status of `response.status=cancelled`. If \nthere is no response to cancel, the server will respond with an error. It's safe\nto call `response.cancel` even if no response is in progress, an error will be\nreturned the session will remain unaffected.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventResponseCancel {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventResponseCancelEventId>,
    #[doc = "A specific response ID to cancel - if not provided, will cancel an \nin-progress response in the default conversation.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_id: ::std::option::Option<::std::string::String>,
    #[doc = "The event type, must be `response.cancel`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventResponseCancelType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventResponseCancelEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventResponseCancelEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventResponseCancelEventId> for ::std::string::String {
    fn from(value: RealtimeClientEventResponseCancelEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventResponseCancelEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventResponseCancelEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeClientEventResponseCancelEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventResponseCancelEventId {
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
#[doc = "The event type, must be `response.cancel`."]
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
pub enum RealtimeClientEventResponseCancelType {
    #[serde(rename = "response.cancel")]
    ResponseCancel,
}
impl ::std::fmt::Display for RealtimeClientEventResponseCancelType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCancel => f.write_str("response.cancel"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventResponseCancelType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.cancel" => Ok(Self::ResponseCancel),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventResponseCancelType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeClientEventResponseCancelType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "This event instructs the server to create a Response, which means triggering \nmodel inference. When in Server VAD mode, the server will create Responses \nautomatically.\n\nA Response will include at least one Item, and may have two, in which case \nthe second will be a function call. These Items will be appended to the \nconversation history by default.\n\nThe server will respond with a `response.created` event, events for Items \nand content created, and finally a `response.done` event to indicate the \nResponse is complete.\n\nThe `response.create` event includes inference configuration like \n`instructions` and `tools`. If these are set, they will override the Session's \nconfiguration for this Response only.\n\nResponses can be created out-of-band of the default Conversation, meaning that they can\nhave arbitrary input, and it's possible to disable writing the output to the Conversation.\nOnly one Response can write to the default Conversation at a time, but otherwise multiple\nResponses can be created in parallel. The `metadata` field is a good way to disambiguate\nmultiple simultaneous Responses.\n\nClients can set `conversation` to `none` to create a Response that does not write to the default\nConversation. Arbitrary input can be provided with the `input` field, which is an array accepting\nraw Items and references to existing Items.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventResponseCreate {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventResponseCreateEventId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response: ::std::option::Option<RealtimeResponseCreateParams>,
    #[doc = "The event type, must be `response.create`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventResponseCreateType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventResponseCreateEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventResponseCreateEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventResponseCreateEventId> for ::std::string::String {
    fn from(value: RealtimeClientEventResponseCreateEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventResponseCreateEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventResponseCreateEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeClientEventResponseCreateEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventResponseCreateEventId {
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
#[doc = "The event type, must be `response.create`."]
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
pub enum RealtimeClientEventResponseCreateType {
    #[serde(rename = "response.create")]
    ResponseCreate,
}
impl ::std::fmt::Display for RealtimeClientEventResponseCreateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCreate => f.write_str("response.create"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventResponseCreateType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.create" => Ok(Self::ResponseCreate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventResponseCreateType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeClientEventResponseCreateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Send this event to update the session’s configuration.\nThe client may send this event at any time to update any field\nexcept for `voice` and `model`. `voice` can be updated only if there have been no other audio outputs yet.\n\nWhen the server receives a `session.update`, it will respond\nwith a `session.updated` event showing the full, effective configuration.\nOnly the fields that are present in the `session.update` are updated. To clear a field like\n`instructions`, pass an empty string. To clear a field like `tools`, pass an empty array.\nTo clear a field like `turn_detection`, pass `null`.\n\nTo turn off input audio noise reduction, send this Realtime event:\n\n```json\n{\"type\":\"session.update\",\"session\":{\"type\":\"realtime\",\"audio\":{\"input\":{\"noise_reduction\":null}}}}\n```\n\nFor a transcription session, use `\"type\":\"transcription\"` inside `session`.\nOmitting `audio.input.noise_reduction` from an update leaves its current setting unchanged.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeClientEventSessionUpdate {
    #[doc = "Optional client-generated ID used to identify this event. This is an arbitrary string that a client may assign. It will be passed back if there is an error with the event, but the corresponding `session.updated` event will not include it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeClientEventSessionUpdateEventId>,
    #[doc = "Update the Realtime session. Choose either a realtime\nsession or a transcription session.\n"]
    pub session: RealtimeClientEventSessionUpdateSession,
    #[doc = "The event type, must be `session.update`."]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventSessionUpdateType,
}
#[doc = "Optional client-generated ID used to identify this event. This is an arbitrary string that a client may assign. It will be passed back if there is an error with the event, but the corresponding `session.updated` event will not include it."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeClientEventSessionUpdateEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeClientEventSessionUpdateEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeClientEventSessionUpdateEventId> for ::std::string::String {
    fn from(value: RealtimeClientEventSessionUpdateEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeClientEventSessionUpdateEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventSessionUpdateEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeClientEventSessionUpdateEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeClientEventSessionUpdateEventId {
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
#[doc = "Update the Realtime session. Choose either a realtime\nsession or a transcription session.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeClientEventSessionUpdateSession {
    SessionCreateRequestGa(RealtimeSessionCreateRequestGa),
    TranscriptionSessionCreateRequestGa(RealtimeTranscriptionSessionCreateRequestGa),
}
impl ::std::convert::From<RealtimeSessionCreateRequestGa>
    for RealtimeClientEventSessionUpdateSession
{
    fn from(value: RealtimeSessionCreateRequestGa) -> Self {
        Self::SessionCreateRequestGa(value)
    }
}
impl ::std::convert::From<RealtimeTranscriptionSessionCreateRequestGa>
    for RealtimeClientEventSessionUpdateSession
{
    fn from(value: RealtimeTranscriptionSessionCreateRequestGa) -> Self {
        Self::TranscriptionSessionCreateRequestGa(value)
    }
}
#[doc = "The event type, must be `session.update`."]
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
pub enum RealtimeClientEventSessionUpdateType {
    #[serde(rename = "session.update")]
    SessionUpdate,
}
impl ::std::fmt::Display for RealtimeClientEventSessionUpdateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionUpdate => f.write_str("session.update"),
        }
    }
}
impl ::std::str::FromStr for RealtimeClientEventSessionUpdateType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.update" => Ok(Self::SessionUpdate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeClientEventSessionUpdateType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeClientEventSessionUpdateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A single item within a Realtime conversation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeConversationItem {
    ConversationItemMessageSystem(RealtimeConversationItemMessageSystem),
    ConversationItemMessageUser(RealtimeConversationItemMessageUser),
    ConversationItemMessageAssistant(RealtimeConversationItemMessageAssistant),
    ConversationItemFunctionCall(RealtimeConversationItemFunctionCall),
    ConversationItemFunctionCallOutput(RealtimeConversationItemFunctionCallOutput),
    McpApprovalResponse(RealtimeMcpApprovalResponse),
    McpListTools(RealtimeMcpListTools),
    McpToolCall(RealtimeMcpToolCall),
    McpApprovalRequest(RealtimeMcpApprovalRequest),
}
impl ::std::convert::From<RealtimeConversationItemMessageSystem> for RealtimeConversationItem {
    fn from(value: RealtimeConversationItemMessageSystem) -> Self {
        Self::ConversationItemMessageSystem(value)
    }
}
impl ::std::convert::From<RealtimeConversationItemMessageUser> for RealtimeConversationItem {
    fn from(value: RealtimeConversationItemMessageUser) -> Self {
        Self::ConversationItemMessageUser(value)
    }
}
impl ::std::convert::From<RealtimeConversationItemMessageAssistant> for RealtimeConversationItem {
    fn from(value: RealtimeConversationItemMessageAssistant) -> Self {
        Self::ConversationItemMessageAssistant(value)
    }
}
impl ::std::convert::From<RealtimeConversationItemFunctionCall> for RealtimeConversationItem {
    fn from(value: RealtimeConversationItemFunctionCall) -> Self {
        Self::ConversationItemFunctionCall(value)
    }
}
impl ::std::convert::From<RealtimeConversationItemFunctionCallOutput> for RealtimeConversationItem {
    fn from(value: RealtimeConversationItemFunctionCallOutput) -> Self {
        Self::ConversationItemFunctionCallOutput(value)
    }
}
impl ::std::convert::From<RealtimeMcpApprovalResponse> for RealtimeConversationItem {
    fn from(value: RealtimeMcpApprovalResponse) -> Self {
        Self::McpApprovalResponse(value)
    }
}
impl ::std::convert::From<RealtimeMcpListTools> for RealtimeConversationItem {
    fn from(value: RealtimeMcpListTools) -> Self {
        Self::McpListTools(value)
    }
}
impl ::std::convert::From<RealtimeMcpToolCall> for RealtimeConversationItem {
    fn from(value: RealtimeMcpToolCall) -> Self {
        Self::McpToolCall(value)
    }
}
impl ::std::convert::From<RealtimeMcpApprovalRequest> for RealtimeConversationItem {
    fn from(value: RealtimeMcpApprovalRequest) -> Self {
        Self::McpApprovalRequest(value)
    }
}
#[doc = "A function call item in a Realtime conversation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeConversationItemFunctionCall {
    #[doc = "The arguments of the function call. This is a JSON-encoded string representing the arguments passed to the function, for example `{\"arg1\": \"value1\", \"arg2\": 42}`."]
    pub arguments: ::std::string::String,
    #[doc = "The ID of the function call."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub call_id: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the item. This may be provided by the client or generated by the server."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function being called."]
    pub name: ::std::string::String,
    #[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<RealtimeConversationItemFunctionCallObject>,
    #[doc = "The status of the item. Has no effect on the conversation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<RealtimeConversationItemFunctionCallStatus>,
    #[doc = "The type of the item. Always `function_call`."]
    #[serde(rename = "type")]
    pub type_: RealtimeConversationItemFunctionCallType,
}
#[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
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
pub enum RealtimeConversationItemFunctionCallObject {
    #[serde(rename = "realtime.item")]
    RealtimeItem,
}
impl ::std::fmt::Display for RealtimeConversationItemFunctionCallObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RealtimeItem => f.write_str("realtime.item"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemFunctionCallObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "realtime.item" => Ok(Self::RealtimeItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemFunctionCallObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationItemFunctionCallObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A function call output item in a Realtime conversation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeConversationItemFunctionCallOutput {
    #[doc = "The ID of the function call this output is for."]
    pub call_id: ::std::string::String,
    #[doc = "The unique ID of the item. This may be provided by the client or generated by the server."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<RealtimeConversationItemFunctionCallOutputObject>,
    #[doc = "The output of the function call, this is free text and can contain any information or simply be empty."]
    pub output: ::std::string::String,
    #[doc = "The status of the item. Has no effect on the conversation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<RealtimeConversationItemFunctionCallOutputStatus>,
    #[doc = "The type of the item. Always `function_call_output`."]
    #[serde(rename = "type")]
    pub type_: RealtimeConversationItemFunctionCallOutputType,
}
#[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
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
pub enum RealtimeConversationItemFunctionCallOutputObject {
    #[serde(rename = "realtime.item")]
    RealtimeItem,
}
impl ::std::fmt::Display for RealtimeConversationItemFunctionCallOutputObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RealtimeItem => f.write_str("realtime.item"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemFunctionCallOutputObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "realtime.item" => Ok(Self::RealtimeItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemFunctionCallOutputObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemFunctionCallOutputObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the item. Has no effect on the conversation."]
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
pub enum RealtimeConversationItemFunctionCallOutputStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "in_progress")]
    InProgress,
}
impl ::std::fmt::Display for RealtimeConversationItemFunctionCallOutputStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::InProgress => f.write_str("in_progress"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemFunctionCallOutputStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "in_progress" => Ok(Self::InProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemFunctionCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemFunctionCallOutputStatus
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `function_call_output`."]
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
pub enum RealtimeConversationItemFunctionCallOutputType {
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}
impl ::std::fmt::Display for RealtimeConversationItemFunctionCallOutputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FunctionCallOutput => f.write_str("function_call_output"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemFunctionCallOutputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function_call_output" => Ok(Self::FunctionCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemFunctionCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemFunctionCallOutputType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the item. Has no effect on the conversation."]
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
pub enum RealtimeConversationItemFunctionCallStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "in_progress")]
    InProgress,
}
impl ::std::fmt::Display for RealtimeConversationItemFunctionCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::InProgress => f.write_str("in_progress"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemFunctionCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "in_progress" => Ok(Self::InProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemFunctionCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationItemFunctionCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `function_call`."]
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
pub enum RealtimeConversationItemFunctionCallType {
    #[serde(rename = "function_call")]
    FunctionCall,
}
impl ::std::fmt::Display for RealtimeConversationItemFunctionCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FunctionCall => f.write_str("function_call"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemFunctionCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function_call" => Ok(Self::FunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemFunctionCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationItemFunctionCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An assistant message item in a Realtime conversation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeConversationItemMessageAssistant {
    #[doc = "The content of the message."]
    pub content: ::std::vec::Vec<RealtimeConversationItemMessageAssistantContentItem>,
    #[doc = "The unique ID of the item. This may be provided by the client or generated by the server."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<RealtimeConversationItemMessageAssistantObject>,
    #[doc = "The role of the message sender. Always `assistant`."]
    pub role: RealtimeConversationItemMessageAssistantRole,
    #[doc = "The status of the item. Has no effect on the conversation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<RealtimeConversationItemMessageAssistantStatus>,
    #[doc = "The type of the item. Always `message`."]
    #[serde(rename = "type")]
    pub type_: RealtimeConversationItemMessageAssistantType,
}
#[doc = "`RealtimeConversationItemMessageAssistantContentItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeConversationItemMessageAssistantContentItem {
    #[doc = "Base64-encoded audio bytes, these will be parsed as the format specified in the session output audio type configuration. This defaults to PCM 16-bit 24kHz mono if not specified."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<::std::string::String>,
    #[doc = "The text content."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "The transcript of the audio content, this will always be present if the output type is `audio`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcript: ::std::option::Option<::std::string::String>,
    #[doc = "The content type, `output_text` or `output_audio` depending on the session `output_modalities` configuration."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RealtimeConversationItemMessageAssistantContentItemType>,
}
#[doc = "The content type, `output_text` or `output_audio` depending on the session `output_modalities` configuration."]
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
pub enum RealtimeConversationItemMessageAssistantContentItemType {
    #[serde(rename = "output_text")]
    OutputText,
    #[serde(rename = "output_audio")]
    OutputAudio,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageAssistantContentItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutputText => f.write_str("output_text"),
            Self::OutputAudio => f.write_str("output_audio"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageAssistantContentItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "output_text" => Ok(Self::OutputText),
            "output_audio" => Ok(Self::OutputAudio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageAssistantContentItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageAssistantContentItemType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
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
pub enum RealtimeConversationItemMessageAssistantObject {
    #[serde(rename = "realtime.item")]
    RealtimeItem,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageAssistantObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RealtimeItem => f.write_str("realtime.item"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageAssistantObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "realtime.item" => Ok(Self::RealtimeItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageAssistantObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageAssistantObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The role of the message sender. Always `assistant`."]
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
pub enum RealtimeConversationItemMessageAssistantRole {
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageAssistantRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageAssistantRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageAssistantRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageAssistantRole
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the item. Has no effect on the conversation."]
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
pub enum RealtimeConversationItemMessageAssistantStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "in_progress")]
    InProgress,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageAssistantStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::InProgress => f.write_str("in_progress"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageAssistantStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "in_progress" => Ok(Self::InProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageAssistantStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageAssistantStatus
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `message`."]
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
pub enum RealtimeConversationItemMessageAssistantType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageAssistantType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageAssistantType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageAssistantType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageAssistantType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A system message in a Realtime conversation can be used to provide additional context or instructions to the model. This is similar but distinct from the instruction prompt provided at the start of a conversation, as system messages can be added at any point in the conversation. For major changes to the conversation's behavior, use instructions, but for smaller updates (e.g. \"the user is now asking about a different topic\"), use system messages."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeConversationItemMessageSystem {
    #[doc = "The content of the message."]
    pub content: ::std::vec::Vec<RealtimeConversationItemMessageSystemContentItem>,
    #[doc = "The unique ID of the item. This may be provided by the client or generated by the server."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<RealtimeConversationItemMessageSystemObject>,
    #[doc = "The role of the message sender. Always `system`."]
    pub role: RealtimeConversationItemMessageSystemRole,
    #[doc = "The status of the item. Has no effect on the conversation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<RealtimeConversationItemMessageSystemStatus>,
    #[doc = "The type of the item. Always `message`."]
    #[serde(rename = "type")]
    pub type_: RealtimeConversationItemMessageSystemType,
}
#[doc = "`RealtimeConversationItemMessageSystemContentItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeConversationItemMessageSystemContentItem {
    #[doc = "The text content."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "The content type. Always `input_text` for system messages."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RealtimeConversationItemMessageSystemContentItemType>,
}
#[doc = "The content type. Always `input_text` for system messages."]
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
pub enum RealtimeConversationItemMessageSystemContentItemType {
    #[serde(rename = "input_text")]
    InputText,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageSystemContentItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputText => f.write_str("input_text"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageSystemContentItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_text" => Ok(Self::InputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageSystemContentItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageSystemContentItemType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
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
pub enum RealtimeConversationItemMessageSystemObject {
    #[serde(rename = "realtime.item")]
    RealtimeItem,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageSystemObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RealtimeItem => f.write_str("realtime.item"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageSystemObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "realtime.item" => Ok(Self::RealtimeItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageSystemObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageSystemObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The role of the message sender. Always `system`."]
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
pub enum RealtimeConversationItemMessageSystemRole {
    #[serde(rename = "system")]
    System,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageSystemRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::System => f.write_str("system"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageSystemRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "system" => Ok(Self::System),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageSystemRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationItemMessageSystemRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the item. Has no effect on the conversation."]
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
pub enum RealtimeConversationItemMessageSystemStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "in_progress")]
    InProgress,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageSystemStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::InProgress => f.write_str("in_progress"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageSystemStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "in_progress" => Ok(Self::InProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageSystemStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageSystemStatus
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `message`."]
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
pub enum RealtimeConversationItemMessageSystemType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageSystemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageSystemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageSystemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationItemMessageSystemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A user message item in a Realtime conversation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeConversationItemMessageUser {
    #[doc = "The content of the message."]
    pub content: ::std::vec::Vec<RealtimeConversationItemMessageUserContentItem>,
    #[doc = "The unique ID of the item. This may be provided by the client or generated by the server."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<RealtimeConversationItemMessageUserObject>,
    #[doc = "The role of the message sender. Always `user`."]
    pub role: RealtimeConversationItemMessageUserRole,
    #[doc = "The status of the item. Has no effect on the conversation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<RealtimeConversationItemMessageUserStatus>,
    #[doc = "The type of the item. Always `message`."]
    #[serde(rename = "type")]
    pub type_: RealtimeConversationItemMessageUserType,
}
#[doc = "`RealtimeConversationItemMessageUserContentItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeConversationItemMessageUserContentItem {
    #[doc = "Base64-encoded audio bytes (for `input_audio`), these will be parsed as the format specified in the session input audio type configuration. This defaults to PCM 16-bit 24kHz mono if not specified."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<::std::string::String>,
    #[doc = "The detail level of the image (for `input_image`). `auto` will default to `high`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<RealtimeConversationItemMessageUserContentItemDetail>,
    #[doc = "Base64-encoded image bytes (for `input_image`) as a data URI. For example `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA...`. Supported formats are PNG and JPEG."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[doc = "The text content (for `input_text`)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "Transcript of the audio (for `input_audio`). This is not sent to the model, but will be attached to the message item for reference."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcript: ::std::option::Option<::std::string::String>,
    #[doc = "The content type (`input_text`, `input_audio`, or `input_image`)."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RealtimeConversationItemMessageUserContentItemType>,
}
#[doc = "The detail level of the image (for `input_image`). `auto` will default to `high`."]
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
pub enum RealtimeConversationItemMessageUserContentItemDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageUserContentItemDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageUserContentItemDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageUserContentItemDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageUserContentItemDetail
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The content type (`input_text`, `input_audio`, or `input_image`)."]
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
pub enum RealtimeConversationItemMessageUserContentItemType {
    #[serde(rename = "input_text")]
    InputText,
    #[serde(rename = "input_audio")]
    InputAudio,
    #[serde(rename = "input_image")]
    InputImage,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageUserContentItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputText => f.write_str("input_text"),
            Self::InputAudio => f.write_str("input_audio"),
            Self::InputImage => f.write_str("input_image"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageUserContentItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_text" => Ok(Self::InputText),
            "input_audio" => Ok(Self::InputAudio),
            "input_image" => Ok(Self::InputImage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageUserContentItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeConversationItemMessageUserContentItemType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier for the API object being returned - always `realtime.item`. Optional when creating a new item."]
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
pub enum RealtimeConversationItemMessageUserObject {
    #[serde(rename = "realtime.item")]
    RealtimeItem,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageUserObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RealtimeItem => f.write_str("realtime.item"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageUserObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "realtime.item" => Ok(Self::RealtimeItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageUserObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationItemMessageUserObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The role of the message sender. Always `user`."]
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
pub enum RealtimeConversationItemMessageUserRole {
    #[serde(rename = "user")]
    User,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageUserRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageUserRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageUserRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationItemMessageUserRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the item. Has no effect on the conversation."]
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
pub enum RealtimeConversationItemMessageUserStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "in_progress")]
    InProgress,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageUserStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::InProgress => f.write_str("in_progress"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageUserStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "in_progress" => Ok(Self::InProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageUserStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationItemMessageUserStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `message`."]
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
pub enum RealtimeConversationItemMessageUserType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for RealtimeConversationItemMessageUserType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationItemMessageUserType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationItemMessageUserType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationItemMessageUserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Create a session and client secret for the Realtime API. The request can specify\neither a realtime or a transcription session configuration.\n[Learn more about the Realtime API](https://developers.openai.com/api/docs/guides/realtime).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeCreateClientSecretRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<ClientSecretExpiration>,
    #[doc = "Session configuration to use for the client secret. Choose either a realtime\nsession or a transcription session.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub session: ::std::option::Option<SessionConfiguration>,
}
#[doc = "Response from creating a session and client secret for the Realtime API.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeCreateClientSecretResponse {
    #[doc = "Expiration timestamp for the client secret, in seconds since epoch."]
    pub expires_at: i64,
    #[doc = "The session configuration for either a realtime or transcription session.\n"]
    pub session: SessionConfiguration,
    #[doc = "The generated client secret value."]
    pub value: ::std::string::String,
}
#[doc = "`RealtimeFunctionTool`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeFunctionTool {
    #[doc = "The description of the function, including guidance on when and how\nto call it, and guidance about what to tell the user when calling\n(if anything).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "Parameters of the function in JSON Schema."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The type of the tool, i.e. `function`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RealtimeFunctionToolType>,
}
#[doc = "The type of the tool, i.e. `function`."]
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
pub enum RealtimeFunctionToolType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for RealtimeFunctionToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for RealtimeFunctionToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeFunctionToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeFunctionToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A Realtime item requesting human approval of a tool invocation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeMcpApprovalRequest {
    #[doc = "A JSON string of arguments for the tool."]
    pub arguments: ::std::string::String,
    #[doc = "The unique ID of the approval request."]
    pub id: ::std::string::String,
    #[doc = "The name of the tool to run."]
    pub name: ::std::string::String,
    #[doc = "The label of the MCP server making the request."]
    pub server_label: ::std::string::String,
    #[doc = "The type of the item. Always `mcp_approval_request`."]
    #[serde(rename = "type")]
    pub type_: RealtimeMcpApprovalRequestType,
}
#[doc = "The type of the item. Always `mcp_approval_request`."]
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
pub enum RealtimeMcpApprovalRequestType {
    #[serde(rename = "mcp_approval_request")]
    McpApprovalRequest,
}
impl ::std::fmt::Display for RealtimeMcpApprovalRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpApprovalRequest => f.write_str("mcp_approval_request"),
        }
    }
}
impl ::std::str::FromStr for RealtimeMcpApprovalRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_approval_request" => Ok(Self::McpApprovalRequest),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeMcpApprovalRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeMcpApprovalRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A Realtime item responding to an MCP approval request.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeMcpApprovalResponse {
    #[doc = "The ID of the approval request being answered."]
    pub approval_request_id: ::std::string::String,
    #[doc = "Whether the request was approved."]
    pub approve: bool,
    #[doc = "The unique ID of the approval response."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the item. Always `mcp_approval_response`."]
    #[serde(rename = "type")]
    pub type_: RealtimeMcpApprovalResponseType,
}
#[doc = "The type of the item. Always `mcp_approval_response`."]
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
pub enum RealtimeMcpApprovalResponseType {
    #[serde(rename = "mcp_approval_response")]
    McpApprovalResponse,
}
impl ::std::fmt::Display for RealtimeMcpApprovalResponseType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpApprovalResponse => f.write_str("mcp_approval_response"),
        }
    }
}
impl ::std::str::FromStr for RealtimeMcpApprovalResponseType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_approval_response" => Ok(Self::McpApprovalResponse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeMcpApprovalResponseType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeMcpApprovalResponseType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A Realtime item listing tools available on an MCP server.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeMcpListTools {
    #[doc = "The unique ID of the list."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The label of the MCP server."]
    pub server_label: ::std::string::String,
    #[doc = "The tools available on the server."]
    pub tools: ::std::vec::Vec<McpListToolsTool>,
    #[doc = "The type of the item. Always `mcp_list_tools`."]
    #[serde(rename = "type")]
    pub type_: RealtimeMcpListToolsType,
}
#[doc = "The type of the item. Always `mcp_list_tools`."]
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
pub enum RealtimeMcpListToolsType {
    #[serde(rename = "mcp_list_tools")]
    McpListTools,
}
impl ::std::fmt::Display for RealtimeMcpListToolsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpListTools => f.write_str("mcp_list_tools"),
        }
    }
}
impl ::std::str::FromStr for RealtimeMcpListToolsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_list_tools" => Ok(Self::McpListTools),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeMcpListToolsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeMcpListToolsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeMcpProtocolError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeMcpProtocolError {
    pub code: i64,
    pub message: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: RealtimeMcpProtocolErrorType,
}
#[doc = "`RealtimeMcpProtocolErrorType`"]
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
pub enum RealtimeMcpProtocolErrorType {
    #[serde(rename = "protocol_error")]
    ProtocolError,
}
impl ::std::fmt::Display for RealtimeMcpProtocolErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProtocolError => f.write_str("protocol_error"),
        }
    }
}
impl ::std::str::FromStr for RealtimeMcpProtocolErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "protocol_error" => Ok(Self::ProtocolError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeMcpProtocolErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeMcpProtocolErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A Realtime item representing an invocation of a tool on an MCP server.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeMcpToolCall {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub approval_request_id: ::std::option::Option<::std::string::String>,
    #[doc = "A JSON string of the arguments passed to the tool."]
    pub arguments: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<RealtimeMcpToolCallError>,
    #[doc = "The unique ID of the tool call."]
    pub id: ::std::string::String,
    #[doc = "The name of the tool that was run."]
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<::std::string::String>,
    #[doc = "The label of the MCP server running the tool."]
    pub server_label: ::std::string::String,
    #[doc = "The type of the item. Always `mcp_call`."]
    #[serde(rename = "type")]
    pub type_: RealtimeMcpToolCallType,
}
#[doc = "The error from the tool call, if any."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeMcpToolCallError {
    McpProtocolError(RealtimeMcpProtocolError),
    McpToolExecutionError(RealtimeMcpToolExecutionError),
    McphttpError(RealtimeMcphttpError),
}
impl ::std::convert::From<RealtimeMcpProtocolError> for RealtimeMcpToolCallError {
    fn from(value: RealtimeMcpProtocolError) -> Self {
        Self::McpProtocolError(value)
    }
}
impl ::std::convert::From<RealtimeMcpToolExecutionError> for RealtimeMcpToolCallError {
    fn from(value: RealtimeMcpToolExecutionError) -> Self {
        Self::McpToolExecutionError(value)
    }
}
impl ::std::convert::From<RealtimeMcphttpError> for RealtimeMcpToolCallError {
    fn from(value: RealtimeMcphttpError) -> Self {
        Self::McphttpError(value)
    }
}
#[doc = "The type of the item. Always `mcp_call`."]
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
pub enum RealtimeMcpToolCallType {
    #[serde(rename = "mcp_call")]
    McpCall,
}
impl ::std::fmt::Display for RealtimeMcpToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpCall => f.write_str("mcp_call"),
        }
    }
}
impl ::std::str::FromStr for RealtimeMcpToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_call" => Ok(Self::McpCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeMcpToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeMcpToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeMcpToolExecutionError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeMcpToolExecutionError {
    pub message: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: RealtimeMcpToolExecutionErrorType,
}
#[doc = "`RealtimeMcpToolExecutionErrorType`"]
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
pub enum RealtimeMcpToolExecutionErrorType {
    #[serde(rename = "tool_execution_error")]
    ToolExecutionError,
}
impl ::std::fmt::Display for RealtimeMcpToolExecutionErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ToolExecutionError => f.write_str("tool_execution_error"),
        }
    }
}
impl ::std::str::FromStr for RealtimeMcpToolExecutionErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool_execution_error" => Ok(Self::ToolExecutionError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeMcpToolExecutionErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeMcpToolExecutionErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeMcphttpError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeMcphttpError {
    pub code: i64,
    pub message: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: RealtimeMcphttpErrorType,
}
#[doc = "`RealtimeMcphttpErrorType`"]
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
pub enum RealtimeMcphttpErrorType {
    #[serde(rename = "http_error")]
    HttpError,
}
impl ::std::fmt::Display for RealtimeMcphttpErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::HttpError => f.write_str("http_error"),
        }
    }
}
impl ::std::str::FromStr for RealtimeMcphttpErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "http_error" => Ok(Self::HttpError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeMcphttpErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeMcphttpErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for reasoning-capable Realtime models such as `gpt-realtime-2`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeReasoning {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub effort: ::std::option::Option<RealtimeReasoningEffort>,
}
#[doc = "Constrains effort on reasoning for reasoning-capable Realtime models such as\n`gpt-realtime-2`.\n"]
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
pub enum RealtimeReasoningEffort {
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
}
impl ::std::fmt::Display for RealtimeReasoningEffort {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Minimal => f.write_str("minimal"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
        }
    }
}
impl ::std::str::FromStr for RealtimeReasoningEffort {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "minimal" => Ok(Self::Minimal),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The response resource."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponse {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<RealtimeResponseAudio>,
    #[doc = "Which conversation the response is added to, determined by the `conversation`\nfield in the `response.create` event. If `auto`, the response will be added to\nthe default conversation and the value of `conversation_id` will be an id like\n`conv_1234`. If `none`, the response will not be added to any conversation and\nthe value of `conversation_id` will be `null`. If responses are being triggered\nautomatically by VAD the response will be added to the default conversation\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub conversation_id: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the response, will look like `resp_1234`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls, that was used in this response.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_tokens: ::std::option::Option<RealtimeResponseMaxOutputTokens>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[doc = "The object type, must be `realtime.response`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<RealtimeResponseObject>,
    #[doc = "The list of output items generated by the response."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub output: ::std::vec::Vec<RealtimeConversationItem>,
    #[doc = "The set of modalities the model used to respond, currently the only possible values are\n`[\\\"audio\\\"]`, `[\\\"text\\\"]`. Audio output always include a text transcript. Setting the\noutput to mode `text` will disable audio output from the model.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub output_modalities: ::std::vec::Vec<RealtimeResponseOutputModalitiesItem>,
    #[doc = "The final status of the response (`completed`, `cancelled`, `failed`, or \n`incomplete`, `in_progress`).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<RealtimeResponseStatus>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status_details: ::std::option::Option<RealtimeResponseStatusDetails>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<RealtimeResponseUsage>,
}
#[doc = "Configuration for audio output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<RealtimeResponseAudioOutput>,
}
#[doc = "`RealtimeResponseAudioOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseAudioOutput {
    #[doc = "The format of the output audio."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the\nsession once the model has responded with audio at least once. Current\nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`,\n`shimmer`, `verse`, `marin`, and `cedar`. We recommend `marin` and `cedar` for\nbest quality.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceIdsShared>,
}
#[doc = "Create a new Realtime response with these parameters"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseCreateParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<RealtimeResponseCreateParamsAudio>,
    #[doc = "Controls which conversation the response is added to. Currently supports\n`auto` and `none`, with `auto` as the default value. The `auto` value\nmeans that the contents of the response will be added to the default\nconversation. Set this to `none` to create an out-of-band response which\nwill not add items to default conversation.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub conversation: ::std::option::Option<::std::string::String>,
    #[doc = "Input items to include in the prompt for the model. Using this field\ncreates a new context for this Response instead of using the default\nconversation. An empty array `[]` will clear the context for this Response.\nNote that this can include references to items that previously appeared in the session\nusing their id.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub input: ::std::vec::Vec<RealtimeConversationItem>,
    #[doc = "The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.\nNote that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_tokens: ::std::option::Option<RealtimeResponseCreateParamsMaxOutputTokens>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[doc = "The set of modalities the model used to respond, currently the only possible values are\n`[\\\"audio\\\"]`, `[\\\"text\\\"]`. Audio output always include a text transcript. Setting the\noutput to mode `text` will disable audio output from the model.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub output_modalities: ::std::vec::Vec<RealtimeResponseCreateParamsOutputModalitiesItem>,
    #[doc = "Whether the model may call multiple tools in parallel. Only supported by\nreasoning Realtime models such as `gpt-realtime-2`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parallel_tool_calls: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<Prompt>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<RealtimeReasoning>,
    #[doc = "How the model chooses tools. Provide one of the string modes or force a specific\nfunction/MCP tool.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<RealtimeResponseCreateParamsToolChoice>,
    #[doc = "Tools available to the model."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<RealtimeResponseCreateParamsToolsItem>,
}
#[doc = "Configuration for audio input and output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseCreateParamsAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<RealtimeResponseCreateParamsAudioOutput>,
}
#[doc = "`RealtimeResponseCreateParamsAudioOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseCreateParamsAudioOutput {
    #[doc = "The format of the output audio."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[doc = "The voice the model uses to respond. Supported built-in voices are\n`alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`, `shimmer`, `verse`,\n`marin`, and `cedar`. You may also provide a custom voice object with\nan `id`, for example `{ \"id\": \"voice_1234\" }`. Voice cannot be changed\nduring the session once the model has responded with audio at least once.\nCustom voices must be created from audio samples.\nWe recommend `marin` and `cedar` for best quality.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceIdsOrCustomVoice>,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsMaxOutputTokens {
    Integer(i64),
    String(RealtimeResponseCreateParamsMaxOutputTokensString),
}
impl ::std::fmt::Display for RealtimeResponseCreateParamsMaxOutputTokens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseCreateParamsMaxOutputTokens {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseCreateParamsMaxOutputTokens {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeResponseCreateParamsMaxOutputTokens
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<i64> for RealtimeResponseCreateParamsMaxOutputTokens {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<RealtimeResponseCreateParamsMaxOutputTokensString>
    for RealtimeResponseCreateParamsMaxOutputTokens
{
    fn from(value: RealtimeResponseCreateParamsMaxOutputTokensString) -> Self {
        Self::String(value)
    }
}
#[doc = "`RealtimeResponseCreateParamsMaxOutputTokensString`"]
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
pub enum RealtimeResponseCreateParamsMaxOutputTokensString {
    #[serde(rename = "inf")]
    Inf,
}
impl ::std::fmt::Display for RealtimeResponseCreateParamsMaxOutputTokensString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inf => f.write_str("inf"),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseCreateParamsMaxOutputTokensString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inf" => Ok(Self::Inf),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseCreateParamsMaxOutputTokensString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeResponseCreateParamsMaxOutputTokensString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeResponseCreateParamsOutputModalitiesItem`"]
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
pub enum RealtimeResponseCreateParamsOutputModalitiesItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
impl ::std::fmt::Display for RealtimeResponseCreateParamsOutputModalitiesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Audio => f.write_str("audio"),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseCreateParamsOutputModalitiesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseCreateParamsOutputModalitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeResponseCreateParamsOutputModalitiesItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "How the model chooses tools. Provide one of the string modes or force a specific\nfunction/MCP tool.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsToolChoice {
    Options(ToolChoiceOptions),
    Function(ToolChoiceFunction),
    Mcp(ToolChoiceMcp),
}
impl ::std::convert::From<ToolChoiceOptions> for RealtimeResponseCreateParamsToolChoice {
    fn from(value: ToolChoiceOptions) -> Self {
        Self::Options(value)
    }
}
impl ::std::convert::From<ToolChoiceFunction> for RealtimeResponseCreateParamsToolChoice {
    fn from(value: ToolChoiceFunction) -> Self {
        Self::Function(value)
    }
}
impl ::std::convert::From<ToolChoiceMcp> for RealtimeResponseCreateParamsToolChoice {
    fn from(value: ToolChoiceMcp) -> Self {
        Self::Mcp(value)
    }
}
#[doc = "`RealtimeResponseCreateParamsToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsToolsItem {
    RealtimeFunctionTool(RealtimeFunctionTool),
    McpTool(McpTool),
}
impl ::std::convert::From<RealtimeFunctionTool> for RealtimeResponseCreateParamsToolsItem {
    fn from(value: RealtimeFunctionTool) -> Self {
        Self::RealtimeFunctionTool(value)
    }
}
impl ::std::convert::From<McpTool> for RealtimeResponseCreateParamsToolsItem {
    fn from(value: McpTool) -> Self {
        Self::McpTool(value)
    }
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls, that was used in this response.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeResponseMaxOutputTokens {
    Integer(i64),
    String(RealtimeResponseMaxOutputTokensString),
}
impl ::std::fmt::Display for RealtimeResponseMaxOutputTokens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseMaxOutputTokens {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseMaxOutputTokens {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeResponseMaxOutputTokens {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<i64> for RealtimeResponseMaxOutputTokens {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<RealtimeResponseMaxOutputTokensString>
    for RealtimeResponseMaxOutputTokens
{
    fn from(value: RealtimeResponseMaxOutputTokensString) -> Self {
        Self::String(value)
    }
}
#[doc = "`RealtimeResponseMaxOutputTokensString`"]
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
pub enum RealtimeResponseMaxOutputTokensString {
    #[serde(rename = "inf")]
    Inf,
}
impl ::std::fmt::Display for RealtimeResponseMaxOutputTokensString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inf => f.write_str("inf"),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseMaxOutputTokensString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inf" => Ok(Self::Inf),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseMaxOutputTokensString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeResponseMaxOutputTokensString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, must be `realtime.response`."]
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
pub enum RealtimeResponseObject {
    #[serde(rename = "realtime.response")]
    RealtimeResponse,
}
impl ::std::fmt::Display for RealtimeResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RealtimeResponse => f.write_str("realtime.response"),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "realtime.response" => Ok(Self::RealtimeResponse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeResponseOutputModalitiesItem`"]
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
pub enum RealtimeResponseOutputModalitiesItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
impl ::std::fmt::Display for RealtimeResponseOutputModalitiesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Audio => f.write_str("audio"),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseOutputModalitiesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseOutputModalitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeResponseOutputModalitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The final status of the response (`completed`, `cancelled`, `failed`, or \n`incomplete`, `in_progress`).\n"]
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
pub enum RealtimeResponseStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "in_progress")]
    InProgress,
}
impl ::std::fmt::Display for RealtimeResponseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Failed => f.write_str("failed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::InProgress => f.write_str("in_progress"),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            "incomplete" => Ok(Self::Incomplete),
            "in_progress" => Ok(Self::InProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Additional details about the status."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseStatusDetails {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<RealtimeResponseStatusDetailsError>,
    #[doc = "The reason the Response did not complete. For a `cancelled` Response,  one of `turn_detected` (the server VAD detected a new start of speech)  or `client_cancelled` (the client sent a cancel event). For an  `incomplete` Response, one of `max_output_tokens` or `content_filter`  (the server-side safety filter activated and cut off the response).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<RealtimeResponseStatusDetailsReason>,
    #[doc = "The type of error that caused the response to fail, corresponding \nwith the `status` field (`completed`, `cancelled`, `incomplete`, \n`failed`).\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RealtimeResponseStatusDetailsType>,
}
#[doc = "A description of the error that caused the response to fail, \npopulated when the `status` is `failed`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseStatusDetailsError {
    #[doc = "Error code, if any."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    #[doc = "The type of error."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
#[doc = "The reason the Response did not complete. For a `cancelled` Response,  one of `turn_detected` (the server VAD detected a new start of speech)  or `client_cancelled` (the client sent a cancel event). For an  `incomplete` Response, one of `max_output_tokens` or `content_filter`  (the server-side safety filter activated and cut off the response).\n"]
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
pub enum RealtimeResponseStatusDetailsReason {
    #[serde(rename = "turn_detected")]
    TurnDetected,
    #[serde(rename = "client_cancelled")]
    ClientCancelled,
    #[serde(rename = "max_output_tokens")]
    MaxOutputTokens,
    #[serde(rename = "content_filter")]
    ContentFilter,
}
impl ::std::fmt::Display for RealtimeResponseStatusDetailsReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TurnDetected => f.write_str("turn_detected"),
            Self::ClientCancelled => f.write_str("client_cancelled"),
            Self::MaxOutputTokens => f.write_str("max_output_tokens"),
            Self::ContentFilter => f.write_str("content_filter"),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseStatusDetailsReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "turn_detected" => Ok(Self::TurnDetected),
            "client_cancelled" => Ok(Self::ClientCancelled),
            "max_output_tokens" => Ok(Self::MaxOutputTokens),
            "content_filter" => Ok(Self::ContentFilter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseStatusDetailsReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeResponseStatusDetailsReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of error that caused the response to fail, corresponding \nwith the `status` field (`completed`, `cancelled`, `incomplete`, \n`failed`).\n"]
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
pub enum RealtimeResponseStatusDetailsType {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for RealtimeResponseStatusDetailsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Failed => f.write_str("failed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for RealtimeResponseStatusDetailsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeResponseStatusDetailsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeResponseStatusDetailsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Usage statistics for the Response, this will correspond to billing. A \nRealtime API session will maintain a conversation context and append new \nItems to the Conversation, thus output from previous turns (text and \naudio tokens) will become the input for later turns.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseUsage {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_token_details: ::std::option::Option<RealtimeResponseUsageInputTokenDetails>,
    #[doc = "The number of input tokens used in the Response, including text and \naudio tokens.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_token_details: ::std::option::Option<RealtimeResponseUsageOutputTokenDetails>,
    #[doc = "The number of output tokens sent in the Response, including text and \naudio tokens.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_tokens: ::std::option::Option<i64>,
    #[doc = "The total number of tokens in the Response including input and output \ntext and audio tokens.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub total_tokens: ::std::option::Option<i64>,
}
#[doc = "Details about the input tokens used in the Response. Cached tokens are tokens from previous turns in the conversation that are included as context for the current response. Cached tokens here are counted as a subset of input tokens, meaning input tokens will include cached and uncached tokens."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseUsageInputTokenDetails {
    #[doc = "The number of audio tokens used as input for the Response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_tokens: ::std::option::Option<i64>,
    #[doc = "The number of cached tokens used as input for the Response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cached_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cached_tokens_details:
        ::std::option::Option<RealtimeResponseUsageInputTokenDetailsCachedTokensDetails>,
    #[doc = "The number of image tokens used as input for the Response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_tokens: ::std::option::Option<i64>,
    #[doc = "The number of text tokens used as input for the Response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text_tokens: ::std::option::Option<i64>,
}
#[doc = "Details about the cached tokens used as input for the Response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseUsageInputTokenDetailsCachedTokensDetails {
    #[doc = "The number of cached audio tokens used as input for the Response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_tokens: ::std::option::Option<i64>,
    #[doc = "The number of cached image tokens used as input for the Response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_tokens: ::std::option::Option<i64>,
    #[doc = "The number of cached text tokens used as input for the Response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text_tokens: ::std::option::Option<i64>,
}
#[doc = "Details about the output tokens used in the Response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeResponseUsageOutputTokenDetails {
    #[doc = "The number of audio tokens used in the Response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_tokens: ::std::option::Option<i64>,
    #[doc = "The number of text tokens used in the Response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text_tokens: ::std::option::Option<i64>,
}
#[doc = "A realtime server event.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeServerEvent {
    ConversationCreated(RealtimeServerEventConversationCreated),
    ConversationItemCreated(RealtimeServerEventConversationItemCreated),
    ConversationItemDeleted(RealtimeServerEventConversationItemDeleted),
    ConversationItemInputAudioTranscriptionCompleted(
        RealtimeServerEventConversationItemInputAudioTranscriptionCompleted,
    ),
    ConversationItemInputAudioTranscriptionDelta(
        RealtimeServerEventConversationItemInputAudioTranscriptionDelta,
    ),
    ConversationItemInputAudioTranscriptionFailed(
        RealtimeServerEventConversationItemInputAudioTranscriptionFailed,
    ),
    ConversationItemRetrieved(RealtimeServerEventConversationItemRetrieved),
    ConversationItemTruncated(RealtimeServerEventConversationItemTruncated),
    Error(RealtimeServerEventError),
    InputAudioBufferCleared(RealtimeServerEventInputAudioBufferCleared),
    InputAudioBufferCommitted(RealtimeServerEventInputAudioBufferCommitted),
    InputAudioBufferDtmfEventReceived(RealtimeServerEventInputAudioBufferDtmfEventReceived),
    InputAudioBufferSpeechStarted(RealtimeServerEventInputAudioBufferSpeechStarted),
    InputAudioBufferSpeechStopped(RealtimeServerEventInputAudioBufferSpeechStopped),
    RateLimitsUpdated(RealtimeServerEventRateLimitsUpdated),
    ResponseAudioDelta(RealtimeServerEventResponseAudioDelta),
    ResponseAudioDone(RealtimeServerEventResponseAudioDone),
    ResponseAudioTranscriptDelta(RealtimeServerEventResponseAudioTranscriptDelta),
    ResponseAudioTranscriptDone(RealtimeServerEventResponseAudioTranscriptDone),
    ResponseContentPartAdded(RealtimeServerEventResponseContentPartAdded),
    ResponseContentPartDone(RealtimeServerEventResponseContentPartDone),
    ResponseCreated(RealtimeServerEventResponseCreated),
    ResponseDone(RealtimeServerEventResponseDone),
    ResponseFunctionCallArgumentsDelta(RealtimeServerEventResponseFunctionCallArgumentsDelta),
    ResponseFunctionCallArgumentsDone(RealtimeServerEventResponseFunctionCallArgumentsDone),
    ResponseOutputItemAdded(RealtimeServerEventResponseOutputItemAdded),
    ResponseOutputItemDone(RealtimeServerEventResponseOutputItemDone),
    ResponseTextDelta(RealtimeServerEventResponseTextDelta),
    ResponseTextDone(RealtimeServerEventResponseTextDone),
    SessionCreated(RealtimeServerEventSessionCreated),
    SessionUpdated(RealtimeServerEventSessionUpdated),
    OutputAudioBufferStarted(RealtimeServerEventOutputAudioBufferStarted),
    OutputAudioBufferStopped(RealtimeServerEventOutputAudioBufferStopped),
    OutputAudioBufferCleared(RealtimeServerEventOutputAudioBufferCleared),
    ConversationItemAdded(RealtimeServerEventConversationItemAdded),
    ConversationItemDone(RealtimeServerEventConversationItemDone),
    InputAudioBufferTimeoutTriggered(RealtimeServerEventInputAudioBufferTimeoutTriggered),
    ConversationItemInputAudioTranscriptionSegment(
        RealtimeServerEventConversationItemInputAudioTranscriptionSegment,
    ),
    McpListToolsInProgress(RealtimeServerEventMcpListToolsInProgress),
    McpListToolsCompleted(RealtimeServerEventMcpListToolsCompleted),
    McpListToolsFailed(RealtimeServerEventMcpListToolsFailed),
    ResponseMcpCallArgumentsDelta(RealtimeServerEventResponseMcpCallArgumentsDelta),
    ResponseMcpCallArgumentsDone(RealtimeServerEventResponseMcpCallArgumentsDone),
    ResponseMcpCallInProgress(RealtimeServerEventResponseMcpCallInProgress),
    ResponseMcpCallCompleted(RealtimeServerEventResponseMcpCallCompleted),
    ResponseMcpCallFailed(RealtimeServerEventResponseMcpCallFailed),
}
impl ::std::convert::From<RealtimeServerEventConversationCreated> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventConversationCreated) -> Self {
        Self::ConversationCreated(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemCreated> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventConversationItemCreated) -> Self {
        Self::ConversationItemCreated(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemDeleted> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventConversationItemDeleted) -> Self {
        Self::ConversationItemDeleted(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemInputAudioTranscriptionCompleted>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventConversationItemInputAudioTranscriptionCompleted) -> Self {
        Self::ConversationItemInputAudioTranscriptionCompleted(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemInputAudioTranscriptionDelta>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventConversationItemInputAudioTranscriptionDelta) -> Self {
        Self::ConversationItemInputAudioTranscriptionDelta(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemInputAudioTranscriptionFailed>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventConversationItemInputAudioTranscriptionFailed) -> Self {
        Self::ConversationItemInputAudioTranscriptionFailed(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemRetrieved> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventConversationItemRetrieved) -> Self {
        Self::ConversationItemRetrieved(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemTruncated> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventConversationItemTruncated) -> Self {
        Self::ConversationItemTruncated(value)
    }
}
impl ::std::convert::From<RealtimeServerEventError> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventError) -> Self {
        Self::Error(value)
    }
}
impl ::std::convert::From<RealtimeServerEventInputAudioBufferCleared> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventInputAudioBufferCleared) -> Self {
        Self::InputAudioBufferCleared(value)
    }
}
impl ::std::convert::From<RealtimeServerEventInputAudioBufferCommitted> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventInputAudioBufferCommitted) -> Self {
        Self::InputAudioBufferCommitted(value)
    }
}
impl ::std::convert::From<RealtimeServerEventInputAudioBufferDtmfEventReceived>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventInputAudioBufferDtmfEventReceived) -> Self {
        Self::InputAudioBufferDtmfEventReceived(value)
    }
}
impl ::std::convert::From<RealtimeServerEventInputAudioBufferSpeechStarted>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventInputAudioBufferSpeechStarted) -> Self {
        Self::InputAudioBufferSpeechStarted(value)
    }
}
impl ::std::convert::From<RealtimeServerEventInputAudioBufferSpeechStopped>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventInputAudioBufferSpeechStopped) -> Self {
        Self::InputAudioBufferSpeechStopped(value)
    }
}
impl ::std::convert::From<RealtimeServerEventRateLimitsUpdated> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventRateLimitsUpdated) -> Self {
        Self::RateLimitsUpdated(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseAudioDelta> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseAudioDelta) -> Self {
        Self::ResponseAudioDelta(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseAudioDone> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseAudioDone) -> Self {
        Self::ResponseAudioDone(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseAudioTranscriptDelta> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseAudioTranscriptDelta) -> Self {
        Self::ResponseAudioTranscriptDelta(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseAudioTranscriptDone> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseAudioTranscriptDone) -> Self {
        Self::ResponseAudioTranscriptDone(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseContentPartAdded> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseContentPartAdded) -> Self {
        Self::ResponseContentPartAdded(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseContentPartDone> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseContentPartDone) -> Self {
        Self::ResponseContentPartDone(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseCreated> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseCreated) -> Self {
        Self::ResponseCreated(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseDone> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseDone) -> Self {
        Self::ResponseDone(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseFunctionCallArgumentsDelta>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventResponseFunctionCallArgumentsDelta) -> Self {
        Self::ResponseFunctionCallArgumentsDelta(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseFunctionCallArgumentsDone>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventResponseFunctionCallArgumentsDone) -> Self {
        Self::ResponseFunctionCallArgumentsDone(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseOutputItemAdded> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseOutputItemAdded) -> Self {
        Self::ResponseOutputItemAdded(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseOutputItemDone> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseOutputItemDone) -> Self {
        Self::ResponseOutputItemDone(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseTextDelta> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseTextDelta) -> Self {
        Self::ResponseTextDelta(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseTextDone> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseTextDone) -> Self {
        Self::ResponseTextDone(value)
    }
}
impl ::std::convert::From<RealtimeServerEventSessionCreated> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventSessionCreated) -> Self {
        Self::SessionCreated(value)
    }
}
impl ::std::convert::From<RealtimeServerEventSessionUpdated> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventSessionUpdated) -> Self {
        Self::SessionUpdated(value)
    }
}
impl ::std::convert::From<RealtimeServerEventOutputAudioBufferStarted> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventOutputAudioBufferStarted) -> Self {
        Self::OutputAudioBufferStarted(value)
    }
}
impl ::std::convert::From<RealtimeServerEventOutputAudioBufferStopped> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventOutputAudioBufferStopped) -> Self {
        Self::OutputAudioBufferStopped(value)
    }
}
impl ::std::convert::From<RealtimeServerEventOutputAudioBufferCleared> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventOutputAudioBufferCleared) -> Self {
        Self::OutputAudioBufferCleared(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemAdded> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventConversationItemAdded) -> Self {
        Self::ConversationItemAdded(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemDone> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventConversationItemDone) -> Self {
        Self::ConversationItemDone(value)
    }
}
impl ::std::convert::From<RealtimeServerEventInputAudioBufferTimeoutTriggered>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventInputAudioBufferTimeoutTriggered) -> Self {
        Self::InputAudioBufferTimeoutTriggered(value)
    }
}
impl ::std::convert::From<RealtimeServerEventConversationItemInputAudioTranscriptionSegment>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventConversationItemInputAudioTranscriptionSegment) -> Self {
        Self::ConversationItemInputAudioTranscriptionSegment(value)
    }
}
impl ::std::convert::From<RealtimeServerEventMcpListToolsInProgress> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventMcpListToolsInProgress) -> Self {
        Self::McpListToolsInProgress(value)
    }
}
impl ::std::convert::From<RealtimeServerEventMcpListToolsCompleted> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventMcpListToolsCompleted) -> Self {
        Self::McpListToolsCompleted(value)
    }
}
impl ::std::convert::From<RealtimeServerEventMcpListToolsFailed> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventMcpListToolsFailed) -> Self {
        Self::McpListToolsFailed(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseMcpCallArgumentsDelta>
    for RealtimeServerEvent
{
    fn from(value: RealtimeServerEventResponseMcpCallArgumentsDelta) -> Self {
        Self::ResponseMcpCallArgumentsDelta(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseMcpCallArgumentsDone> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseMcpCallArgumentsDone) -> Self {
        Self::ResponseMcpCallArgumentsDone(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseMcpCallInProgress> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseMcpCallInProgress) -> Self {
        Self::ResponseMcpCallInProgress(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseMcpCallCompleted> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseMcpCallCompleted) -> Self {
        Self::ResponseMcpCallCompleted(value)
    }
}
impl ::std::convert::From<RealtimeServerEventResponseMcpCallFailed> for RealtimeServerEvent {
    fn from(value: RealtimeServerEventResponseMcpCallFailed) -> Self {
        Self::ResponseMcpCallFailed(value)
    }
}
#[doc = "Returned when a conversation is created. Emitted right after session creation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationCreated {
    pub conversation: RealtimeServerEventConversationCreatedConversation,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, must be `conversation.created`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationCreatedType,
}
#[doc = "The conversation resource."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeServerEventConversationCreatedConversation {
    #[doc = "The unique ID of the conversation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, must be `realtime.conversation`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<::std::string::String>,
}
#[doc = "The event type, must be `conversation.created`."]
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
pub enum RealtimeServerEventConversationCreatedType {
    #[serde(rename = "conversation.created")]
    ConversationCreated,
}
impl ::std::fmt::Display for RealtimeServerEventConversationCreatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationCreated => f.write_str("conversation.created"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationCreatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.created" => Ok(Self::ConversationCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventConversationCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventConversationCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Sent by the server when an Item is added to the default Conversation. This can happen in several cases:\n- When the client sends a `conversation.item.create` event.\n- When the input audio buffer is committed. In this case the item will be a user message containing the audio from the buffer.\n- When the model is generating a Response. In this case the `conversation.item.added` event will be sent when the model starts generating a specific Item, and thus it will not yet have any content (and `status` will be `in_progress`).\n\nThe event will include the full content of the Item (except when model is generating a Response) except for audio data, which can be retrieved separately with a `conversation.item.retrieve` event if necessary.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemAdded {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    pub item: RealtimeConversationItem,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_item_id: ::std::option::Option<::std::string::String>,
    #[doc = "The event type, must be `conversation.item.added`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemAddedType,
}
#[doc = "The event type, must be `conversation.item.added`."]
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
pub enum RealtimeServerEventConversationItemAddedType {
    #[serde(rename = "conversation.item.added")]
    ConversationItemAdded,
}
impl ::std::fmt::Display for RealtimeServerEventConversationItemAddedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemAdded => f.write_str("conversation.item.added"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationItemAddedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.added" => Ok(Self::ConversationItemAdded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventConversationItemAddedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemAddedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a conversation item is created. There are several scenarios that produce this event:\n  - The server is generating a Response, which if successful will produce\n    either one or two Items, which will be of type `message`\n    (role `assistant`) or type `function_call`.\n  - The input audio buffer has been committed, either by the client or the\n    server (in `server_vad` mode). The server will take the content of the\n    input audio buffer and add it to a new user message Item.\n  - The client has sent a `conversation.item.create` event to add a new Item\n    to the Conversation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemCreated {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    pub item: RealtimeConversationItem,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_item_id: ::std::option::Option<::std::string::String>,
    #[doc = "The event type, must be `conversation.item.created`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemCreatedType,
}
#[doc = "The event type, must be `conversation.item.created`."]
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
pub enum RealtimeServerEventConversationItemCreatedType {
    #[serde(rename = "conversation.item.created")]
    ConversationItemCreated,
}
impl ::std::fmt::Display for RealtimeServerEventConversationItemCreatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemCreated => f.write_str("conversation.item.created"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationItemCreatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.created" => Ok(Self::ConversationItemCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventConversationItemCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemCreatedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when an item in the conversation is deleted by the client with a \n`conversation.item.delete` event. This event is used to synchronize the \nserver's understanding of the conversation history with the client's view.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemDeleted {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item that was deleted."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `conversation.item.deleted`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemDeletedType,
}
#[doc = "The event type, must be `conversation.item.deleted`."]
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
pub enum RealtimeServerEventConversationItemDeletedType {
    #[serde(rename = "conversation.item.deleted")]
    ConversationItemDeleted,
}
impl ::std::fmt::Display for RealtimeServerEventConversationItemDeletedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemDeleted => f.write_str("conversation.item.deleted"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationItemDeletedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.deleted" => Ok(Self::ConversationItemDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventConversationItemDeletedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemDeletedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a conversation item is finalized.\n\nThe event will include the full content of the Item except for audio data, which can be retrieved separately with a `conversation.item.retrieve` event if needed.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemDone {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    pub item: RealtimeConversationItem,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_item_id: ::std::option::Option<::std::string::String>,
    #[doc = "The event type, must be `conversation.item.done`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemDoneType,
}
#[doc = "The event type, must be `conversation.item.done`."]
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
pub enum RealtimeServerEventConversationItemDoneType {
    #[serde(rename = "conversation.item.done")]
    ConversationItemDone,
}
impl ::std::fmt::Display for RealtimeServerEventConversationItemDoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemDone => f.write_str("conversation.item.done"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationItemDoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.done" => Ok(Self::ConversationItemDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventConversationItemDoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemDoneType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "This event is the output of audio transcription for user audio written to the\nuser audio buffer. Transcription begins when the input audio buffer is\ncommitted by the client or server (when VAD is enabled). Transcription runs\nasynchronously with Response creation, so this event may come before or after\nthe Response events.\n\nRealtime API models accept audio natively, and thus input transcription is a\nseparate process run on a separate ASR (Automatic Speech Recognition) model.\nThe transcript may diverge somewhat from the model's interpretation, and\nshould be treated as a rough guide.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
    #[doc = "The index of the content part containing the audio."]
    pub content_index: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item containing the audio that is being transcribed."]
    pub item_id: ::std::string::String,
    #[doc = "The languages detected in the audio. Returned by `gpt-transcribe`. An empty array indicates that no language could be reliably detected.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<TranscriptionLanguage>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logprobs: ::std::option::Option<::std::vec::Vec<LogProbProperties>>,
    #[doc = "The transcribed text."]
    pub transcript: ::std::string::String,
    #[doc = "The event type, must be\n`conversation.item.input_audio_transcription.completed`.\n"]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType,
    #[doc = "Usage statistics for the transcription, this is billed according to the ASR model's pricing rather than the realtime model's pricing."]
    pub usage: RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage,
}
#[doc = "The event type, must be\n`conversation.item.input_audio_transcription.completed`.\n"]
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
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType {
    #[serde(rename = "conversation.item.input_audio_transcription.completed")]
    ConversationItemInputAudioTranscriptionCompleted,
}
impl ::std::fmt::Display
    for RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemInputAudioTranscriptionCompleted => {
                f.write_str("conversation.item.input_audio_transcription.completed")
            }
        }
    }
}
impl ::std::str::FromStr
    for RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.input_audio_transcription.completed" => {
                Ok(Self::ConversationItemInputAudioTranscriptionCompleted)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Usage statistics for the transcription, this is billed according to the ASR model's pricing rather than the realtime model's pricing."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage {
    Tokens(TranscriptTextUsageTokens),
    Duration(TranscriptTextUsageDuration),
}
impl ::std::convert::From<TranscriptTextUsageTokens>
    for RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage
{
    fn from(value: TranscriptTextUsageTokens) -> Self {
        Self::Tokens(value)
    }
}
impl ::std::convert::From<TranscriptTextUsageDuration>
    for RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage
{
    fn from(value: TranscriptTextUsageDuration) -> Self {
        Self::Duration(value)
    }
}
#[doc = "Returned when the text value of an input audio transcription content part is updated with incremental transcription results.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta {
    #[doc = "The index of the content part in the item's content array."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content_index: ::std::option::Option<i64>,
    #[doc = "The text delta."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delta: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item containing the audio that is being transcribed."]
    pub item_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logprobs: ::std::option::Option<::std::vec::Vec<LogProbProperties>>,
    #[doc = "The event type, must be `conversation.item.input_audio_transcription.delta`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType,
}
#[doc = "The event type, must be `conversation.item.input_audio_transcription.delta`."]
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
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType {
    #[serde(rename = "conversation.item.input_audio_transcription.delta")]
    ConversationItemInputAudioTranscriptionDelta,
}
impl ::std::fmt::Display for RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemInputAudioTranscriptionDelta => {
                f.write_str("conversation.item.input_audio_transcription.delta")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.input_audio_transcription.delta" => {
                Ok(Self::ConversationItemInputAudioTranscriptionDelta)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when input audio transcription is configured, and a transcription \nrequest for a user message failed. These events are separate from other \n`error` events so that the client can identify the related Item.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
    #[doc = "The index of the content part containing the audio."]
    pub content_index: i64,
    pub error: RealtimeServerEventConversationItemInputAudioTranscriptionFailedError,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the user message item."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be\n`conversation.item.input_audio_transcription.failed`.\n"]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionFailedType,
}
#[doc = "Details of the transcription error."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailedError {
    #[doc = "Error code, if any."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    #[doc = "A human-readable error message."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[doc = "Parameter related to the error, if any."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub param: ::std::option::Option<::std::string::String>,
    #[doc = "The type of error."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
#[doc = "The event type, must be\n`conversation.item.input_audio_transcription.failed`.\n"]
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
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionFailedType {
    #[serde(rename = "conversation.item.input_audio_transcription.failed")]
    ConversationItemInputAudioTranscriptionFailed,
}
impl ::std::fmt::Display for RealtimeServerEventConversationItemInputAudioTranscriptionFailedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemInputAudioTranscriptionFailed => {
                f.write_str("conversation.item.input_audio_transcription.failed")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationItemInputAudioTranscriptionFailedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.input_audio_transcription.failed" => {
                Ok(Self::ConversationItemInputAudioTranscriptionFailed)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for RealtimeServerEventConversationItemInputAudioTranscriptionFailedType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemInputAudioTranscriptionFailedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when an input audio transcription segment is identified for an item."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionSegment {
    #[doc = "The index of the input audio content part within the item."]
    pub content_index: i64,
    #[doc = "End time of the segment in seconds."]
    pub end: f64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The segment identifier."]
    pub id: ::std::string::String,
    #[doc = "The ID of the item containing the input audio content."]
    pub item_id: ::std::string::String,
    #[doc = "The detected speaker label for this segment."]
    pub speaker: ::std::string::String,
    #[doc = "Start time of the segment in seconds."]
    pub start: f64,
    #[doc = "The text for this segment."]
    pub text: ::std::string::String,
    #[doc = "The event type, must be `conversation.item.input_audio_transcription.segment`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionSegmentType,
}
#[doc = "The event type, must be `conversation.item.input_audio_transcription.segment`."]
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
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionSegmentType {
    #[serde(rename = "conversation.item.input_audio_transcription.segment")]
    ConversationItemInputAudioTranscriptionSegment,
}
impl ::std::fmt::Display for RealtimeServerEventConversationItemInputAudioTranscriptionSegmentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemInputAudioTranscriptionSegment => {
                f.write_str("conversation.item.input_audio_transcription.segment")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationItemInputAudioTranscriptionSegmentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.input_audio_transcription.segment" => {
                Ok(Self::ConversationItemInputAudioTranscriptionSegment)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for RealtimeServerEventConversationItemInputAudioTranscriptionSegmentType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemInputAudioTranscriptionSegmentType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a conversation item is retrieved with `conversation.item.retrieve`. This is provided as a way to fetch the server's representation of an item, for example to get access to the post-processed audio data after noise cancellation and VAD. It includes the full content of the Item, including audio data.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemRetrieved {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    pub item: RealtimeConversationItem,
    #[doc = "The event type, must be `conversation.item.retrieved`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemRetrievedType,
}
#[doc = "The event type, must be `conversation.item.retrieved`."]
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
pub enum RealtimeServerEventConversationItemRetrievedType {
    #[serde(rename = "conversation.item.retrieved")]
    ConversationItemRetrieved,
}
impl ::std::fmt::Display for RealtimeServerEventConversationItemRetrievedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemRetrieved => f.write_str("conversation.item.retrieved"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationItemRetrievedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.retrieved" => Ok(Self::ConversationItemRetrieved),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventConversationItemRetrievedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemRetrievedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when an earlier assistant audio message item is truncated by the \nclient with a `conversation.item.truncate` event. This event is used to \nsynchronize the server's understanding of the audio with the client's playback.\n\nThis action will truncate the audio and remove the server-side text transcript \nto ensure there is no text in the context that hasn't been heard by the user.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventConversationItemTruncated {
    #[doc = "The duration up to which the audio was truncated, in milliseconds.\n"]
    pub audio_end_ms: i64,
    #[doc = "The index of the content part that was truncated."]
    pub content_index: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the assistant message item that was truncated."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `conversation.item.truncated`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemTruncatedType,
}
#[doc = "The event type, must be `conversation.item.truncated`."]
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
pub enum RealtimeServerEventConversationItemTruncatedType {
    #[serde(rename = "conversation.item.truncated")]
    ConversationItemTruncated,
}
impl ::std::fmt::Display for RealtimeServerEventConversationItemTruncatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationItemTruncated => f.write_str("conversation.item.truncated"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventConversationItemTruncatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.item.truncated" => Ok(Self::ConversationItemTruncated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventConversationItemTruncatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventConversationItemTruncatedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when an error occurs, which could be a client problem or a server\nproblem. Most errors are recoverable and the session will stay open, we\nrecommend to implementors to monitor and log error messages by default.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventError {
    pub error: RealtimeServerEventErrorError,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, must be `error`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventErrorType,
}
#[doc = "Details of the error."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventErrorError {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<::std::string::String>,
    #[doc = "A human-readable error message."]
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub param: ::std::option::Option<::std::string::String>,
    #[doc = "The type of error (e.g., \"invalid_request_error\", \"server_error\").\n"]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The event type, must be `error`."]
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
pub enum RealtimeServerEventErrorType {
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for RealtimeServerEventErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the input audio buffer is cleared by the client with a \n`input_audio_buffer.clear` event.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventInputAudioBufferCleared {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, must be `input_audio_buffer.cleared`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferClearedType,
}
#[doc = "The event type, must be `input_audio_buffer.cleared`."]
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
pub enum RealtimeServerEventInputAudioBufferClearedType {
    #[serde(rename = "input_audio_buffer.cleared")]
    InputAudioBufferCleared,
}
impl ::std::fmt::Display for RealtimeServerEventInputAudioBufferClearedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudioBufferCleared => f.write_str("input_audio_buffer.cleared"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventInputAudioBufferClearedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio_buffer.cleared" => Ok(Self::InputAudioBufferCleared),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventInputAudioBufferClearedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventInputAudioBufferClearedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when an input audio buffer is committed, either by the client or\nautomatically in server VAD mode. The `item_id` property is the ID of the user\nmessage item that will be created, thus a `conversation.item.created` event\nwill also be sent to the client.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventInputAudioBufferCommitted {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the user message item that will be created."]
    pub item_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_item_id: ::std::option::Option<::std::string::String>,
    #[doc = "The event type, must be `input_audio_buffer.committed`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferCommittedType,
}
#[doc = "The event type, must be `input_audio_buffer.committed`."]
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
pub enum RealtimeServerEventInputAudioBufferCommittedType {
    #[serde(rename = "input_audio_buffer.committed")]
    InputAudioBufferCommitted,
}
impl ::std::fmt::Display for RealtimeServerEventInputAudioBufferCommittedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudioBufferCommitted => f.write_str("input_audio_buffer.committed"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventInputAudioBufferCommittedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio_buffer.committed" => Ok(Self::InputAudioBufferCommitted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventInputAudioBufferCommittedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventInputAudioBufferCommittedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "**SIP Only:** Returned when an DTMF event is received. A DTMF event is a message that\nrepresents a telephone keypad press (0–9, *, #, A–D). The `event` property\nis the keypad that the user press. The `received_at` is the UTC Unix Timestamp\nthat the server received the event.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventInputAudioBufferDtmfEventReceived {
    #[doc = "The telephone keypad that was pressed by the user."]
    pub event: ::std::string::String,
    #[doc = "UTC Unix Timestamp when DTMF Event was received by server.\n"]
    pub received_at: i64,
    #[doc = "The event type, must be `input_audio_buffer.dtmf_event_received`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferDtmfEventReceivedType,
}
#[doc = "The event type, must be `input_audio_buffer.dtmf_event_received`."]
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
pub enum RealtimeServerEventInputAudioBufferDtmfEventReceivedType {
    #[serde(rename = "input_audio_buffer.dtmf_event_received")]
    InputAudioBufferDtmfEventReceived,
}
impl ::std::fmt::Display for RealtimeServerEventInputAudioBufferDtmfEventReceivedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudioBufferDtmfEventReceived => {
                f.write_str("input_audio_buffer.dtmf_event_received")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventInputAudioBufferDtmfEventReceivedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio_buffer.dtmf_event_received" => Ok(Self::InputAudioBufferDtmfEventReceived),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventInputAudioBufferDtmfEventReceivedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventInputAudioBufferDtmfEventReceivedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Sent by the server when in `server_vad` mode to indicate that speech has been \ndetected in the audio buffer. This can happen any time audio is added to the \nbuffer (unless speech is already detected). The client may want to use this \nevent to interrupt audio playback or provide visual feedback to the user. \n\nThe client should expect to receive a `input_audio_buffer.speech_stopped` event \nwhen speech stops. The `item_id` property is the ID of the user message item \nthat will be created when speech stops and will also be included in the \n`input_audio_buffer.speech_stopped` event (unless the client manually commits \nthe audio buffer during VAD activation).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
    #[doc = "Milliseconds from the start of all audio written to the buffer during the \nsession when speech was first detected. This will correspond to the \nbeginning of audio sent to the model, and thus includes the \n`prefix_padding_ms` configured in the Session.\n"]
    pub audio_start_ms: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the user message item that will be created when speech stops.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `input_audio_buffer.speech_started`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferSpeechStartedType,
}
#[doc = "The event type, must be `input_audio_buffer.speech_started`."]
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
pub enum RealtimeServerEventInputAudioBufferSpeechStartedType {
    #[serde(rename = "input_audio_buffer.speech_started")]
    InputAudioBufferSpeechStarted,
}
impl ::std::fmt::Display for RealtimeServerEventInputAudioBufferSpeechStartedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudioBufferSpeechStarted => f.write_str("input_audio_buffer.speech_started"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventInputAudioBufferSpeechStartedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio_buffer.speech_started" => Ok(Self::InputAudioBufferSpeechStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventInputAudioBufferSpeechStartedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventInputAudioBufferSpeechStartedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned in `server_vad` mode when the server detects the end of speech in \nthe audio buffer. The server will also send an `conversation.item.created` \nevent with the user message item that is created from the audio buffer.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
    #[doc = "Milliseconds since the session started when speech stopped. This will \ncorrespond to the end of audio sent to the model, and thus includes the \n`min_silence_duration_ms` configured in the Session.\n"]
    pub audio_end_ms: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the user message item that will be created."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `input_audio_buffer.speech_stopped`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferSpeechStoppedType,
}
#[doc = "The event type, must be `input_audio_buffer.speech_stopped`."]
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
pub enum RealtimeServerEventInputAudioBufferSpeechStoppedType {
    #[serde(rename = "input_audio_buffer.speech_stopped")]
    InputAudioBufferSpeechStopped,
}
impl ::std::fmt::Display for RealtimeServerEventInputAudioBufferSpeechStoppedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudioBufferSpeechStopped => f.write_str("input_audio_buffer.speech_stopped"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventInputAudioBufferSpeechStoppedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio_buffer.speech_stopped" => Ok(Self::InputAudioBufferSpeechStopped),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventInputAudioBufferSpeechStoppedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventInputAudioBufferSpeechStoppedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the Server VAD timeout is triggered for the input audio buffer. This is configured\nwith `idle_timeout_ms` in the `turn_detection` settings of the session, and it indicates that\nthere hasn't been any speech detected for the configured duration.\n\nThe `audio_start_ms` and `audio_end_ms` fields indicate the segment of audio after the last\nmodel response up to the triggering time, as an offset from the beginning of audio written\nto the input audio buffer. This means it demarcates the segment of audio that was silent and\nthe difference between the start and end values will roughly match the configured timeout.\n\nThe empty audio will be committed to the conversation as an `input_audio` item (there will be a\n`input_audio_buffer.committed` event) and a model response will be generated. There may be speech\nthat didn't trigger VAD but is still detected by the model, so the model may respond with\nsomething relevant to the conversation or a prompt to continue speaking.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventInputAudioBufferTimeoutTriggered {
    #[doc = "Millisecond offset of audio written to the input audio buffer at the time the timeout was triggered."]
    pub audio_end_ms: i64,
    #[doc = "Millisecond offset of audio written to the input audio buffer that was after the playback time of the last model response."]
    pub audio_start_ms: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item associated with this segment."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `input_audio_buffer.timeout_triggered`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferTimeoutTriggeredType,
}
#[doc = "The event type, must be `input_audio_buffer.timeout_triggered`."]
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
pub enum RealtimeServerEventInputAudioBufferTimeoutTriggeredType {
    #[serde(rename = "input_audio_buffer.timeout_triggered")]
    InputAudioBufferTimeoutTriggered,
}
impl ::std::fmt::Display for RealtimeServerEventInputAudioBufferTimeoutTriggeredType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudioBufferTimeoutTriggered => {
                f.write_str("input_audio_buffer.timeout_triggered")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventInputAudioBufferTimeoutTriggeredType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio_buffer.timeout_triggered" => Ok(Self::InputAudioBufferTimeoutTriggered),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventInputAudioBufferTimeoutTriggeredType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventInputAudioBufferTimeoutTriggeredType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when listing MCP tools has completed for an item."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventMcpListToolsCompleted {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the MCP list tools item."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `mcp_list_tools.completed`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventMcpListToolsCompletedType,
}
#[doc = "The event type, must be `mcp_list_tools.completed`."]
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
pub enum RealtimeServerEventMcpListToolsCompletedType {
    #[serde(rename = "mcp_list_tools.completed")]
    McpListToolsCompleted,
}
impl ::std::fmt::Display for RealtimeServerEventMcpListToolsCompletedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpListToolsCompleted => f.write_str("mcp_list_tools.completed"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventMcpListToolsCompletedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_list_tools.completed" => Ok(Self::McpListToolsCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventMcpListToolsCompletedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventMcpListToolsCompletedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when listing MCP tools has failed for an item."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventMcpListToolsFailed {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the MCP list tools item."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `mcp_list_tools.failed`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventMcpListToolsFailedType,
}
#[doc = "The event type, must be `mcp_list_tools.failed`."]
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
pub enum RealtimeServerEventMcpListToolsFailedType {
    #[serde(rename = "mcp_list_tools.failed")]
    McpListToolsFailed,
}
impl ::std::fmt::Display for RealtimeServerEventMcpListToolsFailedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpListToolsFailed => f.write_str("mcp_list_tools.failed"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventMcpListToolsFailedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_list_tools.failed" => Ok(Self::McpListToolsFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventMcpListToolsFailedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventMcpListToolsFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when listing MCP tools is in progress for an item."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventMcpListToolsInProgress {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the MCP list tools item."]
    pub item_id: ::std::string::String,
    #[doc = "The event type, must be `mcp_list_tools.in_progress`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventMcpListToolsInProgressType,
}
#[doc = "The event type, must be `mcp_list_tools.in_progress`."]
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
pub enum RealtimeServerEventMcpListToolsInProgressType {
    #[serde(rename = "mcp_list_tools.in_progress")]
    McpListToolsInProgress,
}
impl ::std::fmt::Display for RealtimeServerEventMcpListToolsInProgressType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpListToolsInProgress => f.write_str("mcp_list_tools.in_progress"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventMcpListToolsInProgressType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_list_tools.in_progress" => Ok(Self::McpListToolsInProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventMcpListToolsInProgressType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventMcpListToolsInProgressType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "**WebRTC/SIP Only:** Emitted when the output audio buffer is cleared. This happens either in VAD\nmode when the user has interrupted (`input_audio_buffer.speech_started`),\nor when the client has emitted the `output_audio_buffer.clear` event to manually\ncut off the current audio response.\n[Learn more](https://developers.openai.com/api/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventOutputAudioBufferCleared {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The unique ID of the response that produced the audio."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `output_audio_buffer.cleared`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventOutputAudioBufferClearedType,
}
#[doc = "The event type, must be `output_audio_buffer.cleared`."]
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
pub enum RealtimeServerEventOutputAudioBufferClearedType {
    #[serde(rename = "output_audio_buffer.cleared")]
    OutputAudioBufferCleared,
}
impl ::std::fmt::Display for RealtimeServerEventOutputAudioBufferClearedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutputAudioBufferCleared => f.write_str("output_audio_buffer.cleared"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventOutputAudioBufferClearedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "output_audio_buffer.cleared" => Ok(Self::OutputAudioBufferCleared),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventOutputAudioBufferClearedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventOutputAudioBufferClearedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "**WebRTC/SIP Only:** Emitted when the server begins streaming audio to the client. This event is\nemitted after an audio content part has been added (`response.content_part.added`)\nto the response.\n[Learn more](https://developers.openai.com/api/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventOutputAudioBufferStarted {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The unique ID of the response that produced the audio."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `output_audio_buffer.started`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventOutputAudioBufferStartedType,
}
#[doc = "The event type, must be `output_audio_buffer.started`."]
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
pub enum RealtimeServerEventOutputAudioBufferStartedType {
    #[serde(rename = "output_audio_buffer.started")]
    OutputAudioBufferStarted,
}
impl ::std::fmt::Display for RealtimeServerEventOutputAudioBufferStartedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutputAudioBufferStarted => f.write_str("output_audio_buffer.started"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventOutputAudioBufferStartedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "output_audio_buffer.started" => Ok(Self::OutputAudioBufferStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventOutputAudioBufferStartedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventOutputAudioBufferStartedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "**WebRTC/SIP Only:** Emitted when the output audio buffer has been completely drained on the server,\nand no more audio is forthcoming. This event is emitted after the full response\ndata has been sent to the client (`response.done`).\n[Learn more](https://developers.openai.com/api/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventOutputAudioBufferStopped {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The unique ID of the response that produced the audio."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `output_audio_buffer.stopped`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventOutputAudioBufferStoppedType,
}
#[doc = "The event type, must be `output_audio_buffer.stopped`."]
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
pub enum RealtimeServerEventOutputAudioBufferStoppedType {
    #[serde(rename = "output_audio_buffer.stopped")]
    OutputAudioBufferStopped,
}
impl ::std::fmt::Display for RealtimeServerEventOutputAudioBufferStoppedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutputAudioBufferStopped => f.write_str("output_audio_buffer.stopped"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventOutputAudioBufferStoppedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "output_audio_buffer.stopped" => Ok(Self::OutputAudioBufferStopped),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventOutputAudioBufferStoppedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventOutputAudioBufferStoppedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted at the beginning of a Response to indicate the updated rate limits. \nWhen a Response is created some tokens will be \"reserved\" for the output \ntokens, the rate limits shown here reflect that reservation, which is then \nadjusted accordingly once the Response is completed.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventRateLimitsUpdated {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "List of rate limit information."]
    pub rate_limits: ::std::vec::Vec<RealtimeServerEventRateLimitsUpdatedRateLimitsItem>,
    #[doc = "The event type, must be `rate_limits.updated`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventRateLimitsUpdatedType,
}
#[doc = "`RealtimeServerEventRateLimitsUpdatedRateLimitsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeServerEventRateLimitsUpdatedRateLimitsItem {
    #[doc = "The maximum allowed value for the rate limit."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[doc = "The name of the rate limit (`requests`, `tokens`).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<RealtimeServerEventRateLimitsUpdatedRateLimitsItemName>,
    #[doc = "The remaining value before the limit is reached."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub remaining: ::std::option::Option<i64>,
    #[doc = "Seconds until the rate limit resets."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reset_seconds: ::std::option::Option<f64>,
}
#[doc = "The name of the rate limit (`requests`, `tokens`).\n"]
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
pub enum RealtimeServerEventRateLimitsUpdatedRateLimitsItemName {
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "tokens")]
    Tokens,
}
impl ::std::fmt::Display for RealtimeServerEventRateLimitsUpdatedRateLimitsItemName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Requests => f.write_str("requests"),
            Self::Tokens => f.write_str("tokens"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventRateLimitsUpdatedRateLimitsItemName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "requests" => Ok(Self::Requests),
            "tokens" => Ok(Self::Tokens),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventRateLimitsUpdatedRateLimitsItemName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventRateLimitsUpdatedRateLimitsItemName
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The event type, must be `rate_limits.updated`."]
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
pub enum RealtimeServerEventRateLimitsUpdatedType {
    #[serde(rename = "rate_limits.updated")]
    RateLimitsUpdated,
}
impl ::std::fmt::Display for RealtimeServerEventRateLimitsUpdatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RateLimitsUpdated => f.write_str("rate_limits.updated"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventRateLimitsUpdatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "rate_limits.updated" => Ok(Self::RateLimitsUpdated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventRateLimitsUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventRateLimitsUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the model-generated audio is updated."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseAudioDelta {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: i64,
    #[doc = "Base64-encoded audio data delta."]
    pub delta: ::std::string::String,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.output_audio.delta`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseAudioDeltaType,
}
#[doc = "The event type, must be `response.output_audio.delta`."]
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
pub enum RealtimeServerEventResponseAudioDeltaType {
    #[serde(rename = "response.output_audio.delta")]
    ResponseOutputAudioDelta,
}
impl ::std::fmt::Display for RealtimeServerEventResponseAudioDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputAudioDelta => f.write_str("response.output_audio.delta"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseAudioDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_audio.delta" => Ok(Self::ResponseOutputAudioDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseAudioDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventResponseAudioDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the model-generated audio is done. Also emitted when a Response\nis interrupted, incomplete, or cancelled.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseAudioDone {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.output_audio.done`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseAudioDoneType,
}
#[doc = "The event type, must be `response.output_audio.done`."]
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
pub enum RealtimeServerEventResponseAudioDoneType {
    #[serde(rename = "response.output_audio.done")]
    ResponseOutputAudioDone,
}
impl ::std::fmt::Display for RealtimeServerEventResponseAudioDoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputAudioDone => f.write_str("response.output_audio.done"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseAudioDoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_audio.done" => Ok(Self::ResponseOutputAudioDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseAudioDoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventResponseAudioDoneType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the model-generated transcription of audio output is updated.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseAudioTranscriptDelta {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: i64,
    #[doc = "The transcript delta."]
    pub delta: ::std::string::String,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.output_audio_transcript.delta`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseAudioTranscriptDeltaType,
}
#[doc = "The event type, must be `response.output_audio_transcript.delta`."]
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
pub enum RealtimeServerEventResponseAudioTranscriptDeltaType {
    #[serde(rename = "response.output_audio_transcript.delta")]
    ResponseOutputAudioTranscriptDelta,
}
impl ::std::fmt::Display for RealtimeServerEventResponseAudioTranscriptDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputAudioTranscriptDelta => {
                f.write_str("response.output_audio_transcript.delta")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseAudioTranscriptDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_audio_transcript.delta" => {
                Ok(Self::ResponseOutputAudioTranscriptDelta)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseAudioTranscriptDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseAudioTranscriptDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the model-generated transcription of audio output is done\nstreaming. Also emitted when a Response is interrupted, incomplete, or\ncancelled.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseAudioTranscriptDone {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The final transcript of the audio."]
    pub transcript: ::std::string::String,
    #[doc = "The event type, must be `response.output_audio_transcript.done`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseAudioTranscriptDoneType,
}
#[doc = "The event type, must be `response.output_audio_transcript.done`."]
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
pub enum RealtimeServerEventResponseAudioTranscriptDoneType {
    #[serde(rename = "response.output_audio_transcript.done")]
    ResponseOutputAudioTranscriptDone,
}
impl ::std::fmt::Display for RealtimeServerEventResponseAudioTranscriptDoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputAudioTranscriptDone => {
                f.write_str("response.output_audio_transcript.done")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseAudioTranscriptDoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_audio_transcript.done" => Ok(Self::ResponseOutputAudioTranscriptDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseAudioTranscriptDoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseAudioTranscriptDoneType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a new content part is added to an assistant message item during\nresponse generation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseContentPartAdded {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item to which the content part was added."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    pub part: RealtimeServerEventResponseContentPartAddedPart,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.content_part.added`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseContentPartAddedType,
}
#[doc = "The content part that was added."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeServerEventResponseContentPartAddedPart {
    #[doc = "Base64-encoded audio data (if type is \"audio\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<::std::string::String>,
    #[doc = "The text content (if type is \"text\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "The transcript of the audio (if type is \"audio\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcript: ::std::option::Option<::std::string::String>,
    #[doc = "The content type (\"text\", \"audio\")."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RealtimeServerEventResponseContentPartAddedPartType>,
}
#[doc = "The content type (\"text\", \"audio\")."]
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
pub enum RealtimeServerEventResponseContentPartAddedPartType {
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for RealtimeServerEventResponseContentPartAddedPartType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Audio => f.write_str("audio"),
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseContentPartAddedPartType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio" => Ok(Self::Audio),
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseContentPartAddedPartType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseContentPartAddedPartType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The event type, must be `response.content_part.added`."]
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
pub enum RealtimeServerEventResponseContentPartAddedType {
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded,
}
impl ::std::fmt::Display for RealtimeServerEventResponseContentPartAddedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseContentPartAdded => f.write_str("response.content_part.added"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseContentPartAddedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.content_part.added" => Ok(Self::ResponseContentPartAdded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseContentPartAddedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseContentPartAddedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a content part is done streaming in an assistant message item.\nAlso emitted when a Response is interrupted, incomplete, or cancelled.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseContentPartDone {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    pub part: RealtimeServerEventResponseContentPartDonePart,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.content_part.done`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseContentPartDoneType,
}
#[doc = "The content part that is done."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeServerEventResponseContentPartDonePart {
    #[doc = "Base64-encoded audio data (if type is \"audio\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<::std::string::String>,
    #[doc = "The text content (if type is \"text\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "The transcript of the audio (if type is \"audio\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcript: ::std::option::Option<::std::string::String>,
    #[doc = "The content type (\"text\", \"audio\")."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RealtimeServerEventResponseContentPartDonePartType>,
}
#[doc = "The content type (\"text\", \"audio\")."]
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
pub enum RealtimeServerEventResponseContentPartDonePartType {
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for RealtimeServerEventResponseContentPartDonePartType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Audio => f.write_str("audio"),
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseContentPartDonePartType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio" => Ok(Self::Audio),
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseContentPartDonePartType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseContentPartDonePartType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The event type, must be `response.content_part.done`."]
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
pub enum RealtimeServerEventResponseContentPartDoneType {
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone,
}
impl ::std::fmt::Display for RealtimeServerEventResponseContentPartDoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseContentPartDone => f.write_str("response.content_part.done"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseContentPartDoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.content_part.done" => Ok(Self::ResponseContentPartDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseContentPartDoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseContentPartDoneType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a new Response is created. The first event of response creation,\nwhere the response is in an initial state of `in_progress`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseCreated {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    pub response: RealtimeResponse,
    #[doc = "The event type, must be `response.created`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseCreatedType,
}
#[doc = "The event type, must be `response.created`."]
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
pub enum RealtimeServerEventResponseCreatedType {
    #[serde(rename = "response.created")]
    ResponseCreated,
}
impl ::std::fmt::Display for RealtimeServerEventResponseCreatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCreated => f.write_str("response.created"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseCreatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.created" => Ok(Self::ResponseCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventResponseCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a Response is done streaming. Always emitted, no matter the \nfinal state. The Response object included in the `response.done` event will \ninclude all output Items in the Response but will omit the raw audio data.\n\nClients should check the `status` field of the Response to determine if it was successful\n(`completed`) or if there was another outcome: `cancelled`, `failed`, or `incomplete`.\n\nA response will contain all output items that were generated during the response, excluding\nany audio content.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseDone {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    pub response: RealtimeResponse,
    #[doc = "The event type, must be `response.done`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseDoneType,
}
#[doc = "The event type, must be `response.done`."]
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
pub enum RealtimeServerEventResponseDoneType {
    #[serde(rename = "response.done")]
    ResponseDone,
}
impl ::std::fmt::Display for RealtimeServerEventResponseDoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseDone => f.write_str("response.done"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseDoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.done" => Ok(Self::ResponseDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseDoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventResponseDoneType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the model-generated function call arguments are updated.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
    #[doc = "The ID of the function call."]
    pub call_id: ::std::string::String,
    #[doc = "The arguments delta as a JSON string."]
    pub delta: ::std::string::String,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the function call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.function_call_arguments.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseFunctionCallArgumentsDeltaType,
}
#[doc = "The event type, must be `response.function_call_arguments.delta`.\n"]
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
pub enum RealtimeServerEventResponseFunctionCallArgumentsDeltaType {
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta,
}
impl ::std::fmt::Display for RealtimeServerEventResponseFunctionCallArgumentsDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseFunctionCallArgumentsDelta => {
                f.write_str("response.function_call_arguments.delta")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseFunctionCallArgumentsDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.function_call_arguments.delta" => {
                Ok(Self::ResponseFunctionCallArgumentsDelta)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseFunctionCallArgumentsDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseFunctionCallArgumentsDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the model-generated function call arguments are done streaming.\nAlso emitted when a Response is interrupted, incomplete, or cancelled.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDone {
    #[doc = "The final arguments as a JSON string."]
    pub arguments: ::std::string::String,
    #[doc = "The ID of the function call."]
    pub call_id: ::std::string::String,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the function call item."]
    pub item_id: ::std::string::String,
    #[doc = "The name of the function that was called."]
    pub name: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.function_call_arguments.done`.\n"]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseFunctionCallArgumentsDoneType,
}
#[doc = "The event type, must be `response.function_call_arguments.done`.\n"]
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
pub enum RealtimeServerEventResponseFunctionCallArgumentsDoneType {
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone,
}
impl ::std::fmt::Display for RealtimeServerEventResponseFunctionCallArgumentsDoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseFunctionCallArgumentsDone => {
                f.write_str("response.function_call_arguments.done")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseFunctionCallArgumentsDoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.function_call_arguments.done" => Ok(Self::ResponseFunctionCallArgumentsDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseFunctionCallArgumentsDoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseFunctionCallArgumentsDoneType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when MCP tool call arguments are updated during response generation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseMcpCallArgumentsDelta {
    #[doc = "The JSON-encoded arguments delta."]
    pub delta: ::std::string::String,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the MCP tool call item."]
    pub item_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub obfuscation: ::std::option::Option<::std::string::String>,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.mcp_call_arguments.delta`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseMcpCallArgumentsDeltaType,
}
#[doc = "The event type, must be `response.mcp_call_arguments.delta`."]
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
pub enum RealtimeServerEventResponseMcpCallArgumentsDeltaType {
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMcpCallArgumentsDelta,
}
impl ::std::fmt::Display for RealtimeServerEventResponseMcpCallArgumentsDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallArgumentsDelta => f.write_str("response.mcp_call_arguments.delta"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseMcpCallArgumentsDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call_arguments.delta" => Ok(Self::ResponseMcpCallArgumentsDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseMcpCallArgumentsDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseMcpCallArgumentsDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when MCP tool call arguments are finalized during response generation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseMcpCallArgumentsDone {
    #[doc = "The final JSON-encoded arguments string."]
    pub arguments: ::std::string::String,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the MCP tool call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.mcp_call_arguments.done`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseMcpCallArgumentsDoneType,
}
#[doc = "The event type, must be `response.mcp_call_arguments.done`."]
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
pub enum RealtimeServerEventResponseMcpCallArgumentsDoneType {
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMcpCallArgumentsDone,
}
impl ::std::fmt::Display for RealtimeServerEventResponseMcpCallArgumentsDoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallArgumentsDone => f.write_str("response.mcp_call_arguments.done"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseMcpCallArgumentsDoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call_arguments.done" => Ok(Self::ResponseMcpCallArgumentsDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseMcpCallArgumentsDoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseMcpCallArgumentsDoneType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when an MCP tool call has completed successfully."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseMcpCallCompleted {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the MCP tool call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The event type, must be `response.mcp_call.completed`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseMcpCallCompletedType,
}
#[doc = "The event type, must be `response.mcp_call.completed`."]
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
pub enum RealtimeServerEventResponseMcpCallCompletedType {
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMcpCallCompleted,
}
impl ::std::fmt::Display for RealtimeServerEventResponseMcpCallCompletedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallCompleted => f.write_str("response.mcp_call.completed"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseMcpCallCompletedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call.completed" => Ok(Self::ResponseMcpCallCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseMcpCallCompletedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseMcpCallCompletedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when an MCP tool call has failed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseMcpCallFailed {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the MCP tool call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The event type, must be `response.mcp_call.failed`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseMcpCallFailedType,
}
#[doc = "The event type, must be `response.mcp_call.failed`."]
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
pub enum RealtimeServerEventResponseMcpCallFailedType {
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMcpCallFailed,
}
impl ::std::fmt::Display for RealtimeServerEventResponseMcpCallFailedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallFailed => f.write_str("response.mcp_call.failed"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseMcpCallFailedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call.failed" => Ok(Self::ResponseMcpCallFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseMcpCallFailedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseMcpCallFailedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when an MCP tool call has started and is in progress."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseMcpCallInProgress {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the MCP tool call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The event type, must be `response.mcp_call.in_progress`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseMcpCallInProgressType,
}
#[doc = "The event type, must be `response.mcp_call.in_progress`."]
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
pub enum RealtimeServerEventResponseMcpCallInProgressType {
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMcpCallInProgress,
}
impl ::std::fmt::Display for RealtimeServerEventResponseMcpCallInProgressType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallInProgress => f.write_str("response.mcp_call.in_progress"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseMcpCallInProgressType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call.in_progress" => Ok(Self::ResponseMcpCallInProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseMcpCallInProgressType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseMcpCallInProgressType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a new Item is created during Response generation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseOutputItemAdded {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    pub item: RealtimeConversationItem,
    #[doc = "The index of the output item in the Response."]
    pub output_index: i64,
    #[doc = "The ID of the Response to which the item belongs."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.output_item.added`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseOutputItemAddedType,
}
#[doc = "The event type, must be `response.output_item.added`."]
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
pub enum RealtimeServerEventResponseOutputItemAddedType {
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded,
}
impl ::std::fmt::Display for RealtimeServerEventResponseOutputItemAddedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputItemAdded => f.write_str("response.output_item.added"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseOutputItemAddedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_item.added" => Ok(Self::ResponseOutputItemAdded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseOutputItemAddedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseOutputItemAddedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when an Item is done streaming. Also emitted when a Response is \ninterrupted, incomplete, or cancelled.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseOutputItemDone {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    pub item: RealtimeConversationItem,
    #[doc = "The index of the output item in the Response."]
    pub output_index: i64,
    #[doc = "The ID of the Response to which the item belongs."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.output_item.done`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseOutputItemDoneType,
}
#[doc = "The event type, must be `response.output_item.done`."]
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
pub enum RealtimeServerEventResponseOutputItemDoneType {
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone,
}
impl ::std::fmt::Display for RealtimeServerEventResponseOutputItemDoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputItemDone => f.write_str("response.output_item.done"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseOutputItemDoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_item.done" => Ok(Self::ResponseOutputItemDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseOutputItemDoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeServerEventResponseOutputItemDoneType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the text value of an \"output_text\" content part is updated."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseTextDelta {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: i64,
    #[doc = "The text delta."]
    pub delta: ::std::string::String,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The event type, must be `response.output_text.delta`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseTextDeltaType,
}
#[doc = "The event type, must be `response.output_text.delta`."]
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
pub enum RealtimeServerEventResponseTextDeltaType {
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta,
}
impl ::std::fmt::Display for RealtimeServerEventResponseTextDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputTextDelta => f.write_str("response.output_text.delta"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseTextDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_text.delta" => Ok(Self::ResponseOutputTextDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseTextDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventResponseTextDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the text value of an \"output_text\" content part is done streaming. Also\nemitted when a Response is interrupted, incomplete, or cancelled.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventResponseTextDone {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: i64,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The ID of the item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response."]
    pub output_index: i64,
    #[doc = "The ID of the response."]
    pub response_id: ::std::string::String,
    #[doc = "The final text content."]
    pub text: ::std::string::String,
    #[doc = "The event type, must be `response.output_text.done`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseTextDoneType,
}
#[doc = "The event type, must be `response.output_text.done`."]
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
pub enum RealtimeServerEventResponseTextDoneType {
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone,
}
impl ::std::fmt::Display for RealtimeServerEventResponseTextDoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputTextDone => f.write_str("response.output_text.done"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventResponseTextDoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_text.done" => Ok(Self::ResponseOutputTextDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventResponseTextDoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventResponseTextDoneType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a Session is created. Emitted automatically when a new\nconnection is established as the first server event. This event will contain\nthe default Session configuration.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventSessionCreated {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The session configuration."]
    pub session: RealtimeServerEventSessionCreatedSession,
    #[doc = "The event type, must be `session.created`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventSessionCreatedType,
}
#[doc = "The session configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeServerEventSessionCreatedSession {
    SessionCreateResponseGa(RealtimeSessionCreateResponseGa),
    TranscriptionSessionCreateResponseGa(RealtimeTranscriptionSessionCreateResponseGa),
}
impl ::std::convert::From<RealtimeSessionCreateResponseGa>
    for RealtimeServerEventSessionCreatedSession
{
    fn from(value: RealtimeSessionCreateResponseGa) -> Self {
        Self::SessionCreateResponseGa(value)
    }
}
impl ::std::convert::From<RealtimeTranscriptionSessionCreateResponseGa>
    for RealtimeServerEventSessionCreatedSession
{
    fn from(value: RealtimeTranscriptionSessionCreateResponseGa) -> Self {
        Self::TranscriptionSessionCreateResponseGa(value)
    }
}
#[doc = "The event type, must be `session.created`."]
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
pub enum RealtimeServerEventSessionCreatedType {
    #[serde(rename = "session.created")]
    SessionCreated,
}
impl ::std::fmt::Display for RealtimeServerEventSessionCreatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCreated => f.write_str("session.created"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventSessionCreatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.created" => Ok(Self::SessionCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventSessionCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventSessionCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a session is updated with a `session.update` event, unless\nthere is an error.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeServerEventSessionUpdated {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The session configuration."]
    pub session: RealtimeServerEventSessionUpdatedSession,
    #[doc = "The event type, must be `session.updated`."]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventSessionUpdatedType,
}
#[doc = "The session configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeServerEventSessionUpdatedSession {
    SessionCreateResponseGa(RealtimeSessionCreateResponseGa),
    TranscriptionSessionCreateResponseGa(RealtimeTranscriptionSessionCreateResponseGa),
}
impl ::std::convert::From<RealtimeSessionCreateResponseGa>
    for RealtimeServerEventSessionUpdatedSession
{
    fn from(value: RealtimeSessionCreateResponseGa) -> Self {
        Self::SessionCreateResponseGa(value)
    }
}
impl ::std::convert::From<RealtimeTranscriptionSessionCreateResponseGa>
    for RealtimeServerEventSessionUpdatedSession
{
    fn from(value: RealtimeTranscriptionSessionCreateResponseGa) -> Self {
        Self::TranscriptionSessionCreateResponseGa(value)
    }
}
#[doc = "The event type, must be `session.updated`."]
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
pub enum RealtimeServerEventSessionUpdatedType {
    #[serde(rename = "session.updated")]
    SessionUpdated,
}
impl ::std::fmt::Display for RealtimeServerEventSessionUpdatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionUpdated => f.write_str("session.updated"),
        }
    }
}
impl ::std::str::FromStr for RealtimeServerEventSessionUpdatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.updated" => Ok(Self::SessionUpdated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeServerEventSessionUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeServerEventSessionUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A new Realtime session configuration, with an ephemeral key. Default TTL\nfor keys is one minute.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeSessionCreateRequest {
    pub client_secret: RealtimeSessionCreateRequestClientSecret,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio_format: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio_transcription:
        ::std::option::Option<RealtimeSessionCreateRequestInputAudioTranscription>,
    #[doc = "The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.\nNote that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_response_output_tokens:
        ::std::option::Option<RealtimeSessionCreateRequestMaxResponseOutputTokens>,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub modalities: ::std::vec::Vec<RealtimeSessionCreateRequestModalitiesItem>,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_audio_format: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<Prompt>,
    #[doc = "The speed of the model's spoken response. 1.0 is the default speed. 0.25 is\nthe minimum speed. 1.5 is the maximum speed. This value can only be changed\nin between model turns, not while a response is in progress.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub speed: ::std::option::Option<f64>,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or\nspecify a function.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<::std::string::String>,
    #[doc = "Tools (functions) available to the model."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<RealtimeSessionCreateRequestToolsItem>,
    #[doc = "Configuration options for tracing. Set to null to disable tracing. Once\ntracing is enabled for a session, the configuration cannot be modified.\n\n`auto` will create a trace for the session with default values for the\nworkflow name, group id, and metadata.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tracing: ::std::option::Option<TracingConfiguration>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub truncation: ::std::option::Option<RealtimeTruncation>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub turn_detection: ::std::option::Option<RealtimeSessionCreateRequestTurnDetection>,
    #[doc = "The voice the model uses to respond. Supported built-in voices are\n`alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`, `shimmer`, `verse`,\n`marin`, and `cedar`. You may also provide a custom voice object with an\n`id`, for example `{ \"id\": \"voice_1234\" }`. Voice cannot be changed during\nthe session once the model has responded with audio at least once.\nCustom voices must be created from audio samples.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceIdsOrCustomVoice>,
}
#[doc = "Ephemeral key returned by the API."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeSessionCreateRequestClientSecret {
    #[doc = "Timestamp for when the token expires. Currently, all tokens expire\nafter one minute.\n"]
    pub expires_at: i64,
    #[doc = "Ephemeral key usable in client environments to authenticate connections\nto the Realtime API. Use this in client-side environments rather than\na standard API token, which should only be used server-side.\n"]
    pub value: ::std::string::String,
}
#[doc = "Realtime session object configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeSessionCreateRequestGa {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<RealtimeSessionCreateRequestGaAudio>,
    #[doc = "Additional fields to include in server outputs.\n\n`item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<RealtimeSessionCreateRequestGaIncludeItem>,
    #[doc = "The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.\n\nNote that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_tokens: ::std::option::Option<RealtimeSessionCreateRequestGaMaxOutputTokens>,
    #[doc = "The Realtime model used for this session.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The set of modalities the model can respond with. It defaults to `[\"audio\"]`, indicating\nthat the model will respond with audio plus a transcript. `[\"text\"]` can be used to make\nthe model respond with text only. It is not possible to request both `text` and `audio` at the same time.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub output_modalities: ::std::vec::Vec<RealtimeSessionCreateRequestGaOutputModalitiesItem>,
    #[doc = "Whether the model may call multiple tools in parallel. Only supported by\nreasoning Realtime models such as `gpt-realtime-2`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parallel_tool_calls: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<Prompt>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<RealtimeReasoning>,
    #[doc = "How the model chooses tools. Provide one of the string modes or force a specific\nfunction/MCP tool.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<RealtimeSessionCreateRequestGaToolChoice>,
    #[doc = "Tools available to the model."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<RealtimeSessionCreateRequestGaToolsItem>,
    #[doc = "Realtime API can write session traces to the [Traces Dashboard](https://platform.openai.com/logs?api=traces). Set to null to disable tracing. Once\ntracing is enabled for a session, the configuration cannot be modified.\n\n`auto` will create a trace for the session with default values for the\nworkflow name, group id, and metadata.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tracing: ::std::option::Option<TracingConfiguration>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub truncation: ::std::option::Option<RealtimeTruncation>,
    #[doc = "The type of session to create. Always `realtime` for the Realtime API.\n"]
    #[serde(rename = "type")]
    pub type_: RealtimeSessionCreateRequestGaType,
}
#[doc = "Configuration for input and output audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateRequestGaAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<RealtimeSessionCreateRequestGaAudioInput>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<RealtimeSessionCreateRequestGaAudioOutput>,
}
#[doc = "`RealtimeSessionCreateRequestGaAudioInput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateRequestGaAudioInput {
    #[doc = "The format of the input audio."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub noise_reduction:
        ::std::option::Option<RealtimeSessionCreateRequestGaAudioInputNoiseReduction>,
    #[doc = "Configuration for input audio transcription, defaults to off and can be set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs asynchronously through [the /audio/transcriptions endpoint](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription: ::std::option::Option<AudioTranscription>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub turn_detection: ::std::option::Option<RealtimeTurnDetection>,
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateRequestGaAudioInputNoiseReduction {
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<NoiseReductionType>,
}
#[doc = "`RealtimeSessionCreateRequestGaAudioOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateRequestGaAudioOutput {
    #[doc = "The format of the output audio."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[doc = "The speed of the model's spoken response as a multiple of the original speed.\n1.0 is the default speed. 0.25 is the minimum speed. 1.5 is the maximum speed. This value can only be changed in between model turns, not while a response is in progress.\n\nThis parameter is a post-processing adjustment to the audio after it is generated, it's\nalso possible to prompt the model to speak faster or slower.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub speed: ::std::option::Option<f64>,
    #[doc = "The voice the model uses to respond. Supported built-in voices are\n`alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`, `shimmer`, `verse`,\n`marin`, and `cedar`. You may also provide a custom voice object with\nan `id`, for example `{ \"id\": \"voice_1234\" }`. Voice cannot be changed\nduring the session once the model has responded with audio at least once.\nCustom voices must be created from audio samples.\nWe recommend `marin` and `cedar` for best quality.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceIdsOrCustomVoice>,
}
#[doc = "`RealtimeSessionCreateRequestGaIncludeItem`"]
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
pub enum RealtimeSessionCreateRequestGaIncludeItem {
    #[serde(rename = "item.input_audio_transcription.logprobs")]
    ItemInputAudioTranscriptionLogprobs,
}
impl ::std::fmt::Display for RealtimeSessionCreateRequestGaIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ItemInputAudioTranscriptionLogprobs => {
                f.write_str("item.input_audio_transcription.logprobs")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateRequestGaIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "item.input_audio_transcription.logprobs" => {
                Ok(Self::ItemInputAudioTranscriptionLogprobs)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestGaIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeSessionCreateRequestGaIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestGaMaxOutputTokens {
    Integer(i64),
    String(RealtimeSessionCreateRequestGaMaxOutputTokensString),
}
impl ::std::fmt::Display for RealtimeSessionCreateRequestGaMaxOutputTokens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateRequestGaMaxOutputTokens {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestGaMaxOutputTokens {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateRequestGaMaxOutputTokens
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<i64> for RealtimeSessionCreateRequestGaMaxOutputTokens {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<RealtimeSessionCreateRequestGaMaxOutputTokensString>
    for RealtimeSessionCreateRequestGaMaxOutputTokens
{
    fn from(value: RealtimeSessionCreateRequestGaMaxOutputTokensString) -> Self {
        Self::String(value)
    }
}
#[doc = "`RealtimeSessionCreateRequestGaMaxOutputTokensString`"]
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
pub enum RealtimeSessionCreateRequestGaMaxOutputTokensString {
    #[serde(rename = "inf")]
    Inf,
}
impl ::std::fmt::Display for RealtimeSessionCreateRequestGaMaxOutputTokensString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inf => f.write_str("inf"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateRequestGaMaxOutputTokensString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inf" => Ok(Self::Inf),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestGaMaxOutputTokensString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateRequestGaMaxOutputTokensString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeSessionCreateRequestGaOutputModalitiesItem`"]
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
pub enum RealtimeSessionCreateRequestGaOutputModalitiesItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
impl ::std::fmt::Display for RealtimeSessionCreateRequestGaOutputModalitiesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Audio => f.write_str("audio"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateRequestGaOutputModalitiesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestGaOutputModalitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateRequestGaOutputModalitiesItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "How the model chooses tools. Provide one of the string modes or force a specific\nfunction/MCP tool.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestGaToolChoice {
    Options(ToolChoiceOptions),
    Function(ToolChoiceFunction),
    Mcp(ToolChoiceMcp),
}
impl ::std::convert::From<ToolChoiceOptions> for RealtimeSessionCreateRequestGaToolChoice {
    fn from(value: ToolChoiceOptions) -> Self {
        Self::Options(value)
    }
}
impl ::std::convert::From<ToolChoiceFunction> for RealtimeSessionCreateRequestGaToolChoice {
    fn from(value: ToolChoiceFunction) -> Self {
        Self::Function(value)
    }
}
impl ::std::convert::From<ToolChoiceMcp> for RealtimeSessionCreateRequestGaToolChoice {
    fn from(value: ToolChoiceMcp) -> Self {
        Self::Mcp(value)
    }
}
#[doc = "`RealtimeSessionCreateRequestGaToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestGaToolsItem {
    RealtimeFunctionTool(RealtimeFunctionTool),
    McpTool(McpTool),
}
impl ::std::convert::From<RealtimeFunctionTool> for RealtimeSessionCreateRequestGaToolsItem {
    fn from(value: RealtimeFunctionTool) -> Self {
        Self::RealtimeFunctionTool(value)
    }
}
impl ::std::convert::From<McpTool> for RealtimeSessionCreateRequestGaToolsItem {
    fn from(value: McpTool) -> Self {
        Self::McpTool(value)
    }
}
#[doc = "The type of session to create. Always `realtime` for the Realtime API.\n"]
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
pub enum RealtimeSessionCreateRequestGaType {
    #[serde(rename = "realtime")]
    Realtime,
}
impl ::std::fmt::Display for RealtimeSessionCreateRequestGaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Realtime => f.write_str("realtime"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateRequestGaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "realtime" => Ok(Self::Realtime),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestGaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeSessionCreateRequestGaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for input audio transcription, defaults to off and can be\nset to `null` to turn off once on. Input audio transcription is not native\nto the model, since the model consumes audio directly. Transcription runs\nasynchronously and should be treated as rough guidance\nrather than the representation understood by the model.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateRequestInputAudioTranscription {
    #[doc = "The model to use for transcription.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokens {
    Integer(i64),
    String(RealtimeSessionCreateRequestMaxResponseOutputTokensString),
}
impl ::std::fmt::Display for RealtimeSessionCreateRequestMaxResponseOutputTokens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateRequestMaxResponseOutputTokens {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestMaxResponseOutputTokens {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateRequestMaxResponseOutputTokens
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<i64> for RealtimeSessionCreateRequestMaxResponseOutputTokens {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<RealtimeSessionCreateRequestMaxResponseOutputTokensString>
    for RealtimeSessionCreateRequestMaxResponseOutputTokens
{
    fn from(value: RealtimeSessionCreateRequestMaxResponseOutputTokensString) -> Self {
        Self::String(value)
    }
}
#[doc = "`RealtimeSessionCreateRequestMaxResponseOutputTokensString`"]
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
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokensString {
    #[serde(rename = "inf")]
    Inf,
}
impl ::std::fmt::Display for RealtimeSessionCreateRequestMaxResponseOutputTokensString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inf => f.write_str("inf"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateRequestMaxResponseOutputTokensString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inf" => Ok(Self::Inf),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestMaxResponseOutputTokensString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateRequestMaxResponseOutputTokensString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeSessionCreateRequestModalitiesItem`"]
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
pub enum RealtimeSessionCreateRequestModalitiesItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
impl ::std::fmt::Display for RealtimeSessionCreateRequestModalitiesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Audio => f.write_str("audio"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateRequestModalitiesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestModalitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeSessionCreateRequestModalitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeSessionCreateRequestToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateRequestToolsItem {
    #[doc = "The description of the function, including guidance on when and how\nto call it, and guidance about what to tell the user when calling\n(if anything).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "Parameters of the function in JSON Schema."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The type of the tool, i.e. `function`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RealtimeSessionCreateRequestToolsItemType>,
}
#[doc = "The type of the tool, i.e. `function`."]
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
pub enum RealtimeSessionCreateRequestToolsItemType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for RealtimeSessionCreateRequestToolsItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateRequestToolsItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestToolsItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeSessionCreateRequestToolsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server\nVAD means that the model will detect the start and end of speech based on\naudio volume and respond at the end of user speech.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateRequestTurnDetection {
    #[doc = "Amount of audio to include before the VAD detected speech (in\nmilliseconds). Defaults to 300ms.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prefix_padding_ms: ::std::option::Option<i64>,
    #[doc = "Duration of silence to detect speech stop (in milliseconds). Defaults\nto 500ms. With shorter values the model will respond more quickly,\nbut may jump in on short pauses from the user.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub silence_duration_ms: ::std::option::Option<i64>,
    #[doc = "Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A\nhigher threshold will require louder audio to activate the model, and\nthus might perform better in noisy environments.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub threshold: ::std::option::Option<f64>,
    #[doc = "Type of turn detection, only `server_vad` is currently supported.\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
#[doc = "A Realtime session configuration object.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponse {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<RealtimeSessionCreateResponseAudio>,
    #[doc = "Expiration timestamp for the session, in seconds since epoch."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "Unique identifier for the session that looks like `sess_1234567890abcdef`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "Additional fields to include in server outputs.\n- `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<RealtimeSessionCreateResponseIncludeItem>,
    #[doc = "The default system instructions (i.e. system message) prepended to model\ncalls. This field allows the client to guide the model on desired\nresponses. The model can be instructed on response content and format,\n(e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good\nresponses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion\ninto your voice\", \"laugh frequently\"). The instructions are not guaranteed\nto be followed by the model, but they provide guidance to the model on the\ndesired behavior.\n\nNote that the server sets default instructions which will be used if this\nfield is not set and are visible in the `session.created` event at the\nstart of the session.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_tokens: ::std::option::Option<RealtimeSessionCreateResponseMaxOutputTokens>,
    #[doc = "The Realtime model used for this session."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The object type. Always `realtime.session`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<::std::string::String>,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub output_modalities: ::std::vec::Vec<RealtimeSessionCreateResponseOutputModalitiesItem>,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or\nspecify a function.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<::std::string::String>,
    #[doc = "Tools (functions) available to the model."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<RealtimeFunctionTool>,
    #[doc = "Configuration options for tracing. Set to null to disable tracing. Once\ntracing is enabled for a session, the configuration cannot be modified.\n\n`auto` will create a trace for the session with default values for the\nworkflow name, group id, and metadata.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tracing: ::std::option::Option<TracingConfiguration>,
    #[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server\nVAD means that the model will detect the start and end of speech based on\naudio volume and respond at the end of user speech.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub turn_detection: ::std::option::Option<RealtimeSessionCreateResponseTurnDetection>,
}
#[doc = "Configuration for input and output audio for the session.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<RealtimeSessionCreateResponseAudioInput>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<RealtimeSessionCreateResponseAudioOutput>,
}
#[doc = "`RealtimeSessionCreateResponseAudioInput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseAudioInput {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[doc = "Configuration for input audio noise reduction.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub noise_reduction:
        ::std::option::Option<RealtimeSessionCreateResponseAudioInputNoiseReduction>,
    #[doc = "Configuration for input audio transcription.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription: ::std::option::Option<AudioTranscriptionResponse>,
    #[doc = "Configuration for turn detection.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub turn_detection: ::std::option::Option<RealtimeSessionCreateResponseAudioInputTurnDetection>,
}
#[doc = "Configuration for input audio noise reduction.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseAudioInputNoiseReduction {
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<NoiseReductionType>,
}
#[doc = "Configuration for turn detection.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseAudioInputTurnDetection {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prefix_padding_ms: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub silence_duration_ms: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub threshold: ::std::option::Option<f64>,
    #[doc = "Type of turn detection, only `server_vad` is currently supported.\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
#[doc = "`RealtimeSessionCreateResponseAudioOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseAudioOutput {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub speed: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceIdsShared>,
}
#[doc = "A Realtime session configuration object.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeSessionCreateResponseGa {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<RealtimeSessionCreateResponseGaAudio>,
    #[doc = "Expiration timestamp for the session, in seconds since epoch."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "Unique identifier for the session that looks like `sess_1234567890abcdef`.\n"]
    pub id: ::std::string::String,
    #[doc = "Additional fields to include in server outputs.\n\n`item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include: ::std::option::Option<::std::vec::Vec<RealtimeSessionCreateResponseGaIncludeItem>>,
    #[doc = "The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.\n\nNote that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_tokens: ::std::option::Option<RealtimeSessionCreateResponseGaMaxOutputTokens>,
    #[doc = "The Realtime model used for this session.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The object type. Always `realtime.session`."]
    pub object: RealtimeSessionCreateResponseGaObject,
    #[doc = "The set of modalities the model can respond with. It defaults to `[\"audio\"]`, indicating\nthat the model will respond with audio plus a transcript. `[\"text\"]` can be used to make\nthe model respond with text only. It is not possible to request both `text` and `audio` at the same time.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub output_modalities: ::std::vec::Vec<RealtimeSessionCreateResponseGaOutputModalitiesItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<Prompt>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<RealtimeReasoning>,
    #[doc = "How the model chooses tools. Provide one of the string modes or force a specific\nfunction/MCP tool.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<RealtimeSessionCreateResponseGaToolChoice>,
    #[doc = "Tools available to the model."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<RealtimeSessionCreateResponseGaToolsItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tracing: ::std::option::Option<TracingConfiguration>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub truncation: ::std::option::Option<RealtimeTruncation>,
    #[doc = "The type of session to create. Always `realtime` for the Realtime API.\n"]
    #[serde(rename = "type")]
    pub type_: RealtimeSessionCreateResponseGaType,
}
#[doc = "Configuration for input and output audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseGaAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<RealtimeSessionCreateResponseGaAudioInput>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<RealtimeSessionCreateResponseGaAudioOutput>,
}
#[doc = "`RealtimeSessionCreateResponseGaAudioInput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseGaAudioInput {
    #[doc = "The format of the input audio."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub noise_reduction:
        ::std::option::Option<RealtimeSessionCreateResponseGaAudioInputNoiseReduction>,
    #[doc = "Configuration for input audio transcription, defaults to off and can be set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs asynchronously through [the /audio/transcriptions endpoint](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription: ::std::option::Option<AudioTranscriptionResponse>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub turn_detection: ::std::option::Option<RealtimeTurnDetection>,
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseGaAudioInputNoiseReduction {
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<NoiseReductionType>,
}
#[doc = "`RealtimeSessionCreateResponseGaAudioOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseGaAudioOutput {
    #[doc = "The format of the output audio."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[doc = "The speed of the model's spoken response as a multiple of the original speed.\n1.0 is the default speed. 0.25 is the minimum speed. 1.5 is the maximum speed. This value can only be changed in between model turns, not while a response is in progress.\n\nThis parameter is a post-processing adjustment to the audio after it is generated, it's\nalso possible to prompt the model to speak faster or slower.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub speed: ::std::option::Option<f64>,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the\nsession once the model has responded with audio at least once. Current\nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`,\n`shimmer`, `verse`, `marin`, and `cedar`. We recommend `marin` and `cedar` for\nbest quality.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceIdsShared>,
}
#[doc = "`RealtimeSessionCreateResponseGaIncludeItem`"]
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
pub enum RealtimeSessionCreateResponseGaIncludeItem {
    #[serde(rename = "item.input_audio_transcription.logprobs")]
    ItemInputAudioTranscriptionLogprobs,
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseGaIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ItemInputAudioTranscriptionLogprobs => {
                f.write_str("item.input_audio_transcription.logprobs")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseGaIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "item.input_audio_transcription.logprobs" => {
                Ok(Self::ItemInputAudioTranscriptionLogprobs)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseGaIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeSessionCreateResponseGaIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeSessionCreateResponseGaMaxOutputTokens {
    Integer(i64),
    String(RealtimeSessionCreateResponseGaMaxOutputTokensString),
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseGaMaxOutputTokens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseGaMaxOutputTokens {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseGaMaxOutputTokens {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateResponseGaMaxOutputTokens
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<i64> for RealtimeSessionCreateResponseGaMaxOutputTokens {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<RealtimeSessionCreateResponseGaMaxOutputTokensString>
    for RealtimeSessionCreateResponseGaMaxOutputTokens
{
    fn from(value: RealtimeSessionCreateResponseGaMaxOutputTokensString) -> Self {
        Self::String(value)
    }
}
#[doc = "`RealtimeSessionCreateResponseGaMaxOutputTokensString`"]
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
pub enum RealtimeSessionCreateResponseGaMaxOutputTokensString {
    #[serde(rename = "inf")]
    Inf,
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseGaMaxOutputTokensString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inf => f.write_str("inf"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseGaMaxOutputTokensString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inf" => Ok(Self::Inf),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseGaMaxOutputTokensString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateResponseGaMaxOutputTokensString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type. Always `realtime.session`."]
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
pub enum RealtimeSessionCreateResponseGaObject {
    #[serde(rename = "realtime.session")]
    RealtimeSession,
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseGaObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RealtimeSession => f.write_str("realtime.session"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseGaObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "realtime.session" => Ok(Self::RealtimeSession),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseGaObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeSessionCreateResponseGaObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeSessionCreateResponseGaOutputModalitiesItem`"]
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
pub enum RealtimeSessionCreateResponseGaOutputModalitiesItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseGaOutputModalitiesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Audio => f.write_str("audio"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseGaOutputModalitiesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseGaOutputModalitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateResponseGaOutputModalitiesItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "How the model chooses tools. Provide one of the string modes or force a specific\nfunction/MCP tool.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeSessionCreateResponseGaToolChoice {
    Options(ToolChoiceOptions),
    Function(ToolChoiceFunction),
    Mcp(ToolChoiceMcp),
}
impl ::std::convert::From<ToolChoiceOptions> for RealtimeSessionCreateResponseGaToolChoice {
    fn from(value: ToolChoiceOptions) -> Self {
        Self::Options(value)
    }
}
impl ::std::convert::From<ToolChoiceFunction> for RealtimeSessionCreateResponseGaToolChoice {
    fn from(value: ToolChoiceFunction) -> Self {
        Self::Function(value)
    }
}
impl ::std::convert::From<ToolChoiceMcp> for RealtimeSessionCreateResponseGaToolChoice {
    fn from(value: ToolChoiceMcp) -> Self {
        Self::Mcp(value)
    }
}
#[doc = "`RealtimeSessionCreateResponseGaToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeSessionCreateResponseGaToolsItem {
    RealtimeFunctionTool(RealtimeFunctionTool),
    McpTool(McpTool),
}
impl ::std::convert::From<RealtimeFunctionTool> for RealtimeSessionCreateResponseGaToolsItem {
    fn from(value: RealtimeFunctionTool) -> Self {
        Self::RealtimeFunctionTool(value)
    }
}
impl ::std::convert::From<McpTool> for RealtimeSessionCreateResponseGaToolsItem {
    fn from(value: McpTool) -> Self {
        Self::McpTool(value)
    }
}
#[doc = "The type of session to create. Always `realtime` for the Realtime API.\n"]
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
pub enum RealtimeSessionCreateResponseGaType {
    #[serde(rename = "realtime")]
    Realtime,
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseGaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Realtime => f.write_str("realtime"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseGaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "realtime" => Ok(Self::Realtime),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseGaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeSessionCreateResponseGaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeSessionCreateResponseIncludeItem`"]
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
pub enum RealtimeSessionCreateResponseIncludeItem {
    #[serde(rename = "item.input_audio_transcription.logprobs")]
    ItemInputAudioTranscriptionLogprobs,
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ItemInputAudioTranscriptionLogprobs => {
                f.write_str("item.input_audio_transcription.logprobs")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "item.input_audio_transcription.logprobs" => {
                Ok(Self::ItemInputAudioTranscriptionLogprobs)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeSessionCreateResponseIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeSessionCreateResponseMaxOutputTokens {
    Integer(i64),
    String(RealtimeSessionCreateResponseMaxOutputTokensString),
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseMaxOutputTokens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseMaxOutputTokens {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseMaxOutputTokens {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateResponseMaxOutputTokens
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<i64> for RealtimeSessionCreateResponseMaxOutputTokens {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<RealtimeSessionCreateResponseMaxOutputTokensString>
    for RealtimeSessionCreateResponseMaxOutputTokens
{
    fn from(value: RealtimeSessionCreateResponseMaxOutputTokensString) -> Self {
        Self::String(value)
    }
}
#[doc = "`RealtimeSessionCreateResponseMaxOutputTokensString`"]
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
pub enum RealtimeSessionCreateResponseMaxOutputTokensString {
    #[serde(rename = "inf")]
    Inf,
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseMaxOutputTokensString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inf => f.write_str("inf"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseMaxOutputTokensString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inf" => Ok(Self::Inf),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseMaxOutputTokensString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateResponseMaxOutputTokensString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeSessionCreateResponseOutputModalitiesItem`"]
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
pub enum RealtimeSessionCreateResponseOutputModalitiesItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
impl ::std::fmt::Display for RealtimeSessionCreateResponseOutputModalitiesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Audio => f.write_str("audio"),
        }
    }
}
impl ::std::str::FromStr for RealtimeSessionCreateResponseOutputModalitiesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateResponseOutputModalitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateResponseOutputModalitiesItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server\nVAD means that the model will detect the start and end of speech based on\naudio volume and respond at the end of user speech.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeSessionCreateResponseTurnDetection {
    #[doc = "Amount of audio to include before the VAD detected speech (in\nmilliseconds). Defaults to 300ms.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prefix_padding_ms: ::std::option::Option<i64>,
    #[doc = "Duration of silence to detect speech stop (in milliseconds). Defaults\nto 500ms. With shorter values the model will respond more quickly,\nbut may jump in on short pauses from the user.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub silence_duration_ms: ::std::option::Option<i64>,
    #[doc = "Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A\nhigher threshold will require louder audio to activate the model, and\nthus might perform better in noisy environments.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub threshold: ::std::option::Option<f64>,
    #[doc = "Type of turn detection, only `server_vad` is currently supported.\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
#[doc = "Realtime transcription session object configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequest {
    #[doc = "The set of items to include in the transcription. Current available items are:\n`item.input_audio_transcription.logprobs`\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<RealtimeTranscriptionSessionCreateRequestIncludeItem>,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,\nsingle channel (mono), and little-endian byte order.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio_format:
        ::std::option::Option<RealtimeTranscriptionSessionCreateRequestInputAudioFormat>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio_noise_reduction:
        ::std::option::Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction>,
    #[doc = "Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio_transcription: ::std::option::Option<AudioTranscription>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub turn_detection:
        ::std::option::Option<RealtimeTranscriptionSessionCreateRequestTurnDetection>,
}
#[doc = "Realtime transcription session object configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestGa {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<RealtimeTranscriptionSessionCreateRequestGaAudio>,
    #[doc = "Additional fields to include in server outputs.\n\n`item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<RealtimeTranscriptionSessionCreateRequestGaIncludeItem>,
    #[doc = "The type of session to create. Always `transcription` for transcription sessions.\n"]
    #[serde(rename = "type")]
    pub type_: RealtimeTranscriptionSessionCreateRequestGaType,
}
#[doc = "Configuration for input and output audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestGaAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<RealtimeTranscriptionSessionCreateRequestGaAudioInput>,
}
#[doc = "`RealtimeTranscriptionSessionCreateRequestGaAudioInput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestGaAudioInput {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub noise_reduction:
        ::std::option::Option<RealtimeTranscriptionSessionCreateRequestGaAudioInputNoiseReduction>,
    #[doc = "Configuration for input audio transcription, defaults to off and can be set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs asynchronously through [the /audio/transcriptions endpoint](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription: ::std::option::Option<AudioTranscription>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub turn_detection: ::std::option::Option<RealtimeTurnDetection>,
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestGaAudioInputNoiseReduction {
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<NoiseReductionType>,
}
#[doc = "`RealtimeTranscriptionSessionCreateRequestGaIncludeItem`"]
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
pub enum RealtimeTranscriptionSessionCreateRequestGaIncludeItem {
    #[serde(rename = "item.input_audio_transcription.logprobs")]
    ItemInputAudioTranscriptionLogprobs,
}
impl ::std::fmt::Display for RealtimeTranscriptionSessionCreateRequestGaIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ItemInputAudioTranscriptionLogprobs => {
                f.write_str("item.input_audio_transcription.logprobs")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeTranscriptionSessionCreateRequestGaIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "item.input_audio_transcription.logprobs" => {
                Ok(Self::ItemInputAudioTranscriptionLogprobs)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranscriptionSessionCreateRequestGaIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranscriptionSessionCreateRequestGaIncludeItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of session to create. Always `transcription` for transcription sessions.\n"]
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
pub enum RealtimeTranscriptionSessionCreateRequestGaType {
    #[serde(rename = "transcription")]
    Transcription,
}
impl ::std::fmt::Display for RealtimeTranscriptionSessionCreateRequestGaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transcription => f.write_str("transcription"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranscriptionSessionCreateRequestGaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcription" => Ok(Self::Transcription),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranscriptionSessionCreateRequestGaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranscriptionSessionCreateRequestGaType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeTranscriptionSessionCreateRequestIncludeItem`"]
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
pub enum RealtimeTranscriptionSessionCreateRequestIncludeItem {
    #[serde(rename = "item.input_audio_transcription.logprobs")]
    ItemInputAudioTranscriptionLogprobs,
}
impl ::std::fmt::Display for RealtimeTranscriptionSessionCreateRequestIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ItemInputAudioTranscriptionLogprobs => {
                f.write_str("item.input_audio_transcription.logprobs")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeTranscriptionSessionCreateRequestIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "item.input_audio_transcription.logprobs" => {
                Ok(Self::ItemInputAudioTranscriptionLogprobs)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranscriptionSessionCreateRequestIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranscriptionSessionCreateRequestIncludeItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,\nsingle channel (mono), and little-endian byte order.\n"]
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
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioFormat {
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}
impl ::std::fmt::Display for RealtimeTranscriptionSessionCreateRequestInputAudioFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pcm16 => f.write_str("pcm16"),
            Self::G711Ulaw => f.write_str("g711_ulaw"),
            Self::G711Alaw => f.write_str("g711_alaw"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranscriptionSessionCreateRequestInputAudioFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pcm16" => Ok(Self::Pcm16),
            "g711_ulaw" => Ok(Self::G711Ulaw),
            "g711_alaw" => Ok(Self::G711Alaw),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranscriptionSessionCreateRequestInputAudioFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranscriptionSessionCreateRequestInputAudioFormat
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction {
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<NoiseReductionType>,
}
#[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestTurnDetection {
    #[doc = "Amount of audio to include before the VAD detected speech (in\nmilliseconds). Defaults to 300ms.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prefix_padding_ms: ::std::option::Option<i64>,
    #[doc = "Duration of silence to detect speech stop (in milliseconds). Defaults\nto 500ms. With shorter values the model will respond more quickly,\nbut may jump in on short pauses from the user.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub silence_duration_ms: ::std::option::Option<i64>,
    #[doc = "Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A\nhigher threshold will require louder audio to activate the model, and\nthus might perform better in noisy environments.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub threshold: ::std::option::Option<f64>,
    #[doc = "Type of turn detection. Only `server_vad` is currently supported for transcription sessions.\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RealtimeTranscriptionSessionCreateRequestTurnDetectionType>,
}
#[doc = "Type of turn detection. Only `server_vad` is currently supported for transcription sessions.\n"]
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
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionType {
    #[serde(rename = "server_vad")]
    ServerVad,
}
impl ::std::fmt::Display for RealtimeTranscriptionSessionCreateRequestTurnDetectionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ServerVad => f.write_str("server_vad"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranscriptionSessionCreateRequestTurnDetectionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "server_vad" => Ok(Self::ServerVad),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranscriptionSessionCreateRequestTurnDetectionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranscriptionSessionCreateRequestTurnDetectionType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A new Realtime transcription session configuration.\n\nWhen a session is created on the server via REST API, the session object\nalso contains an ephemeral key. Default TTL for keys is 10 minutes. This\nproperty is not present when a session is updated via the WebSocket API.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateResponse {
    pub client_secret: RealtimeTranscriptionSessionCreateResponseClientSecret,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio_format: ::std::option::Option<::std::string::String>,
    #[doc = "Configuration of the transcription model.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio_transcription: ::std::option::Option<AudioTranscriptionResponse>,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub modalities: ::std::vec::Vec<RealtimeTranscriptionSessionCreateResponseModalitiesItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub turn_detection:
        ::std::option::Option<RealtimeTranscriptionSessionCreateResponseTurnDetection>,
}
#[doc = "Ephemeral key returned by the API. Only present when the session is\ncreated on the server via REST API.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateResponseClientSecret {
    #[doc = "Timestamp for when the token expires. Currently, all tokens expire\nafter one minute.\n"]
    pub expires_at: i64,
    #[doc = "Ephemeral key usable in client environments to authenticate connections\nto the Realtime API. Use this in client-side environments rather than\na standard API token, which should only be used server-side.\n"]
    pub value: ::std::string::String,
}
#[doc = "A Realtime transcription session configuration object.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateResponseGa {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<RealtimeTranscriptionSessionCreateResponseGaAudio>,
    #[doc = "Expiration timestamp for the session, in seconds since epoch."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "Unique identifier for the session that looks like `sess_1234567890abcdef`.\n"]
    pub id: ::std::string::String,
    #[doc = "Additional fields to include in server outputs.\n- `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include: ::std::option::Option<
        ::std::vec::Vec<RealtimeTranscriptionSessionCreateResponseGaIncludeItem>,
    >,
    #[doc = "The object type. Always `realtime.transcription_session`."]
    pub object: ::std::string::String,
    #[doc = "The type of session. Always `transcription` for transcription sessions.\n"]
    #[serde(rename = "type")]
    pub type_: RealtimeTranscriptionSessionCreateResponseGaType,
}
#[doc = "Configuration for input audio for the session.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateResponseGaAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<RealtimeTranscriptionSessionCreateResponseGaAudioInput>,
}
#[doc = "`RealtimeTranscriptionSessionCreateResponseGaAudioInput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateResponseGaAudioInput {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeAudioFormats>,
    #[doc = "Configuration for input audio noise reduction.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub noise_reduction:
        ::std::option::Option<RealtimeTranscriptionSessionCreateResponseGaAudioInputNoiseReduction>,
    #[doc = "Configuration of the transcription model.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription: ::std::option::Option<AudioTranscriptionResponse>,
    #[doc = "Configuration for turn detection. For `gpt-realtime-whisper`, this must be `null`; VAD is not supported.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub turn_detection:
        ::std::option::Option<RealtimeTranscriptionSessionCreateResponseGaAudioInputTurnDetection>,
}
#[doc = "Configuration for input audio noise reduction.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateResponseGaAudioInputNoiseReduction {
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<NoiseReductionType>,
}
#[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server\nVAD means that the model will detect the start and end of speech based on\naudio volume and respond at the end of user speech. For `gpt-realtime-whisper`, this must be `null`; VAD is not supported.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateResponseGaAudioInputTurnDetection {
    #[doc = "Amount of audio to include before the VAD detected speech (in\nmilliseconds). Defaults to 300ms.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prefix_padding_ms: ::std::option::Option<i64>,
    #[doc = "Duration of silence to detect speech stop (in milliseconds). Defaults\nto 500ms. With shorter values the model will respond more quickly,\nbut may jump in on short pauses from the user.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub silence_duration_ms: ::std::option::Option<i64>,
    #[doc = "Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A\nhigher threshold will require louder audio to activate the model, and\nthus might perform better in noisy environments.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub threshold: ::std::option::Option<f64>,
    #[doc = "Type of turn detection, only `server_vad` is currently supported.\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
#[doc = "`RealtimeTranscriptionSessionCreateResponseGaIncludeItem`"]
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
pub enum RealtimeTranscriptionSessionCreateResponseGaIncludeItem {
    #[serde(rename = "item.input_audio_transcription.logprobs")]
    ItemInputAudioTranscriptionLogprobs,
}
impl ::std::fmt::Display for RealtimeTranscriptionSessionCreateResponseGaIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ItemInputAudioTranscriptionLogprobs => {
                f.write_str("item.input_audio_transcription.logprobs")
            }
        }
    }
}
impl ::std::str::FromStr for RealtimeTranscriptionSessionCreateResponseGaIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "item.input_audio_transcription.logprobs" => {
                Ok(Self::ItemInputAudioTranscriptionLogprobs)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranscriptionSessionCreateResponseGaIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranscriptionSessionCreateResponseGaIncludeItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of session. Always `transcription` for transcription sessions.\n"]
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
pub enum RealtimeTranscriptionSessionCreateResponseGaType {
    #[serde(rename = "transcription")]
    Transcription,
}
impl ::std::fmt::Display for RealtimeTranscriptionSessionCreateResponseGaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transcription => f.write_str("transcription"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranscriptionSessionCreateResponseGaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcription" => Ok(Self::Transcription),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranscriptionSessionCreateResponseGaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranscriptionSessionCreateResponseGaType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeTranscriptionSessionCreateResponseModalitiesItem`"]
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
pub enum RealtimeTranscriptionSessionCreateResponseModalitiesItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
impl ::std::fmt::Display for RealtimeTranscriptionSessionCreateResponseModalitiesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Audio => f.write_str("audio"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranscriptionSessionCreateResponseModalitiesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranscriptionSessionCreateResponseModalitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranscriptionSessionCreateResponseModalitiesItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server\nVAD means that the model will detect the start and end of speech based on\naudio volume and respond at the end of user speech.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateResponseTurnDetection {
    #[doc = "Amount of audio to include before the VAD detected speech (in\nmilliseconds). Defaults to 300ms.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prefix_padding_ms: ::std::option::Option<i64>,
    #[doc = "Duration of silence to detect speech stop (in milliseconds). Defaults\nto 500ms. With shorter values the model will respond more quickly,\nbut may jump in on short pauses from the user.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub silence_duration_ms: ::std::option::Option<i64>,
    #[doc = "Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A\nhigher threshold will require louder audio to activate the model, and\nthus might perform better in noisy environments.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub threshold: ::std::option::Option<f64>,
    #[doc = "Type of turn detection, only `server_vad` is currently supported.\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
#[doc = "A Realtime translation client event.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeTranslationClientEvent {
    SessionUpdate(RealtimeTranslationClientEventSessionUpdate),
    InputAudioBufferAppend(RealtimeTranslationClientEventInputAudioBufferAppend),
    SessionClose(RealtimeTranslationClientEventSessionClose),
}
impl ::std::convert::From<RealtimeTranslationClientEventSessionUpdate>
    for RealtimeTranslationClientEvent
{
    fn from(value: RealtimeTranslationClientEventSessionUpdate) -> Self {
        Self::SessionUpdate(value)
    }
}
impl ::std::convert::From<RealtimeTranslationClientEventInputAudioBufferAppend>
    for RealtimeTranslationClientEvent
{
    fn from(value: RealtimeTranslationClientEventInputAudioBufferAppend) -> Self {
        Self::InputAudioBufferAppend(value)
    }
}
impl ::std::convert::From<RealtimeTranslationClientEventSessionClose>
    for RealtimeTranslationClientEvent
{
    fn from(value: RealtimeTranslationClientEventSessionClose) -> Self {
        Self::SessionClose(value)
    }
}
#[doc = "Send this event to append audio bytes to the translation session input audio buffer.\n\nWebSocket translation sessions accept base64-encoded 24 kHz PCM16 mono\nlittle-endian raw audio bytes. Unsupported websocket audio formats return a\nvalidation error because lower-quality audio materially degrades translation\nquality.\n\nTranslation consumes 200 ms engine frames. For best realtime behavior, append\naudio in 200 ms chunks. If a chunk is shorter, the server buffers it until it\nhas enough audio for one frame. If a chunk is longer, the server splits it into\n200 ms frames and enqueues them back-to-back.\n\nKeep appending silence while the session is active. If a client stops sending\naudio and later resumes, model time treats the resumed audio as contiguous with\nthe previous audio rather than as a real-world pause.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationClientEventInputAudioBufferAppend {
    #[doc = "Base64-encoded 24 kHz PCM16 mono audio bytes."]
    pub audio: ::std::string::String,
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id:
        ::std::option::Option<RealtimeTranslationClientEventInputAudioBufferAppendEventId>,
    #[doc = "The event type, must be `session.input_audio_buffer.append`."]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationClientEventInputAudioBufferAppendType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeTranslationClientEventInputAudioBufferAppendEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeTranslationClientEventInputAudioBufferAppendEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeTranslationClientEventInputAudioBufferAppendEventId>
    for ::std::string::String
{
    fn from(value: RealtimeTranslationClientEventInputAudioBufferAppendEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeTranslationClientEventInputAudioBufferAppendEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationClientEventInputAudioBufferAppendEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationClientEventInputAudioBufferAppendEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for RealtimeTranslationClientEventInputAudioBufferAppendEventId
{
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
#[doc = "The event type, must be `session.input_audio_buffer.append`."]
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
pub enum RealtimeTranslationClientEventInputAudioBufferAppendType {
    #[serde(rename = "session.input_audio_buffer.append")]
    SessionInputAudioBufferAppend,
}
impl ::std::fmt::Display for RealtimeTranslationClientEventInputAudioBufferAppendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInputAudioBufferAppend => f.write_str("session.input_audio_buffer.append"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationClientEventInputAudioBufferAppendType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.input_audio_buffer.append" => Ok(Self::SessionInputAudioBufferAppend),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationClientEventInputAudioBufferAppendType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationClientEventInputAudioBufferAppendType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Gracefully close the realtime translation session. The server flushes pending\ninput audio and emits any remaining translated output before closing the\nsession.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationClientEventSessionClose {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeTranslationClientEventSessionCloseEventId>,
    #[doc = "The event type, must be `session.close`."]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationClientEventSessionCloseType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeTranslationClientEventSessionCloseEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeTranslationClientEventSessionCloseEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeTranslationClientEventSessionCloseEventId>
    for ::std::string::String
{
    fn from(value: RealtimeTranslationClientEventSessionCloseEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeTranslationClientEventSessionCloseEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationClientEventSessionCloseEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationClientEventSessionCloseEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeTranslationClientEventSessionCloseEventId {
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
#[doc = "The event type, must be `session.close`."]
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
pub enum RealtimeTranslationClientEventSessionCloseType {
    #[serde(rename = "session.close")]
    SessionClose,
}
impl ::std::fmt::Display for RealtimeTranslationClientEventSessionCloseType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionClose => f.write_str("session.close"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationClientEventSessionCloseType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.close" => Ok(Self::SessionClose),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationClientEventSessionCloseType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationClientEventSessionCloseType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Send this event to update the translation session configuration. Translation\nsessions support updates to `audio.output.language`, `audio.input.transcription`,\nand `audio.input.noise_reduction`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationClientEventSessionUpdate {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<RealtimeTranslationClientEventSessionUpdateEventId>,
    #[doc = "Translation session fields to update. The session `type` and `model` are set\nat creation and cannot be changed with `session.update`.\n"]
    pub session: RealtimeTranslationSessionUpdateRequest,
    #[doc = "The event type, must be `session.update`."]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationClientEventSessionUpdateType,
}
#[doc = "Optional client-generated ID used to identify this event."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RealtimeTranslationClientEventSessionUpdateEventId(::std::string::String);
impl ::std::ops::Deref for RealtimeTranslationClientEventSessionUpdateEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RealtimeTranslationClientEventSessionUpdateEventId>
    for ::std::string::String
{
    fn from(value: RealtimeTranslationClientEventSessionUpdateEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RealtimeTranslationClientEventSessionUpdateEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationClientEventSessionUpdateEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationClientEventSessionUpdateEventId
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RealtimeTranslationClientEventSessionUpdateEventId {
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
#[doc = "The event type, must be `session.update`."]
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
pub enum RealtimeTranslationClientEventSessionUpdateType {
    #[serde(rename = "session.update")]
    SessionUpdate,
}
impl ::std::fmt::Display for RealtimeTranslationClientEventSessionUpdateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionUpdate => f.write_str("session.update"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationClientEventSessionUpdateType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.update" => Ok(Self::SessionUpdate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationClientEventSessionUpdateType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationClientEventSessionUpdateType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Create a translation session and client secret for the Realtime API.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationClientSecretCreateRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<ClientSecretExpiration>,
    pub session: RealtimeTranslationSessionCreateRequest,
}
#[doc = "Response from creating a translation session and client secret for the Realtime API.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationClientSecretCreateResponse {
    #[doc = "Expiration timestamp for the client secret, in seconds since epoch."]
    pub expires_at: i64,
    pub session: RealtimeTranslationSession,
    #[doc = "The generated client secret value."]
    pub value: ::std::string::String,
}
#[doc = "A Realtime translation server event.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeTranslationServerEvent {
    ServerEventError(RealtimeServerEventError),
    TranslationServerEventSessionCreated(RealtimeTranslationServerEventSessionCreated),
    TranslationServerEventSessionUpdated(RealtimeTranslationServerEventSessionUpdated),
    TranslationServerEventSessionClosed(RealtimeTranslationServerEventSessionClosed),
    TranslationServerEventSessionInputTranscriptDelta(
        RealtimeTranslationServerEventSessionInputTranscriptDelta,
    ),
    TranslationServerEventSessionOutputTranscriptDelta(
        RealtimeTranslationServerEventSessionOutputTranscriptDelta,
    ),
    TranslationServerEventSessionOutputAudioDelta(
        RealtimeTranslationServerEventSessionOutputAudioDelta,
    ),
}
impl ::std::convert::From<RealtimeServerEventError> for RealtimeTranslationServerEvent {
    fn from(value: RealtimeServerEventError) -> Self {
        Self::ServerEventError(value)
    }
}
impl ::std::convert::From<RealtimeTranslationServerEventSessionCreated>
    for RealtimeTranslationServerEvent
{
    fn from(value: RealtimeTranslationServerEventSessionCreated) -> Self {
        Self::TranslationServerEventSessionCreated(value)
    }
}
impl ::std::convert::From<RealtimeTranslationServerEventSessionUpdated>
    for RealtimeTranslationServerEvent
{
    fn from(value: RealtimeTranslationServerEventSessionUpdated) -> Self {
        Self::TranslationServerEventSessionUpdated(value)
    }
}
impl ::std::convert::From<RealtimeTranslationServerEventSessionClosed>
    for RealtimeTranslationServerEvent
{
    fn from(value: RealtimeTranslationServerEventSessionClosed) -> Self {
        Self::TranslationServerEventSessionClosed(value)
    }
}
impl ::std::convert::From<RealtimeTranslationServerEventSessionInputTranscriptDelta>
    for RealtimeTranslationServerEvent
{
    fn from(value: RealtimeTranslationServerEventSessionInputTranscriptDelta) -> Self {
        Self::TranslationServerEventSessionInputTranscriptDelta(value)
    }
}
impl ::std::convert::From<RealtimeTranslationServerEventSessionOutputTranscriptDelta>
    for RealtimeTranslationServerEvent
{
    fn from(value: RealtimeTranslationServerEventSessionOutputTranscriptDelta) -> Self {
        Self::TranslationServerEventSessionOutputTranscriptDelta(value)
    }
}
impl ::std::convert::From<RealtimeTranslationServerEventSessionOutputAudioDelta>
    for RealtimeTranslationServerEvent
{
    fn from(value: RealtimeTranslationServerEventSessionOutputAudioDelta) -> Self {
        Self::TranslationServerEventSessionOutputAudioDelta(value)
    }
}
#[doc = "Returned when a realtime translation session is closed.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationServerEventSessionClosed {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, must be `session.closed`."]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationServerEventSessionClosedType,
}
#[doc = "The event type, must be `session.closed`."]
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
pub enum RealtimeTranslationServerEventSessionClosedType {
    #[serde(rename = "session.closed")]
    SessionClosed,
}
impl ::std::fmt::Display for RealtimeTranslationServerEventSessionClosedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionClosed => f.write_str("session.closed"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationServerEventSessionClosedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.closed" => Ok(Self::SessionClosed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationServerEventSessionClosedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationServerEventSessionClosedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a translation session is created. Emitted automatically when a\nnew connection is established as the first server event. This event contains\nthe default translation session configuration.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationServerEventSessionCreated {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The translation session configuration."]
    pub session: RealtimeTranslationSession,
    #[doc = "The event type, must be `session.created`."]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationServerEventSessionCreatedType,
}
#[doc = "The event type, must be `session.created`."]
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
pub enum RealtimeTranslationServerEventSessionCreatedType {
    #[serde(rename = "session.created")]
    SessionCreated,
}
impl ::std::fmt::Display for RealtimeTranslationServerEventSessionCreatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCreated => f.write_str("session.created"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationServerEventSessionCreatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.created" => Ok(Self::SessionCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationServerEventSessionCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationServerEventSessionCreatedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when optional source-language transcript text is available. This event\nis emitted only when `audio.input.transcription` is configured.\n\nTranscript deltas are append-only text fragments. Clients should not insert\nunconditional spaces between deltas.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationServerEventSessionInputTranscriptDelta {
    #[doc = "Append-only source-language transcript text."]
    pub delta: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub elapsed_ms: ::std::option::Option<i64>,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, must be `session.input_transcript.delta`."]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationServerEventSessionInputTranscriptDeltaType,
}
#[doc = "The event type, must be `session.input_transcript.delta`."]
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
pub enum RealtimeTranslationServerEventSessionInputTranscriptDeltaType {
    #[serde(rename = "session.input_transcript.delta")]
    SessionInputTranscriptDelta,
}
impl ::std::fmt::Display for RealtimeTranslationServerEventSessionInputTranscriptDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInputTranscriptDelta => f.write_str("session.input_transcript.delta"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationServerEventSessionInputTranscriptDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.input_transcript.delta" => Ok(Self::SessionInputTranscriptDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for RealtimeTranslationServerEventSessionInputTranscriptDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationServerEventSessionInputTranscriptDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when translated output audio is available. The `delta` contains a\nPCM16 audio chunk whose length can vary. Clients should decode and queue the\ncomplete delta instead of assuming a fixed byte or sample count.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationServerEventSessionOutputAudioDelta {
    #[doc = "Number of audio channels."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub channels: ::std::option::Option<i64>,
    #[doc = "Base64-encoded translated audio data."]
    pub delta: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub elapsed_ms: ::std::option::Option<i64>,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "Audio encoding for `delta`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<RealtimeTranslationServerEventSessionOutputAudioDeltaFormat>,
    #[doc = "Sample rate of the audio delta."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sample_rate: ::std::option::Option<i64>,
    #[doc = "The event type, must be `session.output_audio.delta`."]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationServerEventSessionOutputAudioDeltaType,
}
#[doc = "Audio encoding for `delta`."]
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
pub enum RealtimeTranslationServerEventSessionOutputAudioDeltaFormat {
    #[serde(rename = "pcm16")]
    Pcm16,
}
impl ::std::fmt::Display for RealtimeTranslationServerEventSessionOutputAudioDeltaFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pcm16 => f.write_str("pcm16"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationServerEventSessionOutputAudioDeltaFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pcm16" => Ok(Self::Pcm16),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationServerEventSessionOutputAudioDeltaFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationServerEventSessionOutputAudioDeltaFormat
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The event type, must be `session.output_audio.delta`."]
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
pub enum RealtimeTranslationServerEventSessionOutputAudioDeltaType {
    #[serde(rename = "session.output_audio.delta")]
    SessionOutputAudioDelta,
}
impl ::std::fmt::Display for RealtimeTranslationServerEventSessionOutputAudioDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionOutputAudioDelta => f.write_str("session.output_audio.delta"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationServerEventSessionOutputAudioDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.output_audio.delta" => Ok(Self::SessionOutputAudioDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationServerEventSessionOutputAudioDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationServerEventSessionOutputAudioDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when translated transcript text is available.\n\nTranscript deltas are append-only text fragments. Clients should not insert\nunconditional spaces between deltas.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationServerEventSessionOutputTranscriptDelta {
    #[doc = "Append-only transcript text for the translated output audio."]
    pub delta: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub elapsed_ms: ::std::option::Option<i64>,
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, must be `session.output_transcript.delta`."]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationServerEventSessionOutputTranscriptDeltaType,
}
#[doc = "The event type, must be `session.output_transcript.delta`."]
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
pub enum RealtimeTranslationServerEventSessionOutputTranscriptDeltaType {
    #[serde(rename = "session.output_transcript.delta")]
    SessionOutputTranscriptDelta,
}
impl ::std::fmt::Display for RealtimeTranslationServerEventSessionOutputTranscriptDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionOutputTranscriptDelta => f.write_str("session.output_transcript.delta"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationServerEventSessionOutputTranscriptDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.output_transcript.delta" => Ok(Self::SessionOutputTranscriptDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for RealtimeTranslationServerEventSessionOutputTranscriptDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationServerEventSessionOutputTranscriptDeltaType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a translation session is updated with a `session.update` event,\nunless there is an error.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationServerEventSessionUpdated {
    #[doc = "The unique ID of the server event."]
    pub event_id: ::std::string::String,
    #[doc = "The translation session configuration."]
    pub session: RealtimeTranslationSession,
    #[doc = "The event type, must be `session.updated`."]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationServerEventSessionUpdatedType,
}
#[doc = "The event type, must be `session.updated`."]
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
pub enum RealtimeTranslationServerEventSessionUpdatedType {
    #[serde(rename = "session.updated")]
    SessionUpdated,
}
impl ::std::fmt::Display for RealtimeTranslationServerEventSessionUpdatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionUpdated => f.write_str("session.updated"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationServerEventSessionUpdatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.updated" => Ok(Self::SessionUpdated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationServerEventSessionUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeTranslationServerEventSessionUpdatedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A Realtime translation session. Translation sessions continuously translate input\naudio into the configured output language.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationSession {
    pub audio: RealtimeTranslationSessionAudio,
    #[doc = "Expiration timestamp for the session, in seconds since epoch."]
    pub expires_at: i64,
    #[doc = "Unique identifier for the session that looks like `sess_1234567890abcdef`.\n"]
    pub id: ::std::string::String,
    #[doc = "The Realtime translation model used for this session. This field is set at\nsession creation and cannot be changed with `session.update`.\n"]
    pub model: ::std::string::String,
    #[doc = "The session type. Always `translation` for Realtime translation sessions.\n"]
    #[serde(rename = "type")]
    pub type_: RealtimeTranslationSessionType,
}
#[doc = "Configuration for translation input and output audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<RealtimeTranslationSessionAudioInput>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<RealtimeTranslationSessionAudioOutput>,
}
#[doc = "`RealtimeTranslationSessionAudioInput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionAudioInput {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub noise_reduction: ::std::option::Option<RealtimeTranslationSessionAudioInputNoiseReduction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription: ::std::option::Option<RealtimeTranslationSessionAudioInputTranscription>,
}
#[doc = "Optional input noise reduction.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationSessionAudioInputNoiseReduction {
    #[serde(rename = "type")]
    pub type_: NoiseReductionType,
}
#[doc = "Optional source-language transcription. When configured, the server emits\n`session.input_transcript.delta` events. Translation itself still runs from\nthe input audio stream.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationSessionAudioInputTranscription {
    #[doc = "The transcription model used for source transcript deltas."]
    pub model: ::std::string::String,
}
#[doc = "`RealtimeTranslationSessionAudioOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionAudioOutput {
    #[doc = "Target language for translated output audio and transcript deltas.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
}
#[doc = "Realtime translation session configuration. Translation sessions stream source\naudio in and translated audio plus transcript deltas out continuously.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationSessionCreateRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<RealtimeTranslationSessionCreateRequestAudio>,
    #[doc = "The Realtime translation model used for this session.\n"]
    pub model: ::std::string::String,
}
#[doc = "Configuration for translation input and output audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionCreateRequestAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<RealtimeTranslationSessionCreateRequestAudioInput>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<RealtimeTranslationSessionCreateRequestAudioOutput>,
}
#[doc = "`RealtimeTranslationSessionCreateRequestAudioInput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionCreateRequestAudioInput {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub noise_reduction:
        ::std::option::Option<RealtimeTranslationSessionCreateRequestAudioInputNoiseReduction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription:
        ::std::option::Option<RealtimeTranslationSessionCreateRequestAudioInputTranscription>,
}
#[doc = "Optional input noise reduction. Set to `null` to disable it.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationSessionCreateRequestAudioInputNoiseReduction {
    #[serde(rename = "type")]
    pub type_: NoiseReductionType,
}
#[doc = "Optional source-language transcription. When configured, the server emits\n`session.input_transcript.delta` events. Translation itself still runs from\nthe input audio stream.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationSessionCreateRequestAudioInputTranscription {
    #[doc = "The transcription model to use for source transcript deltas."]
    pub model: ::std::string::String,
}
#[doc = "`RealtimeTranslationSessionCreateRequestAudioOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionCreateRequestAudioOutput {
    #[doc = "Target language for translated output audio and transcript deltas.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
}
#[doc = "The session type. Always `translation` for Realtime translation sessions.\n"]
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
pub enum RealtimeTranslationSessionType {
    #[serde(rename = "translation")]
    Translation,
}
impl ::std::fmt::Display for RealtimeTranslationSessionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Translation => f.write_str("translation"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTranslationSessionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "translation" => Ok(Self::Translation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTranslationSessionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeTranslationSessionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Realtime translation session fields that can be updated with `session.update`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionUpdateRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<RealtimeTranslationSessionUpdateRequestAudio>,
}
#[doc = "Configuration for translation input and output audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionUpdateRequestAudio {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<RealtimeTranslationSessionUpdateRequestAudioInput>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<RealtimeTranslationSessionUpdateRequestAudioOutput>,
}
#[doc = "`RealtimeTranslationSessionUpdateRequestAudioInput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionUpdateRequestAudioInput {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub noise_reduction:
        ::std::option::Option<RealtimeTranslationSessionUpdateRequestAudioInputNoiseReduction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription:
        ::std::option::Option<RealtimeTranslationSessionUpdateRequestAudioInputTranscription>,
}
#[doc = "Optional input noise reduction. Set to `null` to disable it.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationSessionUpdateRequestAudioInputNoiseReduction {
    #[serde(rename = "type")]
    pub type_: NoiseReductionType,
}
#[doc = "Optional source-language transcription. When configured, the server emits\n`session.input_transcript.delta` events. Translation itself still runs from\nthe input audio stream.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeTranslationSessionUpdateRequestAudioInputTranscription {
    #[doc = "The transcription model to use for source transcript deltas."]
    pub model: ::std::string::String,
}
#[doc = "`RealtimeTranslationSessionUpdateRequestAudioOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RealtimeTranslationSessionUpdateRequestAudioOutput {
    #[doc = "Target language for translated output audio and transcript deltas.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
}
#[doc = "When the number of tokens in a conversation exceeds the model's input token limit, the conversation be truncated, meaning messages (starting from the oldest) will not be included in the model's context. A 32k context model with 4,096 max output tokens can only include 28,224 tokens in the context before truncation occurs.\n\nClients can configure truncation behavior to truncate with a lower max token limit, which is an effective way to control token usage and cost.\n\nTruncation will reduce the number of cached tokens on the next turn (busting the cache), since messages are dropped from the beginning of the context. However, clients can also configure truncation to retain messages up to a fraction of the maximum context size, which will reduce the need for future truncations and thus improve the cache rate.\n\nTruncation can be disabled entirely, which means the server will never truncate but would instead return an error if the conversation exceeds the model's input token limit.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RealtimeTruncation {
    String(RealtimeTruncationString),
    RetentionRatioTruncation {
        #[doc = "Fraction of post-instruction conversation tokens to retain (`0.0` - `1.0`) when the conversation exceeds the input token limit. Setting this to `0.8` means that messages will be dropped until 80% of the maximum allowed tokens are used. This helps reduce the frequency of truncations and improve cache rates.\n"]
        retention_ratio: f64,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        token_limits: ::std::option::Option<RetentionRatioTruncationTokenLimits>,
        #[doc = "Use retention ratio truncation."]
        #[serde(rename = "type")]
        type_: RetentionRatioTruncationType,
    },
}
impl ::std::convert::From<RealtimeTruncationString> for RealtimeTruncation {
    fn from(value: RealtimeTruncationString) -> Self {
        Self::String(value)
    }
}
#[doc = "The truncation strategy to use for the session. `auto` is the default truncation strategy. `disabled` will disable truncation and emit errors when the conversation exceeds the input token limit."]
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
pub enum RealtimeTruncationString {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for RealtimeTruncationString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTruncationString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTruncationString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeTruncationString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeTurnDetection`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct RealtimeTurnDetection(pub ::std::option::Option<RealtimeTurnDetectionValue>);
impl ::std::ops::Deref for RealtimeTurnDetection {
    type Target = ::std::option::Option<RealtimeTurnDetectionValue>;
    fn deref(&self) -> &::std::option::Option<RealtimeTurnDetectionValue> {
        &self.0
    }
}
impl ::std::convert::From<RealtimeTurnDetection>
    for ::std::option::Option<RealtimeTurnDetectionValue>
{
    fn from(value: RealtimeTurnDetection) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<RealtimeTurnDetectionValue>>
    for RealtimeTurnDetection
{
    fn from(value: ::std::option::Option<RealtimeTurnDetectionValue>) -> Self {
        Self(value)
    }
}
#[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\n\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\n\nSemantic VAD is more advanced and uses a turn detection model (in conjunction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n\nFor `gpt-realtime-whisper` transcription sessions, turn detection must be\nset to `null`; VAD is not supported.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum RealtimeTurnDetectionValue {
    #[doc = "Server VAD\n\nServer-side voice activity detection (VAD) which flips on when user speech is detected and off after a period of silence."]
    #[serde(rename = "server_vad")]
    ServerVad {
        #[doc = "Whether or not to automatically generate a response when a VAD stop event occurs. If `interrupt_response` is set to `false` this may fail to create a response if the model is already responding.\n\nIf both `create_response` and `interrupt_response` are set to `false`, the model will never respond automatically but VAD events will still be emitted.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        create_response: ::std::option::Option<bool>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        idle_timeout_ms: ::std::option::Option<i64>,
        #[doc = "Whether or not to automatically interrupt (cancel) any ongoing response with output to the default\nconversation (i.e. `conversation` of `auto`) when a VAD start event occurs. If `true` then the response will be cancelled, otherwise it will continue until complete.\n\nIf both `create_response` and `interrupt_response` are set to `false`, the model will never respond automatically but VAD events will still be emitted.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        interrupt_response: ::std::option::Option<bool>,
        #[doc = "Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in\nmilliseconds). Defaults to 300ms.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        prefix_padding_ms: ::std::option::Option<i64>,
        #[doc = "Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults\nto 500ms. With shorter values the model will respond more quickly,\nbut may jump in on short pauses from the user.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        silence_duration_ms: ::std::option::Option<i64>,
        #[doc = "Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A\nhigher threshold will require louder audio to activate the model, and\nthus might perform better in noisy environments.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        threshold: ::std::option::Option<f64>,
    },
    #[doc = "Semantic VAD\n\nServer-side semantic turn detection which uses a model to determine when the user has finished speaking."]
    #[serde(rename = "semantic_vad")]
    SemanticVad {
        #[doc = "Whether or not to automatically generate a response when a VAD stop event occurs.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        create_response: ::std::option::Option<bool>,
        #[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`. `low`, `medium`, and `high` have max timeouts of 8s, 4s, and 2s respectively.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        eagerness: ::std::option::Option<RealtimeTurnDetectionValueEagerness>,
        #[doc = "Whether or not to automatically interrupt any ongoing response with output to the default\nconversation (i.e. `conversation` of `auto`) when a VAD start event occurs.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        interrupt_response: ::std::option::Option<bool>,
    },
}
#[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`. `low`, `medium`, and `high` have max timeouts of 8s, 4s, and 2s respectively.\n"]
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
pub enum RealtimeTurnDetectionValueEagerness {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for RealtimeTurnDetectionValueEagerness {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for RealtimeTurnDetectionValueEagerness {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeTurnDetectionValueEagerness {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeTurnDetectionValueEagerness {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResponsePromptVariables`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ResponsePromptVariables(pub ::std::option::Option<ResponsePromptVariablesValue>);
impl ::std::ops::Deref for ResponsePromptVariables {
    type Target = ::std::option::Option<ResponsePromptVariablesValue>;
    fn deref(&self) -> &::std::option::Option<ResponsePromptVariablesValue> {
        &self.0
    }
}
impl ::std::convert::From<ResponsePromptVariables>
    for ::std::option::Option<ResponsePromptVariablesValue>
{
    fn from(value: ResponsePromptVariables) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<ResponsePromptVariablesValue>>
    for ResponsePromptVariables
{
    fn from(value: ::std::option::Option<ResponsePromptVariablesValue>) -> Self {
        Self(value)
    }
}
#[doc = "Optional map of values to substitute in for variables in your\nprompt. The substitution values can either be strings, or other\nResponse input types like images or files.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ResponsePromptVariablesValue(
    pub ::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>,
);
impl ::std::ops::Deref for ResponsePromptVariablesValue {
    type Target =
        ::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>;
    fn deref(
        &self,
    ) -> &::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>
    {
        &self.0
    }
}
impl ::std::convert::From<ResponsePromptVariablesValue>
    for ::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>
{
    fn from(value: ResponsePromptVariablesValue) -> Self {
        value.0
    }
}
impl
    ::std::convert::From<
        ::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>,
    > for ResponsePromptVariablesValue
{
    fn from(
        value: ::std::collections::HashMap<
            ::std::string::String,
            ResponsePromptVariablesValueValue,
        >,
    ) -> Self {
        Self(value)
    }
}
#[doc = "`ResponsePromptVariablesValueValue`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ResponsePromptVariablesValueValue {
    String(::std::string::String),
    InputTextContent(InputTextContent),
    InputImageContent(InputImageContent),
    InputFileContent(InputFileContent),
}
impl ::std::convert::From<InputTextContent> for ResponsePromptVariablesValueValue {
    fn from(value: InputTextContent) -> Self {
        Self::InputTextContent(value)
    }
}
impl ::std::convert::From<InputImageContent> for ResponsePromptVariablesValueValue {
    fn from(value: InputImageContent) -> Self {
        Self::InputImageContent(value)
    }
}
impl ::std::convert::From<InputFileContent> for ResponsePromptVariablesValueValue {
    fn from(value: InputFileContent) -> Self {
        Self::InputFileContent(value)
    }
}
#[doc = "Optional custom token limits for this truncation strategy. If not provided, the model's default token limits will be used."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RetentionRatioTruncationTokenLimits {
    #[doc = "Maximum tokens allowed in the conversation after instructions (which including tool definitions). For example, setting this to 5,000 would mean that truncation would occur when the conversation exceeds 5,000 tokens after instructions. This cannot be higher than the model's context window size minus the maximum output tokens."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub post_instructions: ::std::option::Option<u64>,
}
#[doc = "Use retention ratio truncation."]
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
pub enum RetentionRatioTruncationType {
    #[serde(rename = "retention_ratio")]
    RetentionRatio,
}
impl ::std::fmt::Display for RetentionRatioTruncationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RetentionRatio => f.write_str("retention_ratio"),
        }
    }
}
impl ::std::str::FromStr for RetentionRatioTruncationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "retention_ratio" => Ok(Self::RetentionRatio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RetentionRatioTruncationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RetentionRatioTruncationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Session configuration to use for the client secret. Choose either a realtime\nsession or a transcription session.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SessionConfiguration {
    SessionCreateRequestGa(RealtimeSessionCreateRequestGa),
    TranscriptionSessionCreateRequestGa(RealtimeTranscriptionSessionCreateRequestGa),
}
impl ::std::convert::From<RealtimeSessionCreateRequestGa> for SessionConfiguration {
    fn from(value: RealtimeSessionCreateRequestGa) -> Self {
        Self::SessionCreateRequestGa(value)
    }
}
impl ::std::convert::From<RealtimeTranscriptionSessionCreateRequestGa> for SessionConfiguration {
    fn from(value: RealtimeTranscriptionSessionCreateRequestGa) -> Self {
        Self::TranscriptionSessionCreateRequestGa(value)
    }
}
#[doc = "Use this option to force the model to call a specific function.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolChoiceFunction {
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
    #[doc = "For function calling, the type is always `function`."]
    #[serde(rename = "type")]
    pub type_: ToolChoiceFunctionType,
}
#[doc = "For function calling, the type is always `function`."]
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
pub enum ToolChoiceFunctionType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for ToolChoiceFunctionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceFunctionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceFunctionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceFunctionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Use this option to force the model to call a specific tool on a remote MCP server.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolChoiceMcp {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The label of the MCP server to use.\n"]
    pub server_label: ::std::string::String,
    #[doc = "For MCP tools, the type is always `mcp`."]
    #[serde(rename = "type")]
    pub type_: ToolChoiceMcpType,
}
#[doc = "For MCP tools, the type is always `mcp`."]
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
pub enum ToolChoiceMcpType {
    #[serde(rename = "mcp")]
    Mcp,
}
impl ::std::fmt::Display for ToolChoiceMcpType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mcp => f.write_str("mcp"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceMcpType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp" => Ok(Self::Mcp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceMcpType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceMcpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Controls which (if any) tool is called by the model.\n\n`none` means the model will not call any tool and instead generates a message.\n\n`auto` means the model can pick between generating a message or calling one or\nmore tools.\n\n`required` means the model must call one or more tools.\n"]
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
pub enum ToolChoiceOptions {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for ToolChoiceOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceOptions {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "auto" => Ok(Self::Auto),
            "required" => Ok(Self::Required),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceOptions {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceOptions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration options for tracing. Set to null to disable tracing. Once\ntracing is enabled for a session, the configuration cannot be modified.\n\n`auto` will create a trace for the session with default values for the\nworkflow name, group id, and metadata.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum TracingConfiguration {
    String(TracingConfigurationString),
    TracingConfiguration {
        #[doc = "The group id to attach to this trace to enable filtering and\ngrouping in the traces dashboard.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        group_id: ::std::option::Option<::std::string::String>,
        #[doc = "The arbitrary metadata to attach to this trace to enable\nfiltering in the traces dashboard.\n"]
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[doc = "The name of the workflow to attach to this trace. This is used to\nname the trace in the traces dashboard.\n"]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        workflow_name: ::std::option::Option<::std::string::String>,
    },
}
impl ::std::convert::From<TracingConfigurationString> for TracingConfiguration {
    fn from(value: TracingConfigurationString) -> Self {
        Self::String(value)
    }
}
#[doc = "Default tracing mode for the session.\n"]
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
pub enum TracingConfigurationString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for TracingConfigurationString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for TracingConfigurationString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TracingConfigurationString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TracingConfigurationString {
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
#[doc = "A language detected in transcribed audio."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptionLanguage {
    #[doc = "The code of a language detected in the audio."]
    pub code: ::std::string::String,
}
#[doc = "A built-in voice name or a custom voice reference.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged, deny_unknown_fields)]
pub enum VoiceIdsOrCustomVoice {
    VoiceIdsShared(VoiceIdsShared),
    Object {
        #[doc = "The custom voice ID, e.g. `voice_1234`."]
        id: ::std::string::String,
    },
}
impl ::std::convert::From<VoiceIdsShared> for VoiceIdsOrCustomVoice {
    fn from(value: VoiceIdsShared) -> Self {
        Self::VoiceIdsShared(value)
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
