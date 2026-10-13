pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`DataRetentionApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DataRetentionApiSchema {
    #[serde(rename = "RetrieveOrganizationDataRetentionError403")]
    pub retrieve_organization_data_retention_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveOrganizationDataRetentionError404")]
    pub retrieve_organization_data_retention_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveOrganizationDataRetentionResponse")]
    pub retrieve_organization_data_retention_response: OrganizationDataRetention,
    #[serde(rename = "RetrieveProjectDataRetentionError400")]
    pub retrieve_project_data_retention_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectDataRetentionError403")]
    pub retrieve_project_data_retention_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectDataRetentionError404")]
    pub retrieve_project_data_retention_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectDataRetentionParams")]
    pub retrieve_project_data_retention_params: RetrieveProjectDataRetentionParams,
    #[serde(rename = "RetrieveProjectDataRetentionResponse")]
    pub retrieve_project_data_retention_response: ProjectDataRetention,
    #[serde(rename = "UpdateOrganizationDataRetentionError400")]
    pub update_organization_data_retention_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateOrganizationDataRetentionError403")]
    pub update_organization_data_retention_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateOrganizationDataRetentionError404")]
    pub update_organization_data_retention_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateOrganizationDataRetentionRequest")]
    pub update_organization_data_retention_request: UpdateOrganizationDataRetentionBody,
    #[serde(rename = "UpdateOrganizationDataRetentionResponse")]
    pub update_organization_data_retention_response: OrganizationDataRetention,
    #[serde(rename = "UpdateProjectDataRetentionError400")]
    pub update_project_data_retention_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectDataRetentionError403")]
    pub update_project_data_retention_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateProjectDataRetentionError404")]
    pub update_project_data_retention_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectDataRetentionParams")]
    pub update_project_data_retention_params: UpdateProjectDataRetentionParams,
    #[serde(rename = "UpdateProjectDataRetentionRequest")]
    pub update_project_data_retention_request: UpdateProjectDataRetentionBody,
    #[serde(rename = "UpdateProjectDataRetentionResponse")]
    pub update_project_data_retention_response: ProjectDataRetention,
}
impl DataRetentionApiSchema {
    pub fn builder() -> builder::DataRetentionApiSchema {
        Default::default()
    }
}
#[doc = "Represents the organization's data retention control setting."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationDataRetention {
    #[doc = "The object type, which is always `organization.data_retention`."]
    pub object: OrganizationDataRetentionObject,
    #[doc = "The configured organization data retention type."]
    #[serde(rename = "type")]
    pub type_: OrganizationDataRetentionType,
}
impl OrganizationDataRetention {
    pub fn builder() -> builder::OrganizationDataRetention {
        Default::default()
    }
}
#[doc = "The object type, which is always `organization.data_retention`."]
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
pub enum OrganizationDataRetentionObject {
    #[serde(rename = "organization.data_retention")]
    OrganizationDataRetention,
}
impl ::std::fmt::Display for OrganizationDataRetentionObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationDataRetention => f.write_str("organization.data_retention"),
        }
    }
}
impl ::std::str::FromStr for OrganizationDataRetentionObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.data_retention" => Ok(Self::OrganizationDataRetention),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationDataRetentionObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationDataRetentionObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The configured organization data retention type."]
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
pub enum OrganizationDataRetentionType {
    #[serde(rename = "zero_data_retention")]
    ZeroDataRetention,
    #[serde(rename = "modified_abuse_monitoring")]
    ModifiedAbuseMonitoring,
    #[serde(rename = "enhanced_zero_data_retention")]
    EnhancedZeroDataRetention,
    #[serde(rename = "enhanced_modified_abuse_monitoring")]
    EnhancedModifiedAbuseMonitoring,
}
impl ::std::fmt::Display for OrganizationDataRetentionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ZeroDataRetention => f.write_str("zero_data_retention"),
            Self::ModifiedAbuseMonitoring => f.write_str("modified_abuse_monitoring"),
            Self::EnhancedZeroDataRetention => f.write_str("enhanced_zero_data_retention"),
            Self::EnhancedModifiedAbuseMonitoring => {
                f.write_str("enhanced_modified_abuse_monitoring")
            }
        }
    }
}
impl ::std::str::FromStr for OrganizationDataRetentionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "zero_data_retention" => Ok(Self::ZeroDataRetention),
            "modified_abuse_monitoring" => Ok(Self::ModifiedAbuseMonitoring),
            "enhanced_zero_data_retention" => Ok(Self::EnhancedZeroDataRetention),
            "enhanced_modified_abuse_monitoring" => Ok(Self::EnhancedModifiedAbuseMonitoring),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationDataRetentionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationDataRetentionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a project's data retention control setting."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectDataRetention {
    #[doc = "The object type, which is always `project.data_retention`."]
    pub object: ProjectDataRetentionObject,
    #[doc = "The configured project data retention type."]
    #[serde(rename = "type")]
    pub type_: ProjectDataRetentionType,
}
impl ProjectDataRetention {
    pub fn builder() -> builder::ProjectDataRetention {
        Default::default()
    }
}
#[doc = "The object type, which is always `project.data_retention`."]
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
pub enum ProjectDataRetentionObject {
    #[serde(rename = "project.data_retention")]
    ProjectDataRetention,
}
impl ::std::fmt::Display for ProjectDataRetentionObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectDataRetention => f.write_str("project.data_retention"),
        }
    }
}
impl ::std::str::FromStr for ProjectDataRetentionObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.data_retention" => Ok(Self::ProjectDataRetention),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectDataRetentionObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectDataRetentionObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The configured project data retention type."]
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
pub enum ProjectDataRetentionType {
    #[serde(rename = "organization_default")]
    OrganizationDefault,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "zero_data_retention")]
    ZeroDataRetention,
    #[serde(rename = "modified_abuse_monitoring")]
    ModifiedAbuseMonitoring,
    #[serde(rename = "enhanced_zero_data_retention")]
    EnhancedZeroDataRetention,
    #[serde(rename = "enhanced_modified_abuse_monitoring")]
    EnhancedModifiedAbuseMonitoring,
}
impl ::std::fmt::Display for ProjectDataRetentionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationDefault => f.write_str("organization_default"),
            Self::None => f.write_str("none"),
            Self::ZeroDataRetention => f.write_str("zero_data_retention"),
            Self::ModifiedAbuseMonitoring => f.write_str("modified_abuse_monitoring"),
            Self::EnhancedZeroDataRetention => f.write_str("enhanced_zero_data_retention"),
            Self::EnhancedModifiedAbuseMonitoring => {
                f.write_str("enhanced_modified_abuse_monitoring")
            }
        }
    }
}
impl ::std::str::FromStr for ProjectDataRetentionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization_default" => Ok(Self::OrganizationDefault),
            "none" => Ok(Self::None),
            "zero_data_retention" => Ok(Self::ZeroDataRetention),
            "modified_abuse_monitoring" => Ok(Self::ModifiedAbuseMonitoring),
            "enhanced_zero_data_retention" => Ok(Self::EnhancedZeroDataRetention),
            "enhanced_modified_abuse_monitoring" => Ok(Self::EnhancedModifiedAbuseMonitoring),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectDataRetentionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectDataRetentionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RetrieveProjectDataRetentionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectDataRetentionParams {
    pub project_id: ::std::string::String,
}
impl RetrieveProjectDataRetentionParams {
    pub fn builder() -> builder::RetrieveProjectDataRetentionParams {
        Default::default()
    }
}
#[doc = "Parameters for updating organization data retention controls."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateOrganizationDataRetentionBody {
    #[doc = "The desired organization data retention type."]
    pub retention_type: UpdateOrganizationDataRetentionBodyRetentionType,
}
impl UpdateOrganizationDataRetentionBody {
    pub fn builder() -> builder::UpdateOrganizationDataRetentionBody {
        Default::default()
    }
}
#[doc = "The desired organization data retention type."]
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
pub enum UpdateOrganizationDataRetentionBodyRetentionType {
    #[serde(rename = "zero_data_retention")]
    ZeroDataRetention,
    #[serde(rename = "modified_abuse_monitoring")]
    ModifiedAbuseMonitoring,
    #[serde(rename = "enhanced_zero_data_retention")]
    EnhancedZeroDataRetention,
    #[serde(rename = "enhanced_modified_abuse_monitoring")]
    EnhancedModifiedAbuseMonitoring,
}
impl ::std::fmt::Display for UpdateOrganizationDataRetentionBodyRetentionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ZeroDataRetention => f.write_str("zero_data_retention"),
            Self::ModifiedAbuseMonitoring => f.write_str("modified_abuse_monitoring"),
            Self::EnhancedZeroDataRetention => f.write_str("enhanced_zero_data_retention"),
            Self::EnhancedModifiedAbuseMonitoring => {
                f.write_str("enhanced_modified_abuse_monitoring")
            }
        }
    }
}
impl ::std::str::FromStr for UpdateOrganizationDataRetentionBodyRetentionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "zero_data_retention" => Ok(Self::ZeroDataRetention),
            "modified_abuse_monitoring" => Ok(Self::ModifiedAbuseMonitoring),
            "enhanced_zero_data_retention" => Ok(Self::EnhancedZeroDataRetention),
            "enhanced_modified_abuse_monitoring" => Ok(Self::EnhancedModifiedAbuseMonitoring),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpdateOrganizationDataRetentionBodyRetentionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for UpdateOrganizationDataRetentionBodyRetentionType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Parameters for updating project data retention controls."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectDataRetentionBody {
    #[doc = "The desired project data retention type."]
    pub retention_type: UpdateProjectDataRetentionBodyRetentionType,
}
impl UpdateProjectDataRetentionBody {
    pub fn builder() -> builder::UpdateProjectDataRetentionBody {
        Default::default()
    }
}
#[doc = "The desired project data retention type."]
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
pub enum UpdateProjectDataRetentionBodyRetentionType {
    #[serde(rename = "organization_default")]
    OrganizationDefault,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "zero_data_retention")]
    ZeroDataRetention,
    #[serde(rename = "modified_abuse_monitoring")]
    ModifiedAbuseMonitoring,
    #[serde(rename = "enhanced_zero_data_retention")]
    EnhancedZeroDataRetention,
    #[serde(rename = "enhanced_modified_abuse_monitoring")]
    EnhancedModifiedAbuseMonitoring,
}
impl ::std::fmt::Display for UpdateProjectDataRetentionBodyRetentionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationDefault => f.write_str("organization_default"),
            Self::None => f.write_str("none"),
            Self::ZeroDataRetention => f.write_str("zero_data_retention"),
            Self::ModifiedAbuseMonitoring => f.write_str("modified_abuse_monitoring"),
            Self::EnhancedZeroDataRetention => f.write_str("enhanced_zero_data_retention"),
            Self::EnhancedModifiedAbuseMonitoring => {
                f.write_str("enhanced_modified_abuse_monitoring")
            }
        }
    }
}
impl ::std::str::FromStr for UpdateProjectDataRetentionBodyRetentionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization_default" => Ok(Self::OrganizationDefault),
            "none" => Ok(Self::None),
            "zero_data_retention" => Ok(Self::ZeroDataRetention),
            "modified_abuse_monitoring" => Ok(Self::ModifiedAbuseMonitoring),
            "enhanced_zero_data_retention" => Ok(Self::EnhancedZeroDataRetention),
            "enhanced_modified_abuse_monitoring" => Ok(Self::EnhancedModifiedAbuseMonitoring),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpdateProjectDataRetentionBodyRetentionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for UpdateProjectDataRetentionBodyRetentionType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UpdateProjectDataRetentionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectDataRetentionParams {
    pub project_id: ::std::string::String,
}
impl UpdateProjectDataRetentionParams {
    pub fn builder() -> builder::UpdateProjectDataRetentionParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DataRetentionApiSchema {
        retrieve_organization_data_retention_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_organization_data_retention_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_organization_data_retention_response:
            ::std::result::Result<super::OrganizationDataRetention, ::std::string::String>,
        retrieve_project_data_retention_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_data_retention_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_data_retention_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_data_retention_params:
            ::std::result::Result<super::RetrieveProjectDataRetentionParams, ::std::string::String>,
        retrieve_project_data_retention_response:
            ::std::result::Result<super::ProjectDataRetention, ::std::string::String>,
        update_organization_data_retention_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_organization_data_retention_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_organization_data_retention_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_organization_data_retention_request: ::std::result::Result<
            super::UpdateOrganizationDataRetentionBody,
            ::std::string::String,
        >,
        update_organization_data_retention_response:
            ::std::result::Result<super::OrganizationDataRetention, ::std::string::String>,
        update_project_data_retention_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_data_retention_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_project_data_retention_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_data_retention_params:
            ::std::result::Result<super::UpdateProjectDataRetentionParams, ::std::string::String>,
        update_project_data_retention_request:
            ::std::result::Result<super::UpdateProjectDataRetentionBody, ::std::string::String>,
        update_project_data_retention_response:
            ::std::result::Result<super::ProjectDataRetention, ::std::string::String>,
    }
    impl ::std::default::Default for DataRetentionApiSchema {
        fn default() -> Self {
            Self {
                retrieve_organization_data_retention_error403: Err(
                    "no value supplied for retrieve_organization_data_retention_error403"
                        .to_string(),
                ),
                retrieve_organization_data_retention_error404: Err(
                    "no value supplied for retrieve_organization_data_retention_error404"
                        .to_string(),
                ),
                retrieve_organization_data_retention_response: Err(
                    "no value supplied for retrieve_organization_data_retention_response"
                        .to_string(),
                ),
                retrieve_project_data_retention_error400: Err(
                    "no value supplied for retrieve_project_data_retention_error400".to_string(),
                ),
                retrieve_project_data_retention_error403: Err(
                    "no value supplied for retrieve_project_data_retention_error403".to_string(),
                ),
                retrieve_project_data_retention_error404: Err(
                    "no value supplied for retrieve_project_data_retention_error404".to_string(),
                ),
                retrieve_project_data_retention_params: Err(
                    "no value supplied for retrieve_project_data_retention_params".to_string(),
                ),
                retrieve_project_data_retention_response: Err(
                    "no value supplied for retrieve_project_data_retention_response".to_string(),
                ),
                update_organization_data_retention_error400: Err(
                    "no value supplied for update_organization_data_retention_error400".to_string(),
                ),
                update_organization_data_retention_error403: Err(
                    "no value supplied for update_organization_data_retention_error403".to_string(),
                ),
                update_organization_data_retention_error404: Err(
                    "no value supplied for update_organization_data_retention_error404".to_string(),
                ),
                update_organization_data_retention_request: Err(
                    "no value supplied for update_organization_data_retention_request".to_string(),
                ),
                update_organization_data_retention_response: Err(
                    "no value supplied for update_organization_data_retention_response".to_string(),
                ),
                update_project_data_retention_error400: Err(
                    "no value supplied for update_project_data_retention_error400".to_string(),
                ),
                update_project_data_retention_error403: Err(
                    "no value supplied for update_project_data_retention_error403".to_string(),
                ),
                update_project_data_retention_error404: Err(
                    "no value supplied for update_project_data_retention_error404".to_string(),
                ),
                update_project_data_retention_params: Err(
                    "no value supplied for update_project_data_retention_params".to_string(),
                ),
                update_project_data_retention_request: Err(
                    "no value supplied for update_project_data_retention_request".to_string(),
                ),
                update_project_data_retention_response: Err(
                    "no value supplied for update_project_data_retention_response".to_string(),
                ),
            }
        }
    }
    impl DataRetentionApiSchema {
        pub fn retrieve_organization_data_retention_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_organization_data_retention_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_organization_data_retention_error403: {e}")) ;
            self
        }
        pub fn retrieve_organization_data_retention_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_organization_data_retention_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_organization_data_retention_error404: {e}")) ;
            self
        }
        pub fn retrieve_organization_data_retention_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationDataRetention>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_organization_data_retention_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_organization_data_retention_response: {e}")) ;
            self
        }
        pub fn retrieve_project_data_retention_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_data_retention_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_data_retention_error400: {e}")) ;
            self
        }
        pub fn retrieve_project_data_retention_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_data_retention_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_data_retention_error403: {e}")) ;
            self
        }
        pub fn retrieve_project_data_retention_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_data_retention_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_data_retention_error404: {e}")) ;
            self
        }
        pub fn retrieve_project_data_retention_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectDataRetentionParams>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_data_retention_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_data_retention_params: {e}")) ;
            self
        }
        pub fn retrieve_project_data_retention_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectDataRetention>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_data_retention_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_data_retention_response: {e}")) ;
            self
        }
        pub fn update_organization_data_retention_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_data_retention_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_data_retention_error400: {e}")) ;
            self
        }
        pub fn update_organization_data_retention_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_data_retention_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_data_retention_error403: {e}")) ;
            self
        }
        pub fn update_organization_data_retention_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_data_retention_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_data_retention_error404: {e}")) ;
            self
        }
        pub fn update_organization_data_retention_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateOrganizationDataRetentionBody>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_data_retention_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_data_retention_request: {e}")) ;
            self
        }
        pub fn update_organization_data_retention_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationDataRetention>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_data_retention_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_data_retention_response: {e}")) ;
            self
        }
        pub fn update_project_data_retention_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_data_retention_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_data_retention_error400: {e}")) ;
            self
        }
        pub fn update_project_data_retention_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_data_retention_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_data_retention_error403: {e}")) ;
            self
        }
        pub fn update_project_data_retention_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_data_retention_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_data_retention_error404: {e}")) ;
            self
        }
        pub fn update_project_data_retention_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectDataRetentionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_data_retention_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_data_retention_params: {e}"
                )
            });
            self
        }
        pub fn update_project_data_retention_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectDataRetentionBody>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_data_retention_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_data_retention_request: {e}")) ;
            self
        }
        pub fn update_project_data_retention_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectDataRetention>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_data_retention_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_data_retention_response: {e}")) ;
            self
        }
    }
    impl ::std::convert::TryFrom<DataRetentionApiSchema> for super::DataRetentionApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DataRetentionApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                retrieve_organization_data_retention_error403: value
                    .retrieve_organization_data_retention_error403?,
                retrieve_organization_data_retention_error404: value
                    .retrieve_organization_data_retention_error404?,
                retrieve_organization_data_retention_response: value
                    .retrieve_organization_data_retention_response?,
                retrieve_project_data_retention_error400: value
                    .retrieve_project_data_retention_error400?,
                retrieve_project_data_retention_error403: value
                    .retrieve_project_data_retention_error403?,
                retrieve_project_data_retention_error404: value
                    .retrieve_project_data_retention_error404?,
                retrieve_project_data_retention_params: value
                    .retrieve_project_data_retention_params?,
                retrieve_project_data_retention_response: value
                    .retrieve_project_data_retention_response?,
                update_organization_data_retention_error400: value
                    .update_organization_data_retention_error400?,
                update_organization_data_retention_error403: value
                    .update_organization_data_retention_error403?,
                update_organization_data_retention_error404: value
                    .update_organization_data_retention_error404?,
                update_organization_data_retention_request: value
                    .update_organization_data_retention_request?,
                update_organization_data_retention_response: value
                    .update_organization_data_retention_response?,
                update_project_data_retention_error400: value
                    .update_project_data_retention_error400?,
                update_project_data_retention_error403: value
                    .update_project_data_retention_error403?,
                update_project_data_retention_error404: value
                    .update_project_data_retention_error404?,
                update_project_data_retention_params: value.update_project_data_retention_params?,
                update_project_data_retention_request: value
                    .update_project_data_retention_request?,
                update_project_data_retention_response: value
                    .update_project_data_retention_response?,
            })
        }
    }
    impl ::std::convert::From<super::DataRetentionApiSchema> for DataRetentionApiSchema {
        fn from(value: super::DataRetentionApiSchema) -> Self {
            Self {
                retrieve_organization_data_retention_error403: Ok(
                    value.retrieve_organization_data_retention_error403
                ),
                retrieve_organization_data_retention_error404: Ok(
                    value.retrieve_organization_data_retention_error404
                ),
                retrieve_organization_data_retention_response: Ok(
                    value.retrieve_organization_data_retention_response
                ),
                retrieve_project_data_retention_error400: Ok(
                    value.retrieve_project_data_retention_error400
                ),
                retrieve_project_data_retention_error403: Ok(
                    value.retrieve_project_data_retention_error403
                ),
                retrieve_project_data_retention_error404: Ok(
                    value.retrieve_project_data_retention_error404
                ),
                retrieve_project_data_retention_params: Ok(
                    value.retrieve_project_data_retention_params
                ),
                retrieve_project_data_retention_response: Ok(
                    value.retrieve_project_data_retention_response
                ),
                update_organization_data_retention_error400: Ok(
                    value.update_organization_data_retention_error400
                ),
                update_organization_data_retention_error403: Ok(
                    value.update_organization_data_retention_error403
                ),
                update_organization_data_retention_error404: Ok(
                    value.update_organization_data_retention_error404
                ),
                update_organization_data_retention_request: Ok(
                    value.update_organization_data_retention_request
                ),
                update_organization_data_retention_response: Ok(
                    value.update_organization_data_retention_response
                ),
                update_project_data_retention_error400: Ok(
                    value.update_project_data_retention_error400
                ),
                update_project_data_retention_error403: Ok(
                    value.update_project_data_retention_error403
                ),
                update_project_data_retention_error404: Ok(
                    value.update_project_data_retention_error404
                ),
                update_project_data_retention_params: Ok(value.update_project_data_retention_params),
                update_project_data_retention_request: Ok(
                    value.update_project_data_retention_request
                ),
                update_project_data_retention_response: Ok(
                    value.update_project_data_retention_response
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationDataRetention {
        object:
            ::std::result::Result<super::OrganizationDataRetentionObject, ::std::string::String>,
        type_: ::std::result::Result<super::OrganizationDataRetentionType, ::std::string::String>,
    }
    impl ::std::default::Default for OrganizationDataRetention {
        fn default() -> Self {
            Self {
                object: Err("no value supplied for object".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl OrganizationDataRetention {
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationDataRetentionObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationDataRetentionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationDataRetention> for super::OrganizationDataRetention {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationDataRetention,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                object: value.object?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationDataRetention> for OrganizationDataRetention {
        fn from(value: super::OrganizationDataRetention) -> Self {
            Self {
                object: Ok(value.object),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectDataRetention {
        object: ::std::result::Result<super::ProjectDataRetentionObject, ::std::string::String>,
        type_: ::std::result::Result<super::ProjectDataRetentionType, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectDataRetention {
        fn default() -> Self {
            Self {
                object: Err("no value supplied for object".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ProjectDataRetention {
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectDataRetentionObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectDataRetentionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectDataRetention> for super::ProjectDataRetention {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectDataRetention,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                object: value.object?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectDataRetention> for ProjectDataRetention {
        fn from(value: super::ProjectDataRetention) -> Self {
            Self {
                object: Ok(value.object),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectDataRetentionParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectDataRetentionParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl RetrieveProjectDataRetentionParams {
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
    impl ::std::convert::TryFrom<RetrieveProjectDataRetentionParams>
        for super::RetrieveProjectDataRetentionParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectDataRetentionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectDataRetentionParams>
        for RetrieveProjectDataRetentionParams
    {
        fn from(value: super::RetrieveProjectDataRetentionParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateOrganizationDataRetentionBody {
        retention_type: ::std::result::Result<
            super::UpdateOrganizationDataRetentionBodyRetentionType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateOrganizationDataRetentionBody {
        fn default() -> Self {
            Self {
                retention_type: Err("no value supplied for retention_type".to_string()),
            }
        }
    }
    impl UpdateOrganizationDataRetentionBody {
        pub fn retention_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateOrganizationDataRetentionBodyRetentionType>,
            T::Error: ::std::fmt::Display,
        {
            self.retention_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for retention_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateOrganizationDataRetentionBody>
        for super::UpdateOrganizationDataRetentionBody
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateOrganizationDataRetentionBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                retention_type: value.retention_type?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateOrganizationDataRetentionBody>
        for UpdateOrganizationDataRetentionBody
    {
        fn from(value: super::UpdateOrganizationDataRetentionBody) -> Self {
            Self {
                retention_type: Ok(value.retention_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectDataRetentionBody {
        retention_type: ::std::result::Result<
            super::UpdateProjectDataRetentionBodyRetentionType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateProjectDataRetentionBody {
        fn default() -> Self {
            Self {
                retention_type: Err("no value supplied for retention_type".to_string()),
            }
        }
    }
    impl UpdateProjectDataRetentionBody {
        pub fn retention_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectDataRetentionBodyRetentionType>,
            T::Error: ::std::fmt::Display,
        {
            self.retention_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for retention_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateProjectDataRetentionBody>
        for super::UpdateProjectDataRetentionBody
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectDataRetentionBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                retention_type: value.retention_type?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectDataRetentionBody>
        for UpdateProjectDataRetentionBody
    {
        fn from(value: super::UpdateProjectDataRetentionBody) -> Self {
            Self {
                retention_type: Ok(value.retention_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectDataRetentionParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectDataRetentionParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl UpdateProjectDataRetentionParams {
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
    impl ::std::convert::TryFrom<UpdateProjectDataRetentionParams>
        for super::UpdateProjectDataRetentionParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectDataRetentionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectDataRetentionParams>
        for UpdateProjectDataRetentionParams
    {
        fn from(value: super::UpdateProjectDataRetentionParams) -> Self {
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
