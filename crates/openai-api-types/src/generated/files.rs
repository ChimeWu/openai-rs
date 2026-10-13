pub use crate::common::{Error, ErrorResponse, FileExpirationAfter, MisalignmentErrorDetailsResource, OpenAiFile, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`CreateFileRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateFileRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<::openai_api_types::common::FileExpirationAfter>,
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
#[doc = "`DeleteFileParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteFileParams {
    pub file_id: ::std::string::String,
}
impl DeleteFileParams {
    pub fn builder() -> builder::DeleteFileParams {
        Default::default()
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
#[doc = "`DownloadFileParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DownloadFileParams {
    pub file_id: ::std::string::String,
}
impl DownloadFileParams {
    pub fn builder() -> builder::DownloadFileParams {
        Default::default()
    }
}
#[doc = "`FilesApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FilesApiSchema {
    #[serde(rename = "CreateFileError400")]
    pub create_file_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateFileError429")]
    pub create_file_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateFileError500")]
    pub create_file_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateFileRequest")]
    pub create_file_request: CreateFileRequest,
    #[serde(rename = "CreateFileResponse")]
    pub create_file_response: ::openai_api_types::common::OpenAiFile,
    #[serde(rename = "DeleteFileError404")]
    pub delete_file_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteFileError429")]
    pub delete_file_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteFileParams")]
    pub delete_file_params: DeleteFileParams,
    #[serde(rename = "DeleteFileResponse")]
    pub delete_file_response: DeleteFileResponse,
    #[serde(rename = "DownloadFileError400")]
    pub download_file_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DownloadFileError404")]
    pub download_file_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DownloadFileError429")]
    pub download_file_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DownloadFileError500")]
    pub download_file_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DownloadFileParams")]
    pub download_file_params: DownloadFileParams,
    #[serde(rename = "ListFilesError400")]
    pub list_files_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListFilesError404")]
    pub list_files_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListFilesError429")]
    pub list_files_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListFilesParams")]
    pub list_files_params: ListFilesParams,
    #[serde(rename = "ListFilesResponse")]
    pub list_files_response: ListFilesResponse,
    #[serde(rename = "RetrieveFileError404")]
    pub retrieve_file_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveFileError429")]
    pub retrieve_file_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveFileParams")]
    pub retrieve_file_params: RetrieveFileParams,
    #[serde(rename = "RetrieveFileResponse")]
    pub retrieve_file_response: ::openai_api_types::common::OpenAiFile,
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
    pub data: ::std::vec::Vec<::openai_api_types::common::OpenAiFile>,
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
#[doc = "`RetrieveFileParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveFileParams {
    pub file_id: ::std::string::String,
}
impl RetrieveFileParams {
    pub fn builder() -> builder::RetrieveFileParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateFileRequest {
        expires_after: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::FileExpirationAfter>,
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
    pub struct DeleteFileParams {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteFileParams {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
            }
        }
    }
    impl DeleteFileParams {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteFileParams> for super::DeleteFileParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteFileParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteFileParams> for DeleteFileParams {
        fn from(value: super::DeleteFileParams) -> Self {
            Self {
                file_id: Ok(value.file_id),
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
    pub struct DownloadFileParams {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DownloadFileParams {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
            }
        }
    }
    impl DownloadFileParams {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DownloadFileParams> for super::DownloadFileParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DownloadFileParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::DownloadFileParams> for DownloadFileParams {
        fn from(value: super::DownloadFileParams) -> Self {
            Self {
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FilesApiSchema {
        create_file_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_file_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_file_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_file_request: ::std::result::Result<super::CreateFileRequest, ::std::string::String>,
        create_file_response:
            ::std::result::Result<::openai_api_types::common::OpenAiFile, ::std::string::String>,
        delete_file_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_file_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_file_params: ::std::result::Result<super::DeleteFileParams, ::std::string::String>,
        delete_file_response:
            ::std::result::Result<super::DeleteFileResponse, ::std::string::String>,
        download_file_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        download_file_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        download_file_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        download_file_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        download_file_params:
            ::std::result::Result<super::DownloadFileParams, ::std::string::String>,
        list_files_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_files_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_files_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_files_params: ::std::result::Result<super::ListFilesParams, ::std::string::String>,
        list_files_response: ::std::result::Result<super::ListFilesResponse, ::std::string::String>,
        retrieve_file_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_file_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_file_params:
            ::std::result::Result<super::RetrieveFileParams, ::std::string::String>,
        retrieve_file_response:
            ::std::result::Result<::openai_api_types::common::OpenAiFile, ::std::string::String>,
    }
    impl ::std::default::Default for FilesApiSchema {
        fn default() -> Self {
            Self {
                create_file_error400: Err("no value supplied for create_file_error400".to_string()),
                create_file_error429: Err("no value supplied for create_file_error429".to_string()),
                create_file_error500: Err("no value supplied for create_file_error500".to_string()),
                create_file_request: Err("no value supplied for create_file_request".to_string()),
                create_file_response: Err("no value supplied for create_file_response".to_string()),
                delete_file_error404: Err("no value supplied for delete_file_error404".to_string()),
                delete_file_error429: Err("no value supplied for delete_file_error429".to_string()),
                delete_file_params: Err("no value supplied for delete_file_params".to_string()),
                delete_file_response: Err("no value supplied for delete_file_response".to_string()),
                download_file_error400: Err(
                    "no value supplied for download_file_error400".to_string()
                ),
                download_file_error404: Err(
                    "no value supplied for download_file_error404".to_string()
                ),
                download_file_error429: Err(
                    "no value supplied for download_file_error429".to_string()
                ),
                download_file_error500: Err(
                    "no value supplied for download_file_error500".to_string()
                ),
                download_file_params: Err("no value supplied for download_file_params".to_string()),
                list_files_error400: Err("no value supplied for list_files_error400".to_string()),
                list_files_error404: Err("no value supplied for list_files_error404".to_string()),
                list_files_error429: Err("no value supplied for list_files_error429".to_string()),
                list_files_params: Err("no value supplied for list_files_params".to_string()),
                list_files_response: Err("no value supplied for list_files_response".to_string()),
                retrieve_file_error404: Err(
                    "no value supplied for retrieve_file_error404".to_string()
                ),
                retrieve_file_error429: Err(
                    "no value supplied for retrieve_file_error429".to_string()
                ),
                retrieve_file_params: Err("no value supplied for retrieve_file_params".to_string()),
                retrieve_file_response: Err(
                    "no value supplied for retrieve_file_response".to_string()
                ),
            }
        }
    }
    impl FilesApiSchema {
        pub fn create_file_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_file_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_file_error400: {e}")
            });
            self
        }
        pub fn create_file_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_file_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_file_error429: {e}")
            });
            self
        }
        pub fn create_file_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_file_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_file_error500: {e}")
            });
            self
        }
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
            T: ::std::convert::TryInto<::openai_api_types::common::OpenAiFile>,
            T::Error: ::std::fmt::Display,
        {
            self.create_file_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_file_response: {e}")
            });
            self
        }
        pub fn delete_file_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_file_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_file_error404: {e}")
            });
            self
        }
        pub fn delete_file_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_file_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_file_error429: {e}")
            });
            self
        }
        pub fn delete_file_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteFileParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_file_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_file_params: {e}")
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
        pub fn download_file_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.download_file_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_file_error400: {e}")
            });
            self
        }
        pub fn download_file_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.download_file_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_file_error404: {e}")
            });
            self
        }
        pub fn download_file_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.download_file_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_file_error429: {e}")
            });
            self
        }
        pub fn download_file_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.download_file_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_file_error500: {e}")
            });
            self
        }
        pub fn download_file_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DownloadFileParams>,
            T::Error: ::std::fmt::Display,
        {
            self.download_file_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_file_params: {e}")
            });
            self
        }
        pub fn list_files_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_files_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_files_error400: {e}")
            });
            self
        }
        pub fn list_files_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_files_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_files_error404: {e}")
            });
            self
        }
        pub fn list_files_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_files_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_files_error429: {e}")
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
        pub fn retrieve_file_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_file_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_file_error404: {e}")
            });
            self
        }
        pub fn retrieve_file_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_file_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_file_error429: {e}")
            });
            self
        }
        pub fn retrieve_file_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveFileParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_file_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_file_params: {e}")
            });
            self
        }
        pub fn retrieve_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::OpenAiFile>,
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
                create_file_error400: value.create_file_error400?,
                create_file_error429: value.create_file_error429?,
                create_file_error500: value.create_file_error500?,
                create_file_request: value.create_file_request?,
                create_file_response: value.create_file_response?,
                delete_file_error404: value.delete_file_error404?,
                delete_file_error429: value.delete_file_error429?,
                delete_file_params: value.delete_file_params?,
                delete_file_response: value.delete_file_response?,
                download_file_error400: value.download_file_error400?,
                download_file_error404: value.download_file_error404?,
                download_file_error429: value.download_file_error429?,
                download_file_error500: value.download_file_error500?,
                download_file_params: value.download_file_params?,
                list_files_error400: value.list_files_error400?,
                list_files_error404: value.list_files_error404?,
                list_files_error429: value.list_files_error429?,
                list_files_params: value.list_files_params?,
                list_files_response: value.list_files_response?,
                retrieve_file_error404: value.retrieve_file_error404?,
                retrieve_file_error429: value.retrieve_file_error429?,
                retrieve_file_params: value.retrieve_file_params?,
                retrieve_file_response: value.retrieve_file_response?,
            })
        }
    }
    impl ::std::convert::From<super::FilesApiSchema> for FilesApiSchema {
        fn from(value: super::FilesApiSchema) -> Self {
            Self {
                create_file_error400: Ok(value.create_file_error400),
                create_file_error429: Ok(value.create_file_error429),
                create_file_error500: Ok(value.create_file_error500),
                create_file_request: Ok(value.create_file_request),
                create_file_response: Ok(value.create_file_response),
                delete_file_error404: Ok(value.delete_file_error404),
                delete_file_error429: Ok(value.delete_file_error429),
                delete_file_params: Ok(value.delete_file_params),
                delete_file_response: Ok(value.delete_file_response),
                download_file_error400: Ok(value.download_file_error400),
                download_file_error404: Ok(value.download_file_error404),
                download_file_error429: Ok(value.download_file_error429),
                download_file_error500: Ok(value.download_file_error500),
                download_file_params: Ok(value.download_file_params),
                list_files_error400: Ok(value.list_files_error400),
                list_files_error404: Ok(value.list_files_error404),
                list_files_error429: Ok(value.list_files_error429),
                list_files_params: Ok(value.list_files_params),
                list_files_response: Ok(value.list_files_response),
                retrieve_file_error404: Ok(value.retrieve_file_error404),
                retrieve_file_error429: Ok(value.retrieve_file_error429),
                retrieve_file_params: Ok(value.retrieve_file_params),
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
        data: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::OpenAiFile>,
            ::std::string::String,
        >,
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
            T: ::std::convert::TryInto<::std::vec::Vec<::openai_api_types::common::OpenAiFile>>,
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
    pub struct RetrieveFileParams {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveFileParams {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
            }
        }
    }
    impl RetrieveFileParams {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveFileParams> for super::RetrieveFileParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveFileParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveFileParams> for RetrieveFileParams {
        fn from(value: super::RetrieveFileParams) -> Self {
            Self {
                file_id: Ok(value.file_id),
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
