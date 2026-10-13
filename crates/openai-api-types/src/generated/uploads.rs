pub use crate::common::{Error, ErrorResponse, FileExpirationAfter, MisalignmentErrorDetailsResource, OpenAiFile, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`AddUploadPartParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AddUploadPartParams {
    pub upload_id: ::std::string::String,
}
impl AddUploadPartParams {
    pub fn builder() -> builder::AddUploadPartParams {
        Default::default()
    }
}
#[doc = "`AddUploadPartRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AddUploadPartRequest {
    #[doc = "The chunk of bytes for this Part.\n"]
    pub data: ::std::string::String,
}
impl AddUploadPartRequest {
    pub fn builder() -> builder::AddUploadPartRequest {
        Default::default()
    }
}
#[doc = "`CancelUploadParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CancelUploadParams {
    pub upload_id: ::std::string::String,
}
impl CancelUploadParams {
    pub fn builder() -> builder::CancelUploadParams {
        Default::default()
    }
}
#[doc = "`CompleteUploadParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CompleteUploadParams {
    pub upload_id: ::std::string::String,
}
impl CompleteUploadParams {
    pub fn builder() -> builder::CompleteUploadParams {
        Default::default()
    }
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
impl CompleteUploadRequest {
    pub fn builder() -> builder::CompleteUploadRequest {
        Default::default()
    }
}
#[doc = "`CreateUploadRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateUploadRequest {
    #[doc = "The number of bytes in the file you are uploading.\n"]
    pub bytes: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<::openai_api_types::common::FileExpirationAfter>,
    #[doc = "The name of the file to upload.\n"]
    pub filename: ::std::string::String,
    #[doc = "The MIME type of the file.\n\n\nThis must fall within the supported MIME types for your file purpose. See\nthe supported MIME types for assistants and vision.\n"]
    pub mime_type: ::std::string::String,
    #[doc = "The intended purpose of the uploaded file.\n\nSee the [documentation on File\npurposes](https://developers.openai.com/api/reference/resources/files/methods/create#%28resource%29%20files%20%3E%20%28method%29%20create%20%3E%20%28params%29%200%20%3E%20%28param%29%20purpose%20%3E%20%28schema%29).\n"]
    pub purpose: CreateUploadRequestPurpose,
}
impl CreateUploadRequest {
    pub fn builder() -> builder::CreateUploadRequest {
        Default::default()
    }
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
    pub file: ::std::option::Option<::openai_api_types::common::OpenAiFile>,
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
impl Upload {
    pub fn builder() -> builder::Upload {
        Default::default()
    }
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
impl UploadPart {
    pub fn builder() -> builder::UploadPart {
        Default::default()
    }
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
    #[serde(rename = "AddUploadPartError400")]
    pub add_upload_part_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddUploadPartError404")]
    pub add_upload_part_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddUploadPartError410")]
    pub add_upload_part_error410: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddUploadPartError429")]
    pub add_upload_part_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddUploadPartError500")]
    pub add_upload_part_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddUploadPartParams")]
    pub add_upload_part_params: AddUploadPartParams,
    #[serde(rename = "AddUploadPartRequest")]
    pub add_upload_part_request: AddUploadPartRequest,
    #[serde(rename = "AddUploadPartResponse")]
    pub add_upload_part_response: UploadPart,
    #[serde(rename = "CancelUploadError400")]
    pub cancel_upload_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelUploadError404")]
    pub cancel_upload_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelUploadError410")]
    pub cancel_upload_error410: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelUploadError429")]
    pub cancel_upload_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelUploadParams")]
    pub cancel_upload_params: CancelUploadParams,
    #[serde(rename = "CancelUploadResponse")]
    pub cancel_upload_response: Upload,
    #[serde(rename = "CompleteUploadError400")]
    pub complete_upload_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CompleteUploadError404")]
    pub complete_upload_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CompleteUploadError410")]
    pub complete_upload_error410: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CompleteUploadError429")]
    pub complete_upload_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CompleteUploadError500")]
    pub complete_upload_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CompleteUploadParams")]
    pub complete_upload_params: CompleteUploadParams,
    #[serde(rename = "CompleteUploadRequest")]
    pub complete_upload_request: CompleteUploadRequest,
    #[serde(rename = "CompleteUploadResponse")]
    pub complete_upload_response: Upload,
    #[serde(rename = "CreateUploadError400")]
    pub create_upload_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateUploadError429")]
    pub create_upload_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateUploadRequest")]
    pub create_upload_request: CreateUploadRequest,
    #[serde(rename = "CreateUploadResponse")]
    pub create_upload_response: Upload,
}
impl UploadsApiSchema {
    pub fn builder() -> builder::UploadsApiSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AddUploadPartParams {
        upload_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AddUploadPartParams {
        fn default() -> Self {
            Self {
                upload_id: Err("no value supplied for upload_id".to_string()),
            }
        }
    }
    impl AddUploadPartParams {
        pub fn upload_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.upload_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for upload_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AddUploadPartParams> for super::AddUploadPartParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AddUploadPartParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                upload_id: value.upload_id?,
            })
        }
    }
    impl ::std::convert::From<super::AddUploadPartParams> for AddUploadPartParams {
        fn from(value: super::AddUploadPartParams) -> Self {
            Self {
                upload_id: Ok(value.upload_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AddUploadPartRequest {
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AddUploadPartRequest {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
            }
        }
    }
    impl AddUploadPartRequest {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AddUploadPartRequest> for super::AddUploadPartRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AddUploadPartRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { data: value.data? })
        }
    }
    impl ::std::convert::From<super::AddUploadPartRequest> for AddUploadPartRequest {
        fn from(value: super::AddUploadPartRequest) -> Self {
            Self {
                data: Ok(value.data),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelUploadParams {
        upload_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CancelUploadParams {
        fn default() -> Self {
            Self {
                upload_id: Err("no value supplied for upload_id".to_string()),
            }
        }
    }
    impl CancelUploadParams {
        pub fn upload_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.upload_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for upload_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelUploadParams> for super::CancelUploadParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelUploadParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                upload_id: value.upload_id?,
            })
        }
    }
    impl ::std::convert::From<super::CancelUploadParams> for CancelUploadParams {
        fn from(value: super::CancelUploadParams) -> Self {
            Self {
                upload_id: Ok(value.upload_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteUploadParams {
        upload_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteUploadParams {
        fn default() -> Self {
            Self {
                upload_id: Err("no value supplied for upload_id".to_string()),
            }
        }
    }
    impl CompleteUploadParams {
        pub fn upload_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.upload_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for upload_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteUploadParams> for super::CompleteUploadParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteUploadParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                upload_id: value.upload_id?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteUploadParams> for CompleteUploadParams {
        fn from(value: super::CompleteUploadParams) -> Self {
            Self {
                upload_id: Ok(value.upload_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteUploadRequest {
        md5: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        part_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteUploadRequest {
        fn default() -> Self {
            Self {
                md5: Ok(Default::default()),
                part_ids: Err("no value supplied for part_ids".to_string()),
            }
        }
    }
    impl CompleteUploadRequest {
        pub fn md5<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.md5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for md5: {e}"));
            self
        }
        pub fn part_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.part_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for part_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteUploadRequest> for super::CompleteUploadRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteUploadRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                md5: value.md5?,
                part_ids: value.part_ids?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteUploadRequest> for CompleteUploadRequest {
        fn from(value: super::CompleteUploadRequest) -> Self {
            Self {
                md5: Ok(value.md5),
                part_ids: Ok(value.part_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateUploadRequest {
        bytes: ::std::result::Result<i64, ::std::string::String>,
        expires_after: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::FileExpirationAfter>,
            ::std::string::String,
        >,
        filename: ::std::result::Result<::std::string::String, ::std::string::String>,
        mime_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        purpose: ::std::result::Result<super::CreateUploadRequestPurpose, ::std::string::String>,
    }
    impl ::std::default::Default for CreateUploadRequest {
        fn default() -> Self {
            Self {
                bytes: Err("no value supplied for bytes".to_string()),
                expires_after: Ok(Default::default()),
                filename: Err("no value supplied for filename".to_string()),
                mime_type: Err("no value supplied for mime_type".to_string()),
                purpose: Err("no value supplied for purpose".to_string()),
            }
        }
    }
    impl CreateUploadRequest {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {e}"));
            self
        }
        pub fn expires_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::FileExpirationAfter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.expires_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_after: {e}"));
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
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn purpose<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateUploadRequestPurpose>,
            T::Error: ::std::fmt::Display,
        {
            self.purpose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for purpose: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateUploadRequest> for super::CreateUploadRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateUploadRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                expires_after: value.expires_after?,
                filename: value.filename?,
                mime_type: value.mime_type?,
                purpose: value.purpose?,
            })
        }
    }
    impl ::std::convert::From<super::CreateUploadRequest> for CreateUploadRequest {
        fn from(value: super::CreateUploadRequest) -> Self {
            Self {
                bytes: Ok(value.bytes),
                expires_after: Ok(value.expires_after),
                filename: Ok(value.filename),
                mime_type: Ok(value.mime_type),
                purpose: Ok(value.purpose),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Upload {
        bytes: ::std::result::Result<i64, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_at: ::std::result::Result<i64, ::std::string::String>,
        file: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::OpenAiFile>,
            ::std::string::String,
        >,
        filename: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            ::std::option::Option<super::UploadObject>,
            ::std::string::String,
        >,
        purpose: ::std::result::Result<::std::string::String, ::std::string::String>,
        status: ::std::result::Result<super::UploadStatus, ::std::string::String>,
    }
    impl ::std::default::Default for Upload {
        fn default() -> Self {
            Self {
                bytes: Err("no value supplied for bytes".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                expires_at: Err("no value supplied for expires_at".to_string()),
                file: Ok(Default::default()),
                filename: Err("no value supplied for filename".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Ok(Default::default()),
                purpose: Err("no value supplied for purpose".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl Upload {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
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
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::OpenAiFile>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<super::UploadObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn purpose<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.purpose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for purpose: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UploadStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Upload> for super::Upload {
        type Error = super::error::ConversionError;
        fn try_from(value: Upload) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                created_at: value.created_at?,
                expires_at: value.expires_at?,
                file: value.file?,
                filename: value.filename?,
                id: value.id?,
                object: value.object?,
                purpose: value.purpose?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::Upload> for Upload {
        fn from(value: super::Upload) -> Self {
            Self {
                bytes: Ok(value.bytes),
                created_at: Ok(value.created_at),
                expires_at: Ok(value.expires_at),
                file: Ok(value.file),
                filename: Ok(value.filename),
                id: Ok(value.id),
                object: Ok(value.object),
                purpose: Ok(value.purpose),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UploadPart {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::UploadPartObject, ::std::string::String>,
        upload_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UploadPart {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                upload_id: Err("no value supplied for upload_id".to_string()),
            }
        }
    }
    impl UploadPart {
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UploadPartObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn upload_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.upload_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for upload_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UploadPart> for super::UploadPart {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UploadPart,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                object: value.object?,
                upload_id: value.upload_id?,
            })
        }
    }
    impl ::std::convert::From<super::UploadPart> for UploadPart {
        fn from(value: super::UploadPart) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                object: Ok(value.object),
                upload_id: Ok(value.upload_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UploadsApiSchema {
        add_upload_part_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_upload_part_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_upload_part_error410:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_upload_part_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_upload_part_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_upload_part_params:
            ::std::result::Result<super::AddUploadPartParams, ::std::string::String>,
        add_upload_part_request:
            ::std::result::Result<super::AddUploadPartRequest, ::std::string::String>,
        add_upload_part_response: ::std::result::Result<super::UploadPart, ::std::string::String>,
        cancel_upload_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_upload_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_upload_error410:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_upload_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_upload_params:
            ::std::result::Result<super::CancelUploadParams, ::std::string::String>,
        cancel_upload_response: ::std::result::Result<super::Upload, ::std::string::String>,
        complete_upload_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        complete_upload_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        complete_upload_error410:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        complete_upload_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        complete_upload_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        complete_upload_params:
            ::std::result::Result<super::CompleteUploadParams, ::std::string::String>,
        complete_upload_request:
            ::std::result::Result<super::CompleteUploadRequest, ::std::string::String>,
        complete_upload_response: ::std::result::Result<super::Upload, ::std::string::String>,
        create_upload_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_upload_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_upload_request:
            ::std::result::Result<super::CreateUploadRequest, ::std::string::String>,
        create_upload_response: ::std::result::Result<super::Upload, ::std::string::String>,
    }
    impl ::std::default::Default for UploadsApiSchema {
        fn default() -> Self {
            Self {
                add_upload_part_error400: Err(
                    "no value supplied for add_upload_part_error400".to_string()
                ),
                add_upload_part_error404: Err(
                    "no value supplied for add_upload_part_error404".to_string()
                ),
                add_upload_part_error410: Err(
                    "no value supplied for add_upload_part_error410".to_string()
                ),
                add_upload_part_error429: Err(
                    "no value supplied for add_upload_part_error429".to_string()
                ),
                add_upload_part_error500: Err(
                    "no value supplied for add_upload_part_error500".to_string()
                ),
                add_upload_part_params: Err(
                    "no value supplied for add_upload_part_params".to_string()
                ),
                add_upload_part_request: Err(
                    "no value supplied for add_upload_part_request".to_string()
                ),
                add_upload_part_response: Err(
                    "no value supplied for add_upload_part_response".to_string()
                ),
                cancel_upload_error400: Err(
                    "no value supplied for cancel_upload_error400".to_string()
                ),
                cancel_upload_error404: Err(
                    "no value supplied for cancel_upload_error404".to_string()
                ),
                cancel_upload_error410: Err(
                    "no value supplied for cancel_upload_error410".to_string()
                ),
                cancel_upload_error429: Err(
                    "no value supplied for cancel_upload_error429".to_string()
                ),
                cancel_upload_params: Err("no value supplied for cancel_upload_params".to_string()),
                cancel_upload_response: Err(
                    "no value supplied for cancel_upload_response".to_string()
                ),
                complete_upload_error400: Err(
                    "no value supplied for complete_upload_error400".to_string()
                ),
                complete_upload_error404: Err(
                    "no value supplied for complete_upload_error404".to_string()
                ),
                complete_upload_error410: Err(
                    "no value supplied for complete_upload_error410".to_string()
                ),
                complete_upload_error429: Err(
                    "no value supplied for complete_upload_error429".to_string()
                ),
                complete_upload_error500: Err(
                    "no value supplied for complete_upload_error500".to_string()
                ),
                complete_upload_params: Err(
                    "no value supplied for complete_upload_params".to_string()
                ),
                complete_upload_request: Err(
                    "no value supplied for complete_upload_request".to_string()
                ),
                complete_upload_response: Err(
                    "no value supplied for complete_upload_response".to_string()
                ),
                create_upload_error400: Err(
                    "no value supplied for create_upload_error400".to_string()
                ),
                create_upload_error429: Err(
                    "no value supplied for create_upload_error429".to_string()
                ),
                create_upload_request: Err(
                    "no value supplied for create_upload_request".to_string()
                ),
                create_upload_response: Err(
                    "no value supplied for create_upload_response".to_string()
                ),
            }
        }
    }
    impl UploadsApiSchema {
        pub fn add_upload_part_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_upload_part_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_upload_part_error400: {e}")
            });
            self
        }
        pub fn add_upload_part_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_upload_part_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_upload_part_error404: {e}")
            });
            self
        }
        pub fn add_upload_part_error410<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_upload_part_error410 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_upload_part_error410: {e}")
            });
            self
        }
        pub fn add_upload_part_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_upload_part_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_upload_part_error429: {e}")
            });
            self
        }
        pub fn add_upload_part_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_upload_part_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_upload_part_error500: {e}")
            });
            self
        }
        pub fn add_upload_part_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddUploadPartParams>,
            T::Error: ::std::fmt::Display,
        {
            self.add_upload_part_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_upload_part_params: {e}")
            });
            self
        }
        pub fn add_upload_part_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddUploadPartRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.add_upload_part_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_upload_part_request: {e}")
            });
            self
        }
        pub fn add_upload_part_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UploadPart>,
            T::Error: ::std::fmt::Display,
        {
            self.add_upload_part_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_upload_part_response: {e}")
            });
            self
        }
        pub fn cancel_upload_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_upload_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_upload_error400: {e}")
            });
            self
        }
        pub fn cancel_upload_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_upload_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_upload_error404: {e}")
            });
            self
        }
        pub fn cancel_upload_error410<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_upload_error410 = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_upload_error410: {e}")
            });
            self
        }
        pub fn cancel_upload_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_upload_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_upload_error429: {e}")
            });
            self
        }
        pub fn cancel_upload_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelUploadParams>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_upload_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_upload_params: {e}")
            });
            self
        }
        pub fn cancel_upload_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Upload>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_upload_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_upload_response: {e}")
            });
            self
        }
        pub fn complete_upload_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.complete_upload_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for complete_upload_error400: {e}")
            });
            self
        }
        pub fn complete_upload_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.complete_upload_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for complete_upload_error404: {e}")
            });
            self
        }
        pub fn complete_upload_error410<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.complete_upload_error410 = value.try_into().map_err(|e| {
                format!("error converting supplied value for complete_upload_error410: {e}")
            });
            self
        }
        pub fn complete_upload_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.complete_upload_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for complete_upload_error429: {e}")
            });
            self
        }
        pub fn complete_upload_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.complete_upload_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for complete_upload_error500: {e}")
            });
            self
        }
        pub fn complete_upload_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteUploadParams>,
            T::Error: ::std::fmt::Display,
        {
            self.complete_upload_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for complete_upload_params: {e}")
            });
            self
        }
        pub fn complete_upload_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteUploadRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.complete_upload_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for complete_upload_request: {e}")
            });
            self
        }
        pub fn complete_upload_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Upload>,
            T::Error: ::std::fmt::Display,
        {
            self.complete_upload_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for complete_upload_response: {e}")
            });
            self
        }
        pub fn create_upload_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_upload_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_upload_error400: {e}")
            });
            self
        }
        pub fn create_upload_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_upload_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_upload_error429: {e}")
            });
            self
        }
        pub fn create_upload_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateUploadRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_upload_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_upload_request: {e}")
            });
            self
        }
        pub fn create_upload_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Upload>,
            T::Error: ::std::fmt::Display,
        {
            self.create_upload_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_upload_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<UploadsApiSchema> for super::UploadsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UploadsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_upload_part_error400: value.add_upload_part_error400?,
                add_upload_part_error404: value.add_upload_part_error404?,
                add_upload_part_error410: value.add_upload_part_error410?,
                add_upload_part_error429: value.add_upload_part_error429?,
                add_upload_part_error500: value.add_upload_part_error500?,
                add_upload_part_params: value.add_upload_part_params?,
                add_upload_part_request: value.add_upload_part_request?,
                add_upload_part_response: value.add_upload_part_response?,
                cancel_upload_error400: value.cancel_upload_error400?,
                cancel_upload_error404: value.cancel_upload_error404?,
                cancel_upload_error410: value.cancel_upload_error410?,
                cancel_upload_error429: value.cancel_upload_error429?,
                cancel_upload_params: value.cancel_upload_params?,
                cancel_upload_response: value.cancel_upload_response?,
                complete_upload_error400: value.complete_upload_error400?,
                complete_upload_error404: value.complete_upload_error404?,
                complete_upload_error410: value.complete_upload_error410?,
                complete_upload_error429: value.complete_upload_error429?,
                complete_upload_error500: value.complete_upload_error500?,
                complete_upload_params: value.complete_upload_params?,
                complete_upload_request: value.complete_upload_request?,
                complete_upload_response: value.complete_upload_response?,
                create_upload_error400: value.create_upload_error400?,
                create_upload_error429: value.create_upload_error429?,
                create_upload_request: value.create_upload_request?,
                create_upload_response: value.create_upload_response?,
            })
        }
    }
    impl ::std::convert::From<super::UploadsApiSchema> for UploadsApiSchema {
        fn from(value: super::UploadsApiSchema) -> Self {
            Self {
                add_upload_part_error400: Ok(value.add_upload_part_error400),
                add_upload_part_error404: Ok(value.add_upload_part_error404),
                add_upload_part_error410: Ok(value.add_upload_part_error410),
                add_upload_part_error429: Ok(value.add_upload_part_error429),
                add_upload_part_error500: Ok(value.add_upload_part_error500),
                add_upload_part_params: Ok(value.add_upload_part_params),
                add_upload_part_request: Ok(value.add_upload_part_request),
                add_upload_part_response: Ok(value.add_upload_part_response),
                cancel_upload_error400: Ok(value.cancel_upload_error400),
                cancel_upload_error404: Ok(value.cancel_upload_error404),
                cancel_upload_error410: Ok(value.cancel_upload_error410),
                cancel_upload_error429: Ok(value.cancel_upload_error429),
                cancel_upload_params: Ok(value.cancel_upload_params),
                cancel_upload_response: Ok(value.cancel_upload_response),
                complete_upload_error400: Ok(value.complete_upload_error400),
                complete_upload_error404: Ok(value.complete_upload_error404),
                complete_upload_error410: Ok(value.complete_upload_error410),
                complete_upload_error429: Ok(value.complete_upload_error429),
                complete_upload_error500: Ok(value.complete_upload_error500),
                complete_upload_params: Ok(value.complete_upload_params),
                complete_upload_request: Ok(value.complete_upload_request),
                complete_upload_response: Ok(value.complete_upload_response),
                create_upload_error400: Ok(value.create_upload_error400),
                create_upload_error429: Ok(value.create_upload_error429),
                create_upload_request: Ok(value.create_upload_request),
                create_upload_response: Ok(value.create_upload_response),
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
