#[doc = "`AddUploadPartRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AddUploadPartRequest {
    #[doc = "The chunk of bytes for this Part.\n"]
    pub data: ::std::string::String,
}
#[doc = "`CompleteUploadRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompleteUploadRequest {
    #[doc = "The optional md5 checksum for the file contents to verify if the bytes uploaded matches what you expect.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub md5: ::std::option::Option<::std::string::String>,
    #[doc = "The ordered list of Part IDs.\n"]
    pub part_ids: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`CreateUploadRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateUploadRequest {
    #[doc = "The number of bytes in the file you are uploading.\n"]
    pub bytes: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<FileExpirationAfter>,
    #[doc = "The name of the file to upload.\n"]
    pub filename: ::std::string::String,
    #[doc = "The MIME type of the file.\n\n\nThis must fall within the supported MIME types for your file purpose. See\nthe supported MIME types for assistants and vision.\n"]
    pub mime_type: ::std::string::String,
    #[doc = "The intended purpose of the uploaded file.\n\nSee the [documentation on File\npurposes](https://developers.openai.com/api/reference/resources/files/methods/create#%28resource%29%20files%20%3E%20%28method%29%20create%20%3E%20%28params%29%200%20%3E%20%28param%29%20purpose%20%3E%20%28schema%29).\n"]
    pub purpose: CreateUploadRequestPurpose,
}
#[doc = "The intended purpose of the uploaded file.\n\nSee the [documentation on File\npurposes](https://developers.openai.com/api/reference/resources/files/methods/create#%28resource%29%20files%20%3E%20%28method%29%20create%20%3E%20%28params%29%200%20%3E%20%28param%29%20purpose%20%3E%20%28schema%29).\n"]
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
pub enum CreateUploadRequestPurpose {
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "vision")]
    Vision,
}
impl ::std::fmt::Display for CreateUploadRequestPurpose {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistants => f.write_str("assistants"),
            Self::Batch => f.write_str("batch"),
            Self::FineTune => f.write_str("fine-tune"),
            Self::Vision => f.write_str("vision"),
        }
    }
}
impl ::std::str::FromStr for CreateUploadRequestPurpose {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistants" => Ok(Self::Assistants),
            "batch" => Ok(Self::Batch),
            "fine-tune" => Ok(Self::FineTune),
            "vision" => Ok(Self::Vision),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateUploadRequestPurpose {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateUploadRequestPurpose {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The expiration policy for a file. By default, files with `purpose=batch` expire after 30 days and all other files are persisted until they are manually deleted."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FileExpirationAfter {
    #[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `created_at`."]
    pub anchor: FileExpirationAfterAnchor,
    #[doc = "The number of seconds after the anchor time that the file will expire. Must be between 3600 (1 hour) and 2592000 (30 days)."]
    pub seconds: i64,
}
#[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `created_at`."]
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
pub enum FileExpirationAfterAnchor {
    #[serde(rename = "created_at")]
    CreatedAt,
}
impl ::std::fmt::Display for FileExpirationAfterAnchor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CreatedAt => f.write_str("created_at"),
        }
    }
}
impl ::std::str::FromStr for FileExpirationAfterAnchor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "created_at" => Ok(Self::CreatedAt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileExpirationAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileExpirationAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The `File` object represents a document that has been uploaded to OpenAI."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OpenAiFile {
    #[doc = "The size of the file, in bytes. In a completed file upload response, this can\nbe null when the file size is not yet available.\n"]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub bytes: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the file was created."]
    pub created_at: i64,
    #[doc = "The Unix timestamp (in seconds) for when the file will expire. In a\ncompleted file upload response, this can be null when no expiry is set.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The name of the file."]
    pub filename: ::std::string::String,
    #[doc = "The file identifier, which can be referenced in the API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always `file`."]
    pub object: OpenAiFileObject,
    #[doc = "The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results`, `vision`, and `user_data`."]
    pub purpose: OpenAiFilePurpose,
    #[doc = "Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`."]
    pub status: OpenAiFileStatus,
    #[doc = "Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`. Completed file upload responses can return null when these details are unset."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status_details: ::std::option::Option<::std::string::String>,
}
#[doc = "The object type, which is always `file`."]
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
pub enum OpenAiFileObject {
    #[serde(rename = "file")]
    File,
}
impl ::std::fmt::Display for OpenAiFileObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::File => f.write_str("file"),
        }
    }
}
impl ::std::str::FromStr for OpenAiFileObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file" => Ok(Self::File),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OpenAiFileObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OpenAiFileObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results`, `vision`, and `user_data`."]
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
pub enum OpenAiFilePurpose {
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "assistants_output")]
    AssistantsOutput,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "batch_output")]
    BatchOutput,
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "fine-tune-results")]
    FineTuneResults,
    #[serde(rename = "vision")]
    Vision,
    #[serde(rename = "user_data")]
    UserData,
}
impl ::std::fmt::Display for OpenAiFilePurpose {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistants => f.write_str("assistants"),
            Self::AssistantsOutput => f.write_str("assistants_output"),
            Self::Batch => f.write_str("batch"),
            Self::BatchOutput => f.write_str("batch_output"),
            Self::FineTune => f.write_str("fine-tune"),
            Self::FineTuneResults => f.write_str("fine-tune-results"),
            Self::Vision => f.write_str("vision"),
            Self::UserData => f.write_str("user_data"),
        }
    }
}
impl ::std::str::FromStr for OpenAiFilePurpose {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistants" => Ok(Self::Assistants),
            "assistants_output" => Ok(Self::AssistantsOutput),
            "batch" => Ok(Self::Batch),
            "batch_output" => Ok(Self::BatchOutput),
            "fine-tune" => Ok(Self::FineTune),
            "fine-tune-results" => Ok(Self::FineTuneResults),
            "vision" => Ok(Self::Vision),
            "user_data" => Ok(Self::UserData),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OpenAiFilePurpose {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OpenAiFilePurpose {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`."]
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
pub enum OpenAiFileStatus {
    #[serde(rename = "uploaded")]
    Uploaded,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for OpenAiFileStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Uploaded => f.write_str("uploaded"),
            Self::Processed => f.write_str("processed"),
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for OpenAiFileStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "uploaded" => Ok(Self::Uploaded),
            "processed" => Ok(Self::Processed),
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OpenAiFileStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OpenAiFileStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The Upload object can accept byte chunks in the form of Parts.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Upload {
    #[doc = "The intended number of bytes to be uploaded."]
    pub bytes: i64,
    #[doc = "The Unix timestamp (in seconds) for when the Upload was created."]
    pub created_at: i64,
    #[doc = "The Unix timestamp (in seconds) for when the Upload will expire."]
    pub expires_at: i64,
    #[doc = "The ready File object after the Upload is completed."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<OpenAiFile>,
    #[doc = "The name of the file to be uploaded."]
    pub filename: ::std::string::String,
    #[doc = "The Upload unique identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always \"upload\"."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<UploadObject>,
    #[doc = "The intended purpose of the file. [Please refer here](https://developers.openai.com/api/reference/resources/files#%28resource%29%20files%20%3E%20%28model%29%20file_object%20%3E%20%28schema%29%20%3E%20%28property%29%20purpose) for acceptable values."]
    pub purpose: ::std::string::String,
    #[doc = "The status of the Upload."]
    pub status: UploadStatus,
}
#[doc = "The object type, which is always \"upload\"."]
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
pub enum UploadObject {
    #[serde(rename = "upload")]
    Upload,
}
impl ::std::fmt::Display for UploadObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Upload => f.write_str("upload"),
        }
    }
}
impl ::std::str::FromStr for UploadObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "upload" => Ok(Self::Upload),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UploadObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UploadObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The upload Part represents a chunk of bytes we can add to an Upload object.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UploadPart {
    #[doc = "The Unix timestamp (in seconds) for when the Part was created."]
    pub created_at: i64,
    #[doc = "The upload Part unique identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always `upload.part`."]
    pub object: UploadPartObject,
    #[doc = "The ID of the Upload object that this Part was added to."]
    pub upload_id: ::std::string::String,
}
#[doc = "The object type, which is always `upload.part`."]
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
pub enum UploadPartObject {
    #[serde(rename = "upload.part")]
    UploadPart,
}
impl ::std::fmt::Display for UploadPartObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UploadPart => f.write_str("upload.part"),
        }
    }
}
impl ::std::str::FromStr for UploadPartObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "upload.part" => Ok(Self::UploadPart),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UploadPartObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UploadPartObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the Upload."]
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
pub enum UploadStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}
impl ::std::fmt::Display for UploadStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
            Self::Completed => f.write_str("completed"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Expired => f.write_str("expired"),
        }
    }
}
impl ::std::str::FromStr for UploadStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UploadStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UploadStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UploadsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UploadsApiSchema {
    #[serde(rename = "AddUploadPartRequest")]
    pub add_upload_part_request: AddUploadPartRequest,
    #[serde(rename = "AddUploadPartResponse")]
    pub add_upload_part_response: UploadPart,
    #[serde(rename = "CancelUploadResponse")]
    pub cancel_upload_response: Upload,
    #[serde(rename = "CompleteUploadRequest")]
    pub complete_upload_request: CompleteUploadRequest,
    #[serde(rename = "CompleteUploadResponse")]
    pub complete_upload_response: Upload,
    #[serde(rename = "CreateUploadRequest")]
    pub create_upload_request: CreateUploadRequest,
    #[serde(rename = "CreateUploadResponse")]
    pub create_upload_response: Upload,
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
