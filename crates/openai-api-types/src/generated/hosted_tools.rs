pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "Permission state for a single hosted tool on a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct HostedToolPermission {
    #[doc = "Whether the hosted tool is enabled for the project."]
    pub enabled: bool,
}
impl HostedToolPermission {
    pub fn builder() -> builder::HostedToolPermission {
        Default::default()
    }
}
#[doc = "`HostedToolPermissionUpdate`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct HostedToolPermissionUpdate {
    #[doc = "Whether to enable the hosted tool for the project."]
    pub enabled: bool,
}
impl HostedToolPermissionUpdate {
    pub fn builder() -> builder::HostedToolPermissionUpdate {
        Default::default()
    }
}
#[doc = "`HostedToolsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct HostedToolsApiSchema {
    #[serde(rename = "RetrieveProjectHostedToolPermissionsError400")]
    pub retrieve_project_hosted_tool_permissions_error400:
        ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectHostedToolPermissionsError403")]
    pub retrieve_project_hosted_tool_permissions_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectHostedToolPermissionsError404")]
    pub retrieve_project_hosted_tool_permissions_error404:
        ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectHostedToolPermissionsError429")]
    pub retrieve_project_hosted_tool_permissions_error429:
        ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectHostedToolPermissionsParams")]
    pub retrieve_project_hosted_tool_permissions_params: RetrieveProjectHostedToolPermissionsParams,
    #[serde(rename = "RetrieveProjectHostedToolPermissionsResponse")]
    pub retrieve_project_hosted_tool_permissions_response: ProjectHostedToolPermissions,
    #[serde(rename = "UpdateProjectHostedToolPermissionsError400")]
    pub update_project_hosted_tool_permissions_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectHostedToolPermissionsError403")]
    pub update_project_hosted_tool_permissions_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateProjectHostedToolPermissionsError404")]
    pub update_project_hosted_tool_permissions_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectHostedToolPermissionsError409")]
    pub update_project_hosted_tool_permissions_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectHostedToolPermissionsError429")]
    pub update_project_hosted_tool_permissions_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectHostedToolPermissionsParams")]
    pub update_project_hosted_tool_permissions_params: UpdateProjectHostedToolPermissionsParams,
    #[serde(rename = "UpdateProjectHostedToolPermissionsRequest")]
    pub update_project_hosted_tool_permissions_request: ProjectHostedToolPermissionsUpdateRequest,
    #[serde(rename = "UpdateProjectHostedToolPermissionsResponse")]
    pub update_project_hosted_tool_permissions_response: ProjectHostedToolPermissions,
}
impl HostedToolsApiSchema {
    pub fn builder() -> builder::HostedToolsApiSchema {
        Default::default()
    }
}
#[doc = "Represents hosted tool permissions for a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectHostedToolPermissions {
    pub code_interpreter: HostedToolPermission,
    pub file_search: HostedToolPermission,
    pub image_generation: HostedToolPermission,
    pub mcp: HostedToolPermission,
    pub web_search: HostedToolPermission,
}
impl ProjectHostedToolPermissions {
    pub fn builder() -> builder::ProjectHostedToolPermissions {
        Default::default()
    }
}
#[doc = "`ProjectHostedToolPermissionsUpdateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ProjectHostedToolPermissionsUpdateRequest {
    #[doc = "The code interpreter permission update."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code_interpreter: ::std::option::Option<HostedToolPermissionUpdate>,
    #[doc = "The file search permission update."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_search: ::std::option::Option<HostedToolPermissionUpdate>,
    #[doc = "The image generation permission update."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_generation: ::std::option::Option<HostedToolPermissionUpdate>,
    #[doc = "The MCP permission update."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mcp: ::std::option::Option<HostedToolPermissionUpdate>,
    #[doc = "The web search permission update."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub web_search: ::std::option::Option<HostedToolPermissionUpdate>,
}
impl ProjectHostedToolPermissionsUpdateRequest {
    pub fn builder() -> builder::ProjectHostedToolPermissionsUpdateRequest {
        Default::default()
    }
}
#[doc = "`RetrieveProjectHostedToolPermissionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectHostedToolPermissionsParams {
    pub project_id: ::std::string::String,
}
impl RetrieveProjectHostedToolPermissionsParams {
    pub fn builder() -> builder::RetrieveProjectHostedToolPermissionsParams {
        Default::default()
    }
}
#[doc = "`UpdateProjectHostedToolPermissionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectHostedToolPermissionsParams {
    pub project_id: ::std::string::String,
}
impl UpdateProjectHostedToolPermissionsParams {
    pub fn builder() -> builder::UpdateProjectHostedToolPermissionsParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct HostedToolPermission {
        enabled: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for HostedToolPermission {
        fn default() -> Self {
            Self {
                enabled: Err("no value supplied for enabled".to_string()),
            }
        }
    }
    impl HostedToolPermission {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<HostedToolPermission> for super::HostedToolPermission {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HostedToolPermission,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
            })
        }
    }
    impl ::std::convert::From<super::HostedToolPermission> for HostedToolPermission {
        fn from(value: super::HostedToolPermission) -> Self {
            Self {
                enabled: Ok(value.enabled),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HostedToolPermissionUpdate {
        enabled: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for HostedToolPermissionUpdate {
        fn default() -> Self {
            Self {
                enabled: Err("no value supplied for enabled".to_string()),
            }
        }
    }
    impl HostedToolPermissionUpdate {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<HostedToolPermissionUpdate> for super::HostedToolPermissionUpdate {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HostedToolPermissionUpdate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
            })
        }
    }
    impl ::std::convert::From<super::HostedToolPermissionUpdate> for HostedToolPermissionUpdate {
        fn from(value: super::HostedToolPermissionUpdate) -> Self {
            Self {
                enabled: Ok(value.enabled),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HostedToolsApiSchema {
        retrieve_project_hosted_tool_permissions_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_hosted_tool_permissions_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_hosted_tool_permissions_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_hosted_tool_permissions_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_hosted_tool_permissions_params: ::std::result::Result<
            super::RetrieveProjectHostedToolPermissionsParams,
            ::std::string::String,
        >,
        retrieve_project_hosted_tool_permissions_response:
            ::std::result::Result<super::ProjectHostedToolPermissions, ::std::string::String>,
        update_project_hosted_tool_permissions_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_hosted_tool_permissions_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_project_hosted_tool_permissions_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_hosted_tool_permissions_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_hosted_tool_permissions_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_hosted_tool_permissions_params: ::std::result::Result<
            super::UpdateProjectHostedToolPermissionsParams,
            ::std::string::String,
        >,
        update_project_hosted_tool_permissions_request: ::std::result::Result<
            super::ProjectHostedToolPermissionsUpdateRequest,
            ::std::string::String,
        >,
        update_project_hosted_tool_permissions_response:
            ::std::result::Result<super::ProjectHostedToolPermissions, ::std::string::String>,
    }
    impl ::std::default::Default for HostedToolsApiSchema {
        fn default() -> Self {
            Self {
                retrieve_project_hosted_tool_permissions_error400: Err(
                    "no value supplied for retrieve_project_hosted_tool_permissions_error400"
                        .to_string(),
                ),
                retrieve_project_hosted_tool_permissions_error403: Err(
                    "no value supplied for retrieve_project_hosted_tool_permissions_error403"
                        .to_string(),
                ),
                retrieve_project_hosted_tool_permissions_error404: Err(
                    "no value supplied for retrieve_project_hosted_tool_permissions_error404"
                        .to_string(),
                ),
                retrieve_project_hosted_tool_permissions_error429: Err(
                    "no value supplied for retrieve_project_hosted_tool_permissions_error429"
                        .to_string(),
                ),
                retrieve_project_hosted_tool_permissions_params: Err(
                    "no value supplied for retrieve_project_hosted_tool_permissions_params"
                        .to_string(),
                ),
                retrieve_project_hosted_tool_permissions_response: Err(
                    "no value supplied for retrieve_project_hosted_tool_permissions_response"
                        .to_string(),
                ),
                update_project_hosted_tool_permissions_error400: Err(
                    "no value supplied for update_project_hosted_tool_permissions_error400"
                        .to_string(),
                ),
                update_project_hosted_tool_permissions_error403: Err(
                    "no value supplied for update_project_hosted_tool_permissions_error403"
                        .to_string(),
                ),
                update_project_hosted_tool_permissions_error404: Err(
                    "no value supplied for update_project_hosted_tool_permissions_error404"
                        .to_string(),
                ),
                update_project_hosted_tool_permissions_error409: Err(
                    "no value supplied for update_project_hosted_tool_permissions_error409"
                        .to_string(),
                ),
                update_project_hosted_tool_permissions_error429: Err(
                    "no value supplied for update_project_hosted_tool_permissions_error429"
                        .to_string(),
                ),
                update_project_hosted_tool_permissions_params: Err(
                    "no value supplied for update_project_hosted_tool_permissions_params"
                        .to_string(),
                ),
                update_project_hosted_tool_permissions_request: Err(
                    "no value supplied for update_project_hosted_tool_permissions_request"
                        .to_string(),
                ),
                update_project_hosted_tool_permissions_response: Err(
                    "no value supplied for update_project_hosted_tool_permissions_response"
                        .to_string(),
                ),
            }
        }
    }
    impl HostedToolsApiSchema {
        pub fn retrieve_project_hosted_tool_permissions_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_hosted_tool_permissions_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_hosted_tool_permissions_error400: {e}")) ;
            self
        }
        pub fn retrieve_project_hosted_tool_permissions_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_hosted_tool_permissions_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_hosted_tool_permissions_error403: {e}")) ;
            self
        }
        pub fn retrieve_project_hosted_tool_permissions_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_hosted_tool_permissions_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_hosted_tool_permissions_error404: {e}")) ;
            self
        }
        pub fn retrieve_project_hosted_tool_permissions_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_hosted_tool_permissions_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_hosted_tool_permissions_error429: {e}")) ;
            self
        }
        pub fn retrieve_project_hosted_tool_permissions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectHostedToolPermissionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_hosted_tool_permissions_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_hosted_tool_permissions_params: {e}")) ;
            self
        }
        pub fn retrieve_project_hosted_tool_permissions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectHostedToolPermissions>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_hosted_tool_permissions_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_hosted_tool_permissions_response: {e}")) ;
            self
        }
        pub fn update_project_hosted_tool_permissions_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_hosted_tool_permissions_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_hosted_tool_permissions_error400: {e}")) ;
            self
        }
        pub fn update_project_hosted_tool_permissions_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_hosted_tool_permissions_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_hosted_tool_permissions_error403: {e}")) ;
            self
        }
        pub fn update_project_hosted_tool_permissions_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_hosted_tool_permissions_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_hosted_tool_permissions_error404: {e}")) ;
            self
        }
        pub fn update_project_hosted_tool_permissions_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_hosted_tool_permissions_error409 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_hosted_tool_permissions_error409: {e}")) ;
            self
        }
        pub fn update_project_hosted_tool_permissions_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_hosted_tool_permissions_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_hosted_tool_permissions_error429: {e}")) ;
            self
        }
        pub fn update_project_hosted_tool_permissions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectHostedToolPermissionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_hosted_tool_permissions_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_hosted_tool_permissions_params: {e}")) ;
            self
        }
        pub fn update_project_hosted_tool_permissions_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectHostedToolPermissionsUpdateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_hosted_tool_permissions_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_hosted_tool_permissions_request: {e}")) ;
            self
        }
        pub fn update_project_hosted_tool_permissions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectHostedToolPermissions>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_hosted_tool_permissions_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_hosted_tool_permissions_response: {e}")) ;
            self
        }
    }
    impl ::std::convert::TryFrom<HostedToolsApiSchema> for super::HostedToolsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HostedToolsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                retrieve_project_hosted_tool_permissions_error400: value
                    .retrieve_project_hosted_tool_permissions_error400?,
                retrieve_project_hosted_tool_permissions_error403: value
                    .retrieve_project_hosted_tool_permissions_error403?,
                retrieve_project_hosted_tool_permissions_error404: value
                    .retrieve_project_hosted_tool_permissions_error404?,
                retrieve_project_hosted_tool_permissions_error429: value
                    .retrieve_project_hosted_tool_permissions_error429?,
                retrieve_project_hosted_tool_permissions_params: value
                    .retrieve_project_hosted_tool_permissions_params?,
                retrieve_project_hosted_tool_permissions_response: value
                    .retrieve_project_hosted_tool_permissions_response?,
                update_project_hosted_tool_permissions_error400: value
                    .update_project_hosted_tool_permissions_error400?,
                update_project_hosted_tool_permissions_error403: value
                    .update_project_hosted_tool_permissions_error403?,
                update_project_hosted_tool_permissions_error404: value
                    .update_project_hosted_tool_permissions_error404?,
                update_project_hosted_tool_permissions_error409: value
                    .update_project_hosted_tool_permissions_error409?,
                update_project_hosted_tool_permissions_error429: value
                    .update_project_hosted_tool_permissions_error429?,
                update_project_hosted_tool_permissions_params: value
                    .update_project_hosted_tool_permissions_params?,
                update_project_hosted_tool_permissions_request: value
                    .update_project_hosted_tool_permissions_request?,
                update_project_hosted_tool_permissions_response: value
                    .update_project_hosted_tool_permissions_response?,
            })
        }
    }
    impl ::std::convert::From<super::HostedToolsApiSchema> for HostedToolsApiSchema {
        fn from(value: super::HostedToolsApiSchema) -> Self {
            Self {
                retrieve_project_hosted_tool_permissions_error400: Ok(
                    value.retrieve_project_hosted_tool_permissions_error400
                ),
                retrieve_project_hosted_tool_permissions_error403: Ok(
                    value.retrieve_project_hosted_tool_permissions_error403
                ),
                retrieve_project_hosted_tool_permissions_error404: Ok(
                    value.retrieve_project_hosted_tool_permissions_error404
                ),
                retrieve_project_hosted_tool_permissions_error429: Ok(
                    value.retrieve_project_hosted_tool_permissions_error429
                ),
                retrieve_project_hosted_tool_permissions_params: Ok(
                    value.retrieve_project_hosted_tool_permissions_params
                ),
                retrieve_project_hosted_tool_permissions_response: Ok(
                    value.retrieve_project_hosted_tool_permissions_response
                ),
                update_project_hosted_tool_permissions_error400: Ok(
                    value.update_project_hosted_tool_permissions_error400
                ),
                update_project_hosted_tool_permissions_error403: Ok(
                    value.update_project_hosted_tool_permissions_error403
                ),
                update_project_hosted_tool_permissions_error404: Ok(
                    value.update_project_hosted_tool_permissions_error404
                ),
                update_project_hosted_tool_permissions_error409: Ok(
                    value.update_project_hosted_tool_permissions_error409
                ),
                update_project_hosted_tool_permissions_error429: Ok(
                    value.update_project_hosted_tool_permissions_error429
                ),
                update_project_hosted_tool_permissions_params: Ok(
                    value.update_project_hosted_tool_permissions_params
                ),
                update_project_hosted_tool_permissions_request: Ok(
                    value.update_project_hosted_tool_permissions_request
                ),
                update_project_hosted_tool_permissions_response: Ok(
                    value.update_project_hosted_tool_permissions_response
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectHostedToolPermissions {
        code_interpreter: ::std::result::Result<super::HostedToolPermission, ::std::string::String>,
        file_search: ::std::result::Result<super::HostedToolPermission, ::std::string::String>,
        image_generation: ::std::result::Result<super::HostedToolPermission, ::std::string::String>,
        mcp: ::std::result::Result<super::HostedToolPermission, ::std::string::String>,
        web_search: ::std::result::Result<super::HostedToolPermission, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectHostedToolPermissions {
        fn default() -> Self {
            Self {
                code_interpreter: Err("no value supplied for code_interpreter".to_string()),
                file_search: Err("no value supplied for file_search".to_string()),
                image_generation: Err("no value supplied for image_generation".to_string()),
                mcp: Err("no value supplied for mcp".to_string()),
                web_search: Err("no value supplied for web_search".to_string()),
            }
        }
    }
    impl ProjectHostedToolPermissions {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HostedToolPermission>,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HostedToolPermission>,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
        pub fn image_generation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HostedToolPermission>,
            T::Error: ::std::fmt::Display,
        {
            self.image_generation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_generation: {e}"));
            self
        }
        pub fn mcp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HostedToolPermission>,
            T::Error: ::std::fmt::Display,
        {
            self.mcp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mcp: {e}"));
            self
        }
        pub fn web_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HostedToolPermission>,
            T::Error: ::std::fmt::Display,
        {
            self.web_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for web_search: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectHostedToolPermissions> for super::ProjectHostedToolPermissions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectHostedToolPermissions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                file_search: value.file_search?,
                image_generation: value.image_generation?,
                mcp: value.mcp?,
                web_search: value.web_search?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectHostedToolPermissions> for ProjectHostedToolPermissions {
        fn from(value: super::ProjectHostedToolPermissions) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                file_search: Ok(value.file_search),
                image_generation: Ok(value.image_generation),
                mcp: Ok(value.mcp),
                web_search: Ok(value.web_search),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectHostedToolPermissionsUpdateRequest {
        code_interpreter: ::std::result::Result<
            ::std::option::Option<super::HostedToolPermissionUpdate>,
            ::std::string::String,
        >,
        file_search: ::std::result::Result<
            ::std::option::Option<super::HostedToolPermissionUpdate>,
            ::std::string::String,
        >,
        image_generation: ::std::result::Result<
            ::std::option::Option<super::HostedToolPermissionUpdate>,
            ::std::string::String,
        >,
        mcp: ::std::result::Result<
            ::std::option::Option<super::HostedToolPermissionUpdate>,
            ::std::string::String,
        >,
        web_search: ::std::result::Result<
            ::std::option::Option<super::HostedToolPermissionUpdate>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectHostedToolPermissionsUpdateRequest {
        fn default() -> Self {
            Self {
                code_interpreter: Ok(Default::default()),
                file_search: Ok(Default::default()),
                image_generation: Ok(Default::default()),
                mcp: Ok(Default::default()),
                web_search: Ok(Default::default()),
            }
        }
    }
    impl ProjectHostedToolPermissionsUpdateRequest {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HostedToolPermissionUpdate>>,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HostedToolPermissionUpdate>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
        pub fn image_generation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HostedToolPermissionUpdate>>,
            T::Error: ::std::fmt::Display,
        {
            self.image_generation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_generation: {e}"));
            self
        }
        pub fn mcp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HostedToolPermissionUpdate>>,
            T::Error: ::std::fmt::Display,
        {
            self.mcp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mcp: {e}"));
            self
        }
        pub fn web_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HostedToolPermissionUpdate>>,
            T::Error: ::std::fmt::Display,
        {
            self.web_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for web_search: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectHostedToolPermissionsUpdateRequest>
        for super::ProjectHostedToolPermissionsUpdateRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectHostedToolPermissionsUpdateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                file_search: value.file_search?,
                image_generation: value.image_generation?,
                mcp: value.mcp?,
                web_search: value.web_search?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectHostedToolPermissionsUpdateRequest>
        for ProjectHostedToolPermissionsUpdateRequest
    {
        fn from(value: super::ProjectHostedToolPermissionsUpdateRequest) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                file_search: Ok(value.file_search),
                image_generation: Ok(value.image_generation),
                mcp: Ok(value.mcp),
                web_search: Ok(value.web_search),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectHostedToolPermissionsParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectHostedToolPermissionsParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl RetrieveProjectHostedToolPermissionsParams {
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveProjectHostedToolPermissionsParams>
        for super::RetrieveProjectHostedToolPermissionsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectHostedToolPermissionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectHostedToolPermissionsParams>
        for RetrieveProjectHostedToolPermissionsParams
    {
        fn from(value: super::RetrieveProjectHostedToolPermissionsParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectHostedToolPermissionsParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectHostedToolPermissionsParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl UpdateProjectHostedToolPermissionsParams {
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateProjectHostedToolPermissionsParams>
        for super::UpdateProjectHostedToolPermissionsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectHostedToolPermissionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectHostedToolPermissionsParams>
        for UpdateProjectHostedToolPermissionsParams
    {
        fn from(value: super::UpdateProjectHostedToolPermissionsParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
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
