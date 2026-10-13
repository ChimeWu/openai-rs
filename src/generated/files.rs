#[doc = "`CreateFileRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateFileRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<FileExpirationAfter>,
    #[doc = "The File object (not file name) to be uploaded.\n"]
    pub file: ::std::string::String,
    #[doc = "The intended purpose of the uploaded file. One of:\n- `assistants`: Used in the Assistants API\n- `batch`: Used in the Batch API\n- `fine-tune`: Used for fine-tuning\n- `vision`: Images used for vision fine-tuning\n- `user_data`: Flexible file type for any purpose\n- `evals`: Used for eval data sets\n"]
    pub purpose: CreateFileRequestPurpose,
}
impl CreateFileRequest {
    pub fn builder() -> builder::CreateFileRequest {
        Default::default()
    }
}
#[doc = "The intended purpose of the uploaded file. One of:\n- `assistants`: Used in the Assistants API\n- `batch`: Used in the Batch API\n- `fine-tune`: Used for fine-tuning\n- `vision`: Images used for vision fine-tuning\n- `user_data`: Flexible file type for any purpose\n- `evals`: Used for eval data sets\n"]
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
pub enum CreateFileRequestPurpose {
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "vision")]
    Vision,
    #[serde(rename = "user_data")]
    UserData,
    #[serde(rename = "evals")]
    Evals,
}
impl ::std::fmt::Display for CreateFileRequestPurpose {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistants => f.write_str("assistants"),
            Self::Batch => f.write_str("batch"),
            Self::FineTune => f.write_str("fine-tune"),
            Self::Vision => f.write_str("vision"),
            Self::UserData => f.write_str("user_data"),
            Self::Evals => f.write_str("evals"),
        }
    }
}
impl ::std::str::FromStr for CreateFileRequestPurpose {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistants" => Ok(Self::Assistants),
            "batch" => Ok(Self::Batch),
            "fine-tune" => Ok(Self::FineTune),
            "vision" => Ok(Self::Vision),
            "user_data" => Ok(Self::UserData),
            "evals" => Ok(Self::Evals),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateFileRequestPurpose {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateFileRequestPurpose {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeleteFileResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteFileResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeleteFileResponseObject,
}
impl DeleteFileResponse {
    pub fn builder() -> builder::DeleteFileResponse {
        Default::default()
    }
}
#[doc = "`DeleteFileResponseObject`"]
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
pub enum DeleteFileResponseObject {
    #[serde(rename = "file")]
    File,
}
impl ::std::fmt::Display for DeleteFileResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::File => f.write_str("file"),
        }
    }
}
impl ::std::str::FromStr for DeleteFileResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file" => Ok(Self::File),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteFileResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteFileResponseObject {
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
impl FileExpirationAfter {
    pub fn builder() -> builder::FileExpirationAfter {
        Default::default()
    }
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
#[doc = "`FilesApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FilesApiSchema {
    #[serde(rename = "CreateFileRequest")]
    pub create_file_request: CreateFileRequest,
    #[serde(rename = "CreateFileResponse")]
    pub create_file_response: OpenAiFile,
    #[serde(rename = "DeleteFileResponse")]
    pub delete_file_response: DeleteFileResponse,
    #[serde(rename = "ListFilesParams")]
    pub list_files_params: ListFilesParams,
    #[serde(rename = "ListFilesResponse")]
    pub list_files_response: ListFilesResponse,
    #[serde(rename = "RetrieveFileResponse")]
    pub retrieve_file_response: OpenAiFile,
}
impl FilesApiSchema {
    pub fn builder() -> builder::FilesApiSchema {
        Default::default()
    }
}
#[doc = "`ListFilesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListFilesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListFilesParamsOrder>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub purpose: ::std::option::Option<::std::string::String>,
}
impl ListFilesParams {
    pub fn builder() -> builder::ListFilesParams {
        Default::default()
    }
}
#[doc = "`ListFilesParamsOrder`"]
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
pub enum ListFilesParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListFilesParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListFilesParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListFilesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListFilesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListFilesResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListFilesResponse {
    pub data: ::std::vec::Vec<OpenAiFile>,
    pub first_id: ::std::string::String,
    pub has_more: bool,
    pub last_id: ::std::string::String,
    pub object: ::std::string::String,
}
impl ListFilesResponse {
    pub fn builder() -> builder::ListFilesResponse {
        Default::default()
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
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<OpenAiFileStatus>,
    #[doc = "Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`. Completed file upload responses can return null when these details are unset."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status_details: ::std::option::Option<::std::string::String>,
}
impl OpenAiFile {
    pub fn builder() -> builder::OpenAiFile {
        Default::default()
    }
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
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateFileRequest {
        expires_after: ::std::result::Result<
            ::std::option::Option<super::FileExpirationAfter>,
            ::std::string::String,
        >,
        file: ::std::result::Result<::std::string::String, ::std::string::String>,
        purpose: ::std::result::Result<super::CreateFileRequestPurpose, ::std::string::String>,
    }
    impl ::std::default::Default for CreateFileRequest {
        fn default() -> Self {
            Self {
                expires_after: Ok(Default::default()),
                file: Err("no value supplied for file".to_string()),
                purpose: Err("no value supplied for purpose".to_string()),
            }
        }
    }
    impl CreateFileRequest {
        pub fn expires_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FileExpirationAfter>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_after: {e}"));
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
        pub fn purpose<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateFileRequestPurpose>,
            T::Error: ::std::fmt::Display,
        {
            self.purpose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for purpose: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateFileRequest> for super::CreateFileRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateFileRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_after: value.expires_after?,
                file: value.file?,
                purpose: value.purpose?,
            })
        }
    }
    impl ::std::convert::From<super::CreateFileRequest> for CreateFileRequest {
        fn from(value: super::CreateFileRequest) -> Self {
            Self {
                expires_after: Ok(value.expires_after),
                file: Ok(value.file),
                purpose: Ok(value.purpose),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteFileResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::DeleteFileResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteFileResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteFileResponse {
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
            T: ::std::convert::TryInto<super::DeleteFileResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteFileResponse> for super::DeleteFileResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteFileResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteFileResponse> for DeleteFileResponse {
        fn from(value: super::DeleteFileResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileExpirationAfter {
        anchor: ::std::result::Result<super::FileExpirationAfterAnchor, ::std::string::String>,
        seconds: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for FileExpirationAfter {
        fn default() -> Self {
            Self {
                anchor: Err("no value supplied for anchor".to_string()),
                seconds: Err("no value supplied for seconds".to_string()),
            }
        }
    }
    impl FileExpirationAfter {
        pub fn anchor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FileExpirationAfterAnchor>,
            T::Error: ::std::fmt::Display,
        {
            self.anchor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for anchor: {e}"));
            self
        }
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seconds: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FileExpirationAfter> for super::FileExpirationAfter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FileExpirationAfter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                anchor: value.anchor?,
                seconds: value.seconds?,
            })
        }
    }
    impl ::std::convert::From<super::FileExpirationAfter> for FileExpirationAfter {
        fn from(value: super::FileExpirationAfter) -> Self {
            Self {
                anchor: Ok(value.anchor),
                seconds: Ok(value.seconds),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FilesApiSchema {
        create_file_request: ::std::result::Result<super::CreateFileRequest, ::std::string::String>,
        create_file_response: ::std::result::Result<super::OpenAiFile, ::std::string::String>,
        delete_file_response:
            ::std::result::Result<super::DeleteFileResponse, ::std::string::String>,
        list_files_params: ::std::result::Result<super::ListFilesParams, ::std::string::String>,
        list_files_response: ::std::result::Result<super::ListFilesResponse, ::std::string::String>,
        retrieve_file_response: ::std::result::Result<super::OpenAiFile, ::std::string::String>,
    }
    impl ::std::default::Default for FilesApiSchema {
        fn default() -> Self {
            Self {
                create_file_request: Err("no value supplied for create_file_request".to_string()),
                create_file_response: Err("no value supplied for create_file_response".to_string()),
                delete_file_response: Err("no value supplied for delete_file_response".to_string()),
                list_files_params: Err("no value supplied for list_files_params".to_string()),
                list_files_response: Err("no value supplied for list_files_response".to_string()),
                retrieve_file_response: Err(
                    "no value supplied for retrieve_file_response".to_string()
                ),
            }
        }
    }
    impl FilesApiSchema {
        pub fn create_file_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateFileRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_file_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_file_request: {e}")
            });
            self
        }
        pub fn create_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OpenAiFile>,
            T::Error: ::std::fmt::Display,
        {
            self.create_file_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_file_response: {e}")
            });
            self
        }
        pub fn delete_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteFileResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_file_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_file_response: {e}")
            });
            self
        }
        pub fn list_files_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFilesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_files_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_files_params: {e}"));
            self
        }
        pub fn list_files_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFilesResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_files_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_files_response: {e}")
            });
            self
        }
        pub fn retrieve_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OpenAiFile>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_file_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_file_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<FilesApiSchema> for super::FilesApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FilesApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_file_request: value.create_file_request?,
                create_file_response: value.create_file_response?,
                delete_file_response: value.delete_file_response?,
                list_files_params: value.list_files_params?,
                list_files_response: value.list_files_response?,
                retrieve_file_response: value.retrieve_file_response?,
            })
        }
    }
    impl ::std::convert::From<super::FilesApiSchema> for FilesApiSchema {
        fn from(value: super::FilesApiSchema) -> Self {
            Self {
                create_file_request: Ok(value.create_file_request),
                create_file_response: Ok(value.create_file_response),
                delete_file_response: Ok(value.delete_file_response),
                list_files_params: Ok(value.list_files_params),
                list_files_response: Ok(value.list_files_response),
                retrieve_file_response: Ok(value.retrieve_file_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListFilesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListFilesParamsOrder>,
            ::std::string::String,
        >,
        purpose: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListFilesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                purpose: Ok(Default::default()),
            }
        }
    }
    impl ListFilesParams {
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
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListFilesParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn purpose<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.purpose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for purpose: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListFilesParams> for super::ListFilesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListFilesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                purpose: value.purpose?,
            })
        }
    }
    impl ::std::convert::From<super::ListFilesParams> for ListFilesParams {
        fn from(value: super::ListFilesParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                purpose: Ok(value.purpose),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListFilesResponse {
        data: ::std::result::Result<::std::vec::Vec<super::OpenAiFile>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListFilesResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                first_id: Err("no value supplied for first_id".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                last_id: Err("no value supplied for last_id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ListFilesResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OpenAiFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn first_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.last_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_id: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListFilesResponse> for super::ListFilesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListFilesResponse,
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
    impl ::std::convert::From<super::ListFilesResponse> for ListFilesResponse {
        fn from(value: super::ListFilesResponse) -> Self {
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
    pub struct OpenAiFile {
        bytes: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        filename: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::OpenAiFileObject, ::std::string::String>,
        purpose: ::std::result::Result<super::OpenAiFilePurpose, ::std::string::String>,
        status: ::std::result::Result<
            ::std::option::Option<super::OpenAiFileStatus>,
            ::std::string::String,
        >,
        status_details: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OpenAiFile {
        fn default() -> Self {
            Self {
                bytes: Err("no value supplied for bytes".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                expires_at: Ok(Default::default()),
                filename: Err("no value supplied for filename".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                purpose: Err("no value supplied for purpose".to_string()),
                status: Ok(Default::default()),
                status_details: Ok(Default::default()),
            }
        }
    }
    impl OpenAiFile {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {e}"));
            self
        }
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
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn filename<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.filename = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filename: {e}"));
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
            T: ::std::convert::TryInto<super::OpenAiFileObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn purpose<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OpenAiFilePurpose>,
            T::Error: ::std::fmt::Display,
        {
            self.purpose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for purpose: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OpenAiFileStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn status_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_details: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OpenAiFile> for super::OpenAiFile {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OpenAiFile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                created_at: value.created_at?,
                expires_at: value.expires_at?,
                filename: value.filename?,
                id: value.id?,
                object: value.object?,
                purpose: value.purpose?,
                status: value.status?,
                status_details: value.status_details?,
            })
        }
    }
    impl ::std::convert::From<super::OpenAiFile> for OpenAiFile {
        fn from(value: super::OpenAiFile) -> Self {
            Self {
                bytes: Ok(value.bytes),
                created_at: Ok(value.created_at),
                expires_at: Ok(value.expires_at),
                filename: Ok(value.filename),
                id: Ok(value.id),
                object: Ok(value.object),
                purpose: Ok(value.purpose),
                status: Ok(value.status),
                status_details: Ok(value.status_details),
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
