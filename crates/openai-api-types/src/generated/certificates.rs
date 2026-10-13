pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`ActivateProjectCertificatesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ActivateProjectCertificatesParams {
    pub project_id: ::std::string::String,
}
impl ActivateProjectCertificatesParams {
    pub fn builder() -> builder::ActivateProjectCertificatesParams {
        Default::default()
    }
}
#[doc = "Represents an individual `certificate` uploaded to the organization."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Certificate {
    #[doc = "Whether the certificate is currently active at the specified scope. Not returned when getting details for a specific certificate."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub active: ::std::option::Option<bool>,
    pub certificate_details: CertificateCertificateDetails,
    #[doc = "The Unix timestamp (in seconds) of when the certificate was uploaded."]
    pub created_at: i64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The name of the certificate."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type.\n\n- If creating, updating, or getting a specific certificate, the object type is `certificate`.\n- If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`.\n- If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`.\n"]
    pub object: CertificateObject,
}
impl Certificate {
    pub fn builder() -> builder::Certificate {
        Default::default()
    }
}
#[doc = "`CertificateCertificateDetails`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateCertificateDetails {
    #[doc = "The content of the certificate in PEM format."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[doc = "The Unix timestamp (in seconds) of when the certificate expires."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) of when the certificate becomes valid."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub valid_at: ::std::option::Option<i64>,
}
impl CertificateCertificateDetails {
    pub fn builder() -> builder::CertificateCertificateDetails {
        Default::default()
    }
}
#[doc = "The object type.\n\n- If creating, updating, or getting a specific certificate, the object type is `certificate`.\n- If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`.\n- If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`.\n"]
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
pub enum CertificateObject {
    #[serde(rename = "certificate")]
    Certificate,
    #[serde(rename = "organization.certificate")]
    OrganizationCertificate,
    #[serde(rename = "organization.project.certificate")]
    OrganizationProjectCertificate,
}
impl ::std::fmt::Display for CertificateObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Certificate => f.write_str("certificate"),
            Self::OrganizationCertificate => f.write_str("organization.certificate"),
            Self::OrganizationProjectCertificate => f.write_str("organization.project.certificate"),
        }
    }
}
impl ::std::str::FromStr for CertificateObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "certificate" => Ok(Self::Certificate),
            "organization.certificate" => Ok(Self::OrganizationCertificate),
            "organization.project.certificate" => Ok(Self::OrganizationProjectCertificate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CertificateObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CertificateObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CertificatesApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CertificatesApiSchema {
    #[serde(rename = "ActivateOrganizationCertificatesError400")]
    pub activate_organization_certificates_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ActivateOrganizationCertificatesError403")]
    pub activate_organization_certificates_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ActivateOrganizationCertificatesError404")]
    pub activate_organization_certificates_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ActivateOrganizationCertificatesError429")]
    pub activate_organization_certificates_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ActivateOrganizationCertificatesError500")]
    pub activate_organization_certificates_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ActivateOrganizationCertificatesRequest")]
    pub activate_organization_certificates_request: ToggleCertificatesRequest,
    #[serde(rename = "ActivateOrganizationCertificatesResponse")]
    pub activate_organization_certificates_response: OrganizationCertificateActivationResponse,
    #[serde(rename = "ActivateProjectCertificatesError400")]
    pub activate_project_certificates_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ActivateProjectCertificatesError403")]
    pub activate_project_certificates_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ActivateProjectCertificatesError404")]
    pub activate_project_certificates_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ActivateProjectCertificatesError429")]
    pub activate_project_certificates_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ActivateProjectCertificatesError500")]
    pub activate_project_certificates_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ActivateProjectCertificatesParams")]
    pub activate_project_certificates_params: ActivateProjectCertificatesParams,
    #[serde(rename = "ActivateProjectCertificatesRequest")]
    pub activate_project_certificates_request: ToggleCertificatesRequest,
    #[serde(rename = "ActivateProjectCertificatesResponse")]
    pub activate_project_certificates_response: OrganizationProjectCertificateActivationResponse,
    #[serde(rename = "DeactivateOrganizationCertificatesError400")]
    pub deactivate_organization_certificates_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeactivateOrganizationCertificatesError403")]
    pub deactivate_organization_certificates_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeactivateOrganizationCertificatesError404")]
    pub deactivate_organization_certificates_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeactivateOrganizationCertificatesError429")]
    pub deactivate_organization_certificates_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeactivateOrganizationCertificatesError500")]
    pub deactivate_organization_certificates_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeactivateOrganizationCertificatesRequest")]
    pub deactivate_organization_certificates_request: ToggleCertificatesRequest,
    #[serde(rename = "DeactivateOrganizationCertificatesResponse")]
    pub deactivate_organization_certificates_response: OrganizationCertificateDeactivationResponse,
    #[serde(rename = "DeactivateProjectCertificatesError400")]
    pub deactivate_project_certificates_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeactivateProjectCertificatesError403")]
    pub deactivate_project_certificates_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeactivateProjectCertificatesError404")]
    pub deactivate_project_certificates_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeactivateProjectCertificatesError429")]
    pub deactivate_project_certificates_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeactivateProjectCertificatesError500")]
    pub deactivate_project_certificates_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeactivateProjectCertificatesParams")]
    pub deactivate_project_certificates_params: DeactivateProjectCertificatesParams,
    #[serde(rename = "DeactivateProjectCertificatesRequest")]
    pub deactivate_project_certificates_request: ToggleCertificatesRequest,
    #[serde(rename = "DeactivateProjectCertificatesResponse")]
    pub deactivate_project_certificates_response:
        OrganizationProjectCertificateDeactivationResponse,
    #[serde(rename = "DeleteCertificateError400")]
    pub delete_certificate_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteCertificateError403")]
    pub delete_certificate_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteCertificateError404")]
    pub delete_certificate_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteCertificateError429")]
    pub delete_certificate_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteCertificateParams")]
    pub delete_certificate_params: DeleteCertificateParams,
    #[serde(rename = "DeleteCertificateResponse")]
    pub delete_certificate_response: DeleteCertificateResponse,
    #[serde(rename = "GetCertificateError400")]
    pub get_certificate_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetCertificateError403")]
    pub get_certificate_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "GetCertificateError404")]
    pub get_certificate_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetCertificateError429")]
    pub get_certificate_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetCertificateParams")]
    pub get_certificate_params: GetCertificateParams,
    #[serde(rename = "GetCertificateResponse")]
    pub get_certificate_response: Certificate,
    #[serde(rename = "ListOrganizationCertificatesError400")]
    pub list_organization_certificates_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListOrganizationCertificatesError403")]
    pub list_organization_certificates_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListOrganizationCertificatesError429")]
    pub list_organization_certificates_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListOrganizationCertificatesParams")]
    pub list_organization_certificates_params: ListOrganizationCertificatesParams,
    #[serde(rename = "ListOrganizationCertificatesResponse")]
    pub list_organization_certificates_response: ListCertificatesResponse,
    #[serde(rename = "ListProjectCertificatesError400")]
    pub list_project_certificates_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectCertificatesError403")]
    pub list_project_certificates_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectCertificatesError429")]
    pub list_project_certificates_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectCertificatesParams")]
    pub list_project_certificates_params: ListProjectCertificatesParams,
    #[serde(rename = "ListProjectCertificatesResponse")]
    pub list_project_certificates_response: ListProjectCertificatesResponse,
    #[serde(rename = "ModifyCertificateError400")]
    pub modify_certificate_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyCertificateError403")]
    pub modify_certificate_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ModifyCertificateError404")]
    pub modify_certificate_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyCertificateError429")]
    pub modify_certificate_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyCertificateError500")]
    pub modify_certificate_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyCertificateParams")]
    pub modify_certificate_params: ModifyCertificateParams,
    #[serde(rename = "ModifyCertificateRequest")]
    pub modify_certificate_request: ModifyCertificateRequest,
    #[serde(rename = "ModifyCertificateResponse")]
    pub modify_certificate_response: Certificate,
    #[serde(rename = "UploadCertificateError400")]
    pub upload_certificate_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UploadCertificateError403")]
    pub upload_certificate_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UploadCertificateError429")]
    pub upload_certificate_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UploadCertificateRequest")]
    pub upload_certificate_request: UploadCertificateRequest,
    #[serde(rename = "UploadCertificateResponse")]
    pub upload_certificate_response: Certificate,
}
impl CertificatesApiSchema {
    pub fn builder() -> builder::CertificatesApiSchema {
        Default::default()
    }
}
#[doc = "`DeactivateProjectCertificatesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeactivateProjectCertificatesParams {
    pub project_id: ::std::string::String,
}
impl DeactivateProjectCertificatesParams {
    pub fn builder() -> builder::DeactivateProjectCertificatesParams {
        Default::default()
    }
}
#[doc = "`DeleteCertificateParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteCertificateParams {
    pub certificate_id: ::std::string::String,
}
impl DeleteCertificateParams {
    pub fn builder() -> builder::DeleteCertificateParams {
        Default::default()
    }
}
#[doc = "`DeleteCertificateResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteCertificateResponse {
    #[doc = "The ID of the certificate that was deleted."]
    pub id: ::std::string::String,
    #[doc = "The object type, must be `certificate.deleted`."]
    pub object: DeleteCertificateResponseObject,
}
impl DeleteCertificateResponse {
    pub fn builder() -> builder::DeleteCertificateResponse {
        Default::default()
    }
}
#[doc = "The object type, must be `certificate.deleted`."]
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
pub enum DeleteCertificateResponseObject {
    #[serde(rename = "certificate.deleted")]
    CertificateDeleted,
}
impl ::std::fmt::Display for DeleteCertificateResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CertificateDeleted => f.write_str("certificate.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeleteCertificateResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "certificate.deleted" => Ok(Self::CertificateDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteCertificateResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteCertificateResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetCertificateParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetCertificateParams {
    pub certificate_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<GetCertificateParamsIncludeItem>,
}
impl GetCertificateParams {
    pub fn builder() -> builder::GetCertificateParams {
        Default::default()
    }
}
#[doc = "`GetCertificateParamsIncludeItem`"]
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
pub enum GetCertificateParamsIncludeItem {
    #[serde(rename = "content")]
    Content,
}
impl ::std::fmt::Display for GetCertificateParamsIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Content => f.write_str("content"),
        }
    }
}
impl ::std::str::FromStr for GetCertificateParamsIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "content" => Ok(Self::Content),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetCertificateParamsIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetCertificateParamsIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListCertificatesResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListCertificatesResponse {
    pub data: ::std::vec::Vec<OrganizationCertificate>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ListCertificatesResponseObject,
}
impl ListCertificatesResponse {
    pub fn builder() -> builder::ListCertificatesResponse {
        Default::default()
    }
}
#[doc = "`ListCertificatesResponseObject`"]
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
pub enum ListCertificatesResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ListCertificatesResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ListCertificatesResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListCertificatesResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListCertificatesResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListOrganizationCertificatesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListOrganizationCertificatesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListOrganizationCertificatesParamsOrder>,
}
impl ListOrganizationCertificatesParams {
    pub fn builder() -> builder::ListOrganizationCertificatesParams {
        Default::default()
    }
}
#[doc = "`ListOrganizationCertificatesParamsOrder`"]
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
pub enum ListOrganizationCertificatesParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListOrganizationCertificatesParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListOrganizationCertificatesParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListOrganizationCertificatesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListOrganizationCertificatesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListProjectCertificatesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectCertificatesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListProjectCertificatesParamsOrder>,
    pub project_id: ::std::string::String,
}
impl ListProjectCertificatesParams {
    pub fn builder() -> builder::ListProjectCertificatesParams {
        Default::default()
    }
}
#[doc = "`ListProjectCertificatesParamsOrder`"]
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
pub enum ListProjectCertificatesParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListProjectCertificatesParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListProjectCertificatesParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListProjectCertificatesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListProjectCertificatesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListProjectCertificatesResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectCertificatesResponse {
    pub data: ::std::vec::Vec<OrganizationProjectCertificate>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ListProjectCertificatesResponseObject,
}
impl ListProjectCertificatesResponse {
    pub fn builder() -> builder::ListProjectCertificatesResponse {
        Default::default()
    }
}
#[doc = "`ListProjectCertificatesResponseObject`"]
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
pub enum ListProjectCertificatesResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ListProjectCertificatesResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ListProjectCertificatesResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListProjectCertificatesResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListProjectCertificatesResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ModifyCertificateParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModifyCertificateParams {
    pub certificate_id: ::std::string::String,
}
impl ModifyCertificateParams {
    pub fn builder() -> builder::ModifyCertificateParams {
        Default::default()
    }
}
#[doc = "`ModifyCertificateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModifyCertificateRequest {
    #[doc = "The updated name for the certificate"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ModifyCertificateRequest {
    pub fn builder() -> builder::ModifyCertificateRequest {
        Default::default()
    }
}
#[doc = "Represents an individual certificate configured at the organization level."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationCertificate {
    #[doc = "Whether the certificate is currently active at the organization level."]
    pub active: bool,
    pub certificate_details: OrganizationCertificateCertificateDetails,
    #[doc = "The Unix timestamp (in seconds) of when the certificate was uploaded."]
    pub created_at: i64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The name of the certificate."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `organization.certificate`."]
    pub object: OrganizationCertificateObject,
}
impl OrganizationCertificate {
    pub fn builder() -> builder::OrganizationCertificate {
        Default::default()
    }
}
#[doc = "`OrganizationCertificateActivationResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationCertificateActivationResponse {
    pub data: ::std::vec::Vec<OrganizationCertificate>,
    #[doc = "The organization certificate activation result type."]
    pub object: OrganizationCertificateActivationResponseObject,
}
impl OrganizationCertificateActivationResponse {
    pub fn builder() -> builder::OrganizationCertificateActivationResponse {
        Default::default()
    }
}
#[doc = "The organization certificate activation result type."]
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
pub enum OrganizationCertificateActivationResponseObject {
    #[serde(rename = "organization.certificate.activation")]
    OrganizationCertificateActivation,
}
impl ::std::fmt::Display for OrganizationCertificateActivationResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationCertificateActivation => {
                f.write_str("organization.certificate.activation")
            }
        }
    }
}
impl ::std::str::FromStr for OrganizationCertificateActivationResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.certificate.activation" => Ok(Self::OrganizationCertificateActivation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationCertificateActivationResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for OrganizationCertificateActivationResponseObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`OrganizationCertificateCertificateDetails`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct OrganizationCertificateCertificateDetails {
    #[doc = "The Unix timestamp (in seconds) of when the certificate expires."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) of when the certificate becomes valid."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub valid_at: ::std::option::Option<i64>,
}
impl OrganizationCertificateCertificateDetails {
    pub fn builder() -> builder::OrganizationCertificateCertificateDetails {
        Default::default()
    }
}
#[doc = "`OrganizationCertificateDeactivationResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationCertificateDeactivationResponse {
    pub data: ::std::vec::Vec<OrganizationCertificate>,
    #[doc = "The organization certificate deactivation result type."]
    pub object: OrganizationCertificateDeactivationResponseObject,
}
impl OrganizationCertificateDeactivationResponse {
    pub fn builder() -> builder::OrganizationCertificateDeactivationResponse {
        Default::default()
    }
}
#[doc = "The organization certificate deactivation result type."]
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
pub enum OrganizationCertificateDeactivationResponseObject {
    #[serde(rename = "organization.certificate.deactivation")]
    OrganizationCertificateDeactivation,
}
impl ::std::fmt::Display for OrganizationCertificateDeactivationResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationCertificateDeactivation => {
                f.write_str("organization.certificate.deactivation")
            }
        }
    }
}
impl ::std::str::FromStr for OrganizationCertificateDeactivationResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.certificate.deactivation" => {
                Ok(Self::OrganizationCertificateDeactivation)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationCertificateDeactivationResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for OrganizationCertificateDeactivationResponseObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `organization.certificate`."]
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
pub enum OrganizationCertificateObject {
    #[serde(rename = "organization.certificate")]
    OrganizationCertificate,
}
impl ::std::fmt::Display for OrganizationCertificateObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationCertificate => f.write_str("organization.certificate"),
        }
    }
}
impl ::std::str::FromStr for OrganizationCertificateObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.certificate" => Ok(Self::OrganizationCertificate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationCertificateObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationCertificateObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents an individual certificate configured at the project level."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationProjectCertificate {
    #[doc = "Whether the certificate is currently active at the project level."]
    pub active: bool,
    pub certificate_details: OrganizationProjectCertificateCertificateDetails,
    #[doc = "The Unix timestamp (in seconds) of when the certificate was uploaded."]
    pub created_at: i64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The name of the certificate."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `organization.project.certificate`."]
    pub object: OrganizationProjectCertificateObject,
}
impl OrganizationProjectCertificate {
    pub fn builder() -> builder::OrganizationProjectCertificate {
        Default::default()
    }
}
#[doc = "`OrganizationProjectCertificateActivationResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationProjectCertificateActivationResponse {
    pub data: ::std::vec::Vec<OrganizationProjectCertificate>,
    #[doc = "The project certificate activation result type."]
    pub object: OrganizationProjectCertificateActivationResponseObject,
}
impl OrganizationProjectCertificateActivationResponse {
    pub fn builder() -> builder::OrganizationProjectCertificateActivationResponse {
        Default::default()
    }
}
#[doc = "The project certificate activation result type."]
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
pub enum OrganizationProjectCertificateActivationResponseObject {
    #[serde(rename = "organization.project.certificate.activation")]
    OrganizationProjectCertificateActivation,
}
impl ::std::fmt::Display for OrganizationProjectCertificateActivationResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectCertificateActivation => {
                f.write_str("organization.project.certificate.activation")
            }
        }
    }
}
impl ::std::str::FromStr for OrganizationProjectCertificateActivationResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.certificate.activation" => {
                Ok(Self::OrganizationProjectCertificateActivation)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationProjectCertificateActivationResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for OrganizationProjectCertificateActivationResponseObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`OrganizationProjectCertificateCertificateDetails`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct OrganizationProjectCertificateCertificateDetails {
    #[doc = "The Unix timestamp (in seconds) of when the certificate expires."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) of when the certificate becomes valid."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub valid_at: ::std::option::Option<i64>,
}
impl OrganizationProjectCertificateCertificateDetails {
    pub fn builder() -> builder::OrganizationProjectCertificateCertificateDetails {
        Default::default()
    }
}
#[doc = "`OrganizationProjectCertificateDeactivationResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationProjectCertificateDeactivationResponse {
    pub data: ::std::vec::Vec<OrganizationProjectCertificate>,
    #[doc = "The project certificate deactivation result type."]
    pub object: OrganizationProjectCertificateDeactivationResponseObject,
}
impl OrganizationProjectCertificateDeactivationResponse {
    pub fn builder() -> builder::OrganizationProjectCertificateDeactivationResponse {
        Default::default()
    }
}
#[doc = "The project certificate deactivation result type."]
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
pub enum OrganizationProjectCertificateDeactivationResponseObject {
    #[serde(rename = "organization.project.certificate.deactivation")]
    OrganizationProjectCertificateDeactivation,
}
impl ::std::fmt::Display for OrganizationProjectCertificateDeactivationResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectCertificateDeactivation => {
                f.write_str("organization.project.certificate.deactivation")
            }
        }
    }
}
impl ::std::str::FromStr for OrganizationProjectCertificateDeactivationResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.certificate.deactivation" => {
                Ok(Self::OrganizationProjectCertificateDeactivation)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationProjectCertificateDeactivationResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for OrganizationProjectCertificateDeactivationResponseObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `organization.project.certificate`."]
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
pub enum OrganizationProjectCertificateObject {
    #[serde(rename = "organization.project.certificate")]
    OrganizationProjectCertificate,
}
impl ::std::fmt::Display for OrganizationProjectCertificateObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectCertificate => f.write_str("organization.project.certificate"),
        }
    }
}
impl ::std::str::FromStr for OrganizationProjectCertificateObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.certificate" => Ok(Self::OrganizationProjectCertificate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationProjectCertificateObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationProjectCertificateObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ToggleCertificatesRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToggleCertificatesRequest {
    pub certificate_ids: ::std::vec::Vec<::std::string::String>,
}
impl ToggleCertificatesRequest {
    pub fn builder() -> builder::ToggleCertificatesRequest {
        Default::default()
    }
}
#[doc = "`UploadCertificateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UploadCertificateRequest {
    #[doc = "The certificate content in PEM format"]
    pub certificate: ::std::string::String,
    #[doc = "An optional name for the certificate"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl UploadCertificateRequest {
    pub fn builder() -> builder::UploadCertificateRequest {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ActivateProjectCertificatesParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActivateProjectCertificatesParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ActivateProjectCertificatesParams {
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
    impl ::std::convert::TryFrom<ActivateProjectCertificatesParams>
        for super::ActivateProjectCertificatesParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActivateProjectCertificatesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ActivateProjectCertificatesParams>
        for ActivateProjectCertificatesParams
    {
        fn from(value: super::ActivateProjectCertificatesParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Certificate {
        active: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        certificate_details:
            ::std::result::Result<super::CertificateCertificateDetails, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::CertificateObject, ::std::string::String>,
    }
    impl ::std::default::Default for Certificate {
        fn default() -> Self {
            Self {
                active: Ok(Default::default()),
                certificate_details: Err("no value supplied for certificate_details".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl Certificate {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active: {e}"));
            self
        }
        pub fn certificate_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CertificateCertificateDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for certificate_details: {e}")
            });
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CertificateObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Certificate> for super::Certificate {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Certificate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                certificate_details: value.certificate_details?,
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::Certificate> for Certificate {
        fn from(value: super::Certificate) -> Self {
            Self {
                active: Ok(value.active),
                certificate_details: Ok(value.certificate_details),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CertificateCertificateDetails {
        content: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        valid_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for CertificateCertificateDetails {
        fn default() -> Self {
            Self {
                content: Ok(Default::default()),
                expires_at: Ok(Default::default()),
                valid_at: Ok(Default::default()),
            }
        }
    }
    impl CertificateCertificateDetails {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn valid_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_at: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CertificateCertificateDetails>
        for super::CertificateCertificateDetails
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CertificateCertificateDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                expires_at: value.expires_at?,
                valid_at: value.valid_at?,
            })
        }
    }
    impl ::std::convert::From<super::CertificateCertificateDetails> for CertificateCertificateDetails {
        fn from(value: super::CertificateCertificateDetails) -> Self {
            Self {
                content: Ok(value.content),
                expires_at: Ok(value.expires_at),
                valid_at: Ok(value.valid_at),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CertificatesApiSchema {
        activate_organization_certificates_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        activate_organization_certificates_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        activate_organization_certificates_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        activate_organization_certificates_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        activate_organization_certificates_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        activate_organization_certificates_request:
            ::std::result::Result<super::ToggleCertificatesRequest, ::std::string::String>,
        activate_organization_certificates_response: ::std::result::Result<
            super::OrganizationCertificateActivationResponse,
            ::std::string::String,
        >,
        activate_project_certificates_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        activate_project_certificates_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        activate_project_certificates_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        activate_project_certificates_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        activate_project_certificates_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        activate_project_certificates_params:
            ::std::result::Result<super::ActivateProjectCertificatesParams, ::std::string::String>,
        activate_project_certificates_request:
            ::std::result::Result<super::ToggleCertificatesRequest, ::std::string::String>,
        activate_project_certificates_response: ::std::result::Result<
            super::OrganizationProjectCertificateActivationResponse,
            ::std::string::String,
        >,
        deactivate_organization_certificates_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deactivate_organization_certificates_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        deactivate_organization_certificates_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deactivate_organization_certificates_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deactivate_organization_certificates_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deactivate_organization_certificates_request:
            ::std::result::Result<super::ToggleCertificatesRequest, ::std::string::String>,
        deactivate_organization_certificates_response: ::std::result::Result<
            super::OrganizationCertificateDeactivationResponse,
            ::std::string::String,
        >,
        deactivate_project_certificates_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deactivate_project_certificates_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        deactivate_project_certificates_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deactivate_project_certificates_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deactivate_project_certificates_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deactivate_project_certificates_params: ::std::result::Result<
            super::DeactivateProjectCertificatesParams,
            ::std::string::String,
        >,
        deactivate_project_certificates_request:
            ::std::result::Result<super::ToggleCertificatesRequest, ::std::string::String>,
        deactivate_project_certificates_response: ::std::result::Result<
            super::OrganizationProjectCertificateDeactivationResponse,
            ::std::string::String,
        >,
        delete_certificate_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_certificate_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_certificate_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_certificate_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_certificate_params:
            ::std::result::Result<super::DeleteCertificateParams, ::std::string::String>,
        delete_certificate_response:
            ::std::result::Result<super::DeleteCertificateResponse, ::std::string::String>,
        get_certificate_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_certificate_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        get_certificate_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_certificate_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_certificate_params:
            ::std::result::Result<super::GetCertificateParams, ::std::string::String>,
        get_certificate_response: ::std::result::Result<super::Certificate, ::std::string::String>,
        list_organization_certificates_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_organization_certificates_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_organization_certificates_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_organization_certificates_params:
            ::std::result::Result<super::ListOrganizationCertificatesParams, ::std::string::String>,
        list_organization_certificates_response:
            ::std::result::Result<super::ListCertificatesResponse, ::std::string::String>,
        list_project_certificates_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_certificates_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_certificates_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_certificates_params:
            ::std::result::Result<super::ListProjectCertificatesParams, ::std::string::String>,
        list_project_certificates_response:
            ::std::result::Result<super::ListProjectCertificatesResponse, ::std::string::String>,
        modify_certificate_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_certificate_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        modify_certificate_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_certificate_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_certificate_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_certificate_params:
            ::std::result::Result<super::ModifyCertificateParams, ::std::string::String>,
        modify_certificate_request:
            ::std::result::Result<super::ModifyCertificateRequest, ::std::string::String>,
        modify_certificate_response:
            ::std::result::Result<super::Certificate, ::std::string::String>,
        upload_certificate_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        upload_certificate_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        upload_certificate_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        upload_certificate_request:
            ::std::result::Result<super::UploadCertificateRequest, ::std::string::String>,
        upload_certificate_response:
            ::std::result::Result<super::Certificate, ::std::string::String>,
    }
    impl ::std::default::Default for CertificatesApiSchema {
        fn default() -> Self {
            Self {
                activate_organization_certificates_error400: Err(
                    "no value supplied for activate_organization_certificates_error400".to_string(),
                ),
                activate_organization_certificates_error403: Err(
                    "no value supplied for activate_organization_certificates_error403".to_string(),
                ),
                activate_organization_certificates_error404: Err(
                    "no value supplied for activate_organization_certificates_error404".to_string(),
                ),
                activate_organization_certificates_error429: Err(
                    "no value supplied for activate_organization_certificates_error429".to_string(),
                ),
                activate_organization_certificates_error500: Err(
                    "no value supplied for activate_organization_certificates_error500".to_string(),
                ),
                activate_organization_certificates_request: Err(
                    "no value supplied for activate_organization_certificates_request".to_string(),
                ),
                activate_organization_certificates_response: Err(
                    "no value supplied for activate_organization_certificates_response".to_string(),
                ),
                activate_project_certificates_error400: Err(
                    "no value supplied for activate_project_certificates_error400".to_string(),
                ),
                activate_project_certificates_error403: Err(
                    "no value supplied for activate_project_certificates_error403".to_string(),
                ),
                activate_project_certificates_error404: Err(
                    "no value supplied for activate_project_certificates_error404".to_string(),
                ),
                activate_project_certificates_error429: Err(
                    "no value supplied for activate_project_certificates_error429".to_string(),
                ),
                activate_project_certificates_error500: Err(
                    "no value supplied for activate_project_certificates_error500".to_string(),
                ),
                activate_project_certificates_params: Err(
                    "no value supplied for activate_project_certificates_params".to_string(),
                ),
                activate_project_certificates_request: Err(
                    "no value supplied for activate_project_certificates_request".to_string(),
                ),
                activate_project_certificates_response: Err(
                    "no value supplied for activate_project_certificates_response".to_string(),
                ),
                deactivate_organization_certificates_error400: Err(
                    "no value supplied for deactivate_organization_certificates_error400"
                        .to_string(),
                ),
                deactivate_organization_certificates_error403: Err(
                    "no value supplied for deactivate_organization_certificates_error403"
                        .to_string(),
                ),
                deactivate_organization_certificates_error404: Err(
                    "no value supplied for deactivate_organization_certificates_error404"
                        .to_string(),
                ),
                deactivate_organization_certificates_error429: Err(
                    "no value supplied for deactivate_organization_certificates_error429"
                        .to_string(),
                ),
                deactivate_organization_certificates_error500: Err(
                    "no value supplied for deactivate_organization_certificates_error500"
                        .to_string(),
                ),
                deactivate_organization_certificates_request: Err(
                    "no value supplied for deactivate_organization_certificates_request"
                        .to_string(),
                ),
                deactivate_organization_certificates_response: Err(
                    "no value supplied for deactivate_organization_certificates_response"
                        .to_string(),
                ),
                deactivate_project_certificates_error400: Err(
                    "no value supplied for deactivate_project_certificates_error400".to_string(),
                ),
                deactivate_project_certificates_error403: Err(
                    "no value supplied for deactivate_project_certificates_error403".to_string(),
                ),
                deactivate_project_certificates_error404: Err(
                    "no value supplied for deactivate_project_certificates_error404".to_string(),
                ),
                deactivate_project_certificates_error429: Err(
                    "no value supplied for deactivate_project_certificates_error429".to_string(),
                ),
                deactivate_project_certificates_error500: Err(
                    "no value supplied for deactivate_project_certificates_error500".to_string(),
                ),
                deactivate_project_certificates_params: Err(
                    "no value supplied for deactivate_project_certificates_params".to_string(),
                ),
                deactivate_project_certificates_request: Err(
                    "no value supplied for deactivate_project_certificates_request".to_string(),
                ),
                deactivate_project_certificates_response: Err(
                    "no value supplied for deactivate_project_certificates_response".to_string(),
                ),
                delete_certificate_error400: Err(
                    "no value supplied for delete_certificate_error400".to_string(),
                ),
                delete_certificate_error403: Err(
                    "no value supplied for delete_certificate_error403".to_string(),
                ),
                delete_certificate_error404: Err(
                    "no value supplied for delete_certificate_error404".to_string(),
                ),
                delete_certificate_error429: Err(
                    "no value supplied for delete_certificate_error429".to_string(),
                ),
                delete_certificate_params: Err(
                    "no value supplied for delete_certificate_params".to_string()
                ),
                delete_certificate_response: Err(
                    "no value supplied for delete_certificate_response".to_string(),
                ),
                get_certificate_error400: Err(
                    "no value supplied for get_certificate_error400".to_string()
                ),
                get_certificate_error403: Err(
                    "no value supplied for get_certificate_error403".to_string()
                ),
                get_certificate_error404: Err(
                    "no value supplied for get_certificate_error404".to_string()
                ),
                get_certificate_error429: Err(
                    "no value supplied for get_certificate_error429".to_string()
                ),
                get_certificate_params: Err(
                    "no value supplied for get_certificate_params".to_string()
                ),
                get_certificate_response: Err(
                    "no value supplied for get_certificate_response".to_string()
                ),
                list_organization_certificates_error400: Err(
                    "no value supplied for list_organization_certificates_error400".to_string(),
                ),
                list_organization_certificates_error403: Err(
                    "no value supplied for list_organization_certificates_error403".to_string(),
                ),
                list_organization_certificates_error429: Err(
                    "no value supplied for list_organization_certificates_error429".to_string(),
                ),
                list_organization_certificates_params: Err(
                    "no value supplied for list_organization_certificates_params".to_string(),
                ),
                list_organization_certificates_response: Err(
                    "no value supplied for list_organization_certificates_response".to_string(),
                ),
                list_project_certificates_error400: Err(
                    "no value supplied for list_project_certificates_error400".to_string(),
                ),
                list_project_certificates_error403: Err(
                    "no value supplied for list_project_certificates_error403".to_string(),
                ),
                list_project_certificates_error429: Err(
                    "no value supplied for list_project_certificates_error429".to_string(),
                ),
                list_project_certificates_params: Err(
                    "no value supplied for list_project_certificates_params".to_string(),
                ),
                list_project_certificates_response: Err(
                    "no value supplied for list_project_certificates_response".to_string(),
                ),
                modify_certificate_error400: Err(
                    "no value supplied for modify_certificate_error400".to_string(),
                ),
                modify_certificate_error403: Err(
                    "no value supplied for modify_certificate_error403".to_string(),
                ),
                modify_certificate_error404: Err(
                    "no value supplied for modify_certificate_error404".to_string(),
                ),
                modify_certificate_error429: Err(
                    "no value supplied for modify_certificate_error429".to_string(),
                ),
                modify_certificate_error500: Err(
                    "no value supplied for modify_certificate_error500".to_string(),
                ),
                modify_certificate_params: Err(
                    "no value supplied for modify_certificate_params".to_string()
                ),
                modify_certificate_request: Err(
                    "no value supplied for modify_certificate_request".to_string()
                ),
                modify_certificate_response: Err(
                    "no value supplied for modify_certificate_response".to_string(),
                ),
                upload_certificate_error400: Err(
                    "no value supplied for upload_certificate_error400".to_string(),
                ),
                upload_certificate_error403: Err(
                    "no value supplied for upload_certificate_error403".to_string(),
                ),
                upload_certificate_error429: Err(
                    "no value supplied for upload_certificate_error429".to_string(),
                ),
                upload_certificate_request: Err(
                    "no value supplied for upload_certificate_request".to_string()
                ),
                upload_certificate_response: Err(
                    "no value supplied for upload_certificate_response".to_string(),
                ),
            }
        }
    }
    impl CertificatesApiSchema {
        pub fn activate_organization_certificates_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_organization_certificates_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_organization_certificates_error400: {e}")) ;
            self
        }
        pub fn activate_organization_certificates_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_organization_certificates_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_organization_certificates_error403: {e}")) ;
            self
        }
        pub fn activate_organization_certificates_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_organization_certificates_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_organization_certificates_error404: {e}")) ;
            self
        }
        pub fn activate_organization_certificates_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_organization_certificates_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_organization_certificates_error429: {e}")) ;
            self
        }
        pub fn activate_organization_certificates_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_organization_certificates_error500 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_organization_certificates_error500: {e}")) ;
            self
        }
        pub fn activate_organization_certificates_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToggleCertificatesRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_organization_certificates_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_organization_certificates_request: {e}")) ;
            self
        }
        pub fn activate_organization_certificates_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationCertificateActivationResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_organization_certificates_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_organization_certificates_response: {e}")) ;
            self
        }
        pub fn activate_project_certificates_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_project_certificates_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_project_certificates_error400: {e}")) ;
            self
        }
        pub fn activate_project_certificates_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_project_certificates_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_project_certificates_error403: {e}")) ;
            self
        }
        pub fn activate_project_certificates_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_project_certificates_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_project_certificates_error404: {e}")) ;
            self
        }
        pub fn activate_project_certificates_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_project_certificates_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_project_certificates_error429: {e}")) ;
            self
        }
        pub fn activate_project_certificates_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_project_certificates_error500 = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_project_certificates_error500: {e}")) ;
            self
        }
        pub fn activate_project_certificates_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActivateProjectCertificatesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.activate_project_certificates_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for activate_project_certificates_params: {e}"
                )
            });
            self
        }
        pub fn activate_project_certificates_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToggleCertificatesRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_project_certificates_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_project_certificates_request: {e}")) ;
            self
        }
        pub fn activate_project_certificates_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationProjectCertificateActivationResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . activate_project_certificates_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for activate_project_certificates_response: {e}")) ;
            self
        }
        pub fn deactivate_organization_certificates_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_organization_certificates_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_organization_certificates_error400: {e}")) ;
            self
        }
        pub fn deactivate_organization_certificates_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_organization_certificates_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_organization_certificates_error403: {e}")) ;
            self
        }
        pub fn deactivate_organization_certificates_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_organization_certificates_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_organization_certificates_error404: {e}")) ;
            self
        }
        pub fn deactivate_organization_certificates_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_organization_certificates_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_organization_certificates_error429: {e}")) ;
            self
        }
        pub fn deactivate_organization_certificates_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_organization_certificates_error500 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_organization_certificates_error500: {e}")) ;
            self
        }
        pub fn deactivate_organization_certificates_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToggleCertificatesRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_organization_certificates_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_organization_certificates_request: {e}")) ;
            self
        }
        pub fn deactivate_organization_certificates_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationCertificateDeactivationResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_organization_certificates_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_organization_certificates_response: {e}")) ;
            self
        }
        pub fn deactivate_project_certificates_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_project_certificates_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_project_certificates_error400: {e}")) ;
            self
        }
        pub fn deactivate_project_certificates_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_project_certificates_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_project_certificates_error403: {e}")) ;
            self
        }
        pub fn deactivate_project_certificates_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_project_certificates_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_project_certificates_error404: {e}")) ;
            self
        }
        pub fn deactivate_project_certificates_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_project_certificates_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_project_certificates_error429: {e}")) ;
            self
        }
        pub fn deactivate_project_certificates_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_project_certificates_error500 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_project_certificates_error500: {e}")) ;
            self
        }
        pub fn deactivate_project_certificates_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeactivateProjectCertificatesParams>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_project_certificates_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_project_certificates_params: {e}")) ;
            self
        }
        pub fn deactivate_project_certificates_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToggleCertificatesRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_project_certificates_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_project_certificates_request: {e}")) ;
            self
        }
        pub fn deactivate_project_certificates_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationProjectCertificateDeactivationResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deactivate_project_certificates_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for deactivate_project_certificates_response: {e}")) ;
            self
        }
        pub fn delete_certificate_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_certificate_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_certificate_error400: {e}")
            });
            self
        }
        pub fn delete_certificate_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_certificate_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_certificate_error403: {e}")
            });
            self
        }
        pub fn delete_certificate_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_certificate_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_certificate_error404: {e}")
            });
            self
        }
        pub fn delete_certificate_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_certificate_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_certificate_error429: {e}")
            });
            self
        }
        pub fn delete_certificate_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteCertificateParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_certificate_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_certificate_params: {e}")
            });
            self
        }
        pub fn delete_certificate_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteCertificateResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_certificate_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_certificate_response: {e}")
            });
            self
        }
        pub fn get_certificate_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_certificate_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_certificate_error400: {e}")
            });
            self
        }
        pub fn get_certificate_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_certificate_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_certificate_error403: {e}")
            });
            self
        }
        pub fn get_certificate_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_certificate_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_certificate_error404: {e}")
            });
            self
        }
        pub fn get_certificate_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_certificate_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_certificate_error429: {e}")
            });
            self
        }
        pub fn get_certificate_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetCertificateParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_certificate_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_certificate_params: {e}")
            });
            self
        }
        pub fn get_certificate_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Certificate>,
            T::Error: ::std::fmt::Display,
        {
            self.get_certificate_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_certificate_response: {e}")
            });
            self
        }
        pub fn list_organization_certificates_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_certificates_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_certificates_error400: {e}")) ;
            self
        }
        pub fn list_organization_certificates_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_certificates_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_certificates_error403: {e}")) ;
            self
        }
        pub fn list_organization_certificates_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_certificates_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_certificates_error429: {e}")) ;
            self
        }
        pub fn list_organization_certificates_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListOrganizationCertificatesParams>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_certificates_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_certificates_params: {e}")) ;
            self
        }
        pub fn list_organization_certificates_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListCertificatesResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_certificates_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_certificates_response: {e}")) ;
            self
        }
        pub fn list_project_certificates_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_certificates_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_certificates_error400: {e}"
                )
            });
            self
        }
        pub fn list_project_certificates_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_certificates_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_certificates_error403: {e}"
                )
            });
            self
        }
        pub fn list_project_certificates_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_certificates_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_certificates_error429: {e}"
                )
            });
            self
        }
        pub fn list_project_certificates_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectCertificatesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_certificates_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_certificates_params: {e}")
            });
            self
        }
        pub fn list_project_certificates_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectCertificatesResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_certificates_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_certificates_response: {e}"
                )
            });
            self
        }
        pub fn modify_certificate_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_certificate_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_certificate_error400: {e}")
            });
            self
        }
        pub fn modify_certificate_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_certificate_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_certificate_error403: {e}")
            });
            self
        }
        pub fn modify_certificate_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_certificate_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_certificate_error404: {e}")
            });
            self
        }
        pub fn modify_certificate_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_certificate_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_certificate_error429: {e}")
            });
            self
        }
        pub fn modify_certificate_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_certificate_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_certificate_error500: {e}")
            });
            self
        }
        pub fn modify_certificate_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyCertificateParams>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_certificate_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_certificate_params: {e}")
            });
            self
        }
        pub fn modify_certificate_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyCertificateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_certificate_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_certificate_request: {e}")
            });
            self
        }
        pub fn modify_certificate_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Certificate>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_certificate_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_certificate_response: {e}")
            });
            self
        }
        pub fn upload_certificate_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.upload_certificate_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for upload_certificate_error400: {e}")
            });
            self
        }
        pub fn upload_certificate_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.upload_certificate_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for upload_certificate_error403: {e}")
            });
            self
        }
        pub fn upload_certificate_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.upload_certificate_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for upload_certificate_error429: {e}")
            });
            self
        }
        pub fn upload_certificate_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UploadCertificateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.upload_certificate_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for upload_certificate_request: {e}")
            });
            self
        }
        pub fn upload_certificate_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Certificate>,
            T::Error: ::std::fmt::Display,
        {
            self.upload_certificate_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for upload_certificate_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CertificatesApiSchema> for super::CertificatesApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CertificatesApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                activate_organization_certificates_error400: value
                    .activate_organization_certificates_error400?,
                activate_organization_certificates_error403: value
                    .activate_organization_certificates_error403?,
                activate_organization_certificates_error404: value
                    .activate_organization_certificates_error404?,
                activate_organization_certificates_error429: value
                    .activate_organization_certificates_error429?,
                activate_organization_certificates_error500: value
                    .activate_organization_certificates_error500?,
                activate_organization_certificates_request: value
                    .activate_organization_certificates_request?,
                activate_organization_certificates_response: value
                    .activate_organization_certificates_response?,
                activate_project_certificates_error400: value
                    .activate_project_certificates_error400?,
                activate_project_certificates_error403: value
                    .activate_project_certificates_error403?,
                activate_project_certificates_error404: value
                    .activate_project_certificates_error404?,
                activate_project_certificates_error429: value
                    .activate_project_certificates_error429?,
                activate_project_certificates_error500: value
                    .activate_project_certificates_error500?,
                activate_project_certificates_params: value.activate_project_certificates_params?,
                activate_project_certificates_request: value
                    .activate_project_certificates_request?,
                activate_project_certificates_response: value
                    .activate_project_certificates_response?,
                deactivate_organization_certificates_error400: value
                    .deactivate_organization_certificates_error400?,
                deactivate_organization_certificates_error403: value
                    .deactivate_organization_certificates_error403?,
                deactivate_organization_certificates_error404: value
                    .deactivate_organization_certificates_error404?,
                deactivate_organization_certificates_error429: value
                    .deactivate_organization_certificates_error429?,
                deactivate_organization_certificates_error500: value
                    .deactivate_organization_certificates_error500?,
                deactivate_organization_certificates_request: value
                    .deactivate_organization_certificates_request?,
                deactivate_organization_certificates_response: value
                    .deactivate_organization_certificates_response?,
                deactivate_project_certificates_error400: value
                    .deactivate_project_certificates_error400?,
                deactivate_project_certificates_error403: value
                    .deactivate_project_certificates_error403?,
                deactivate_project_certificates_error404: value
                    .deactivate_project_certificates_error404?,
                deactivate_project_certificates_error429: value
                    .deactivate_project_certificates_error429?,
                deactivate_project_certificates_error500: value
                    .deactivate_project_certificates_error500?,
                deactivate_project_certificates_params: value
                    .deactivate_project_certificates_params?,
                deactivate_project_certificates_request: value
                    .deactivate_project_certificates_request?,
                deactivate_project_certificates_response: value
                    .deactivate_project_certificates_response?,
                delete_certificate_error400: value.delete_certificate_error400?,
                delete_certificate_error403: value.delete_certificate_error403?,
                delete_certificate_error404: value.delete_certificate_error404?,
                delete_certificate_error429: value.delete_certificate_error429?,
                delete_certificate_params: value.delete_certificate_params?,
                delete_certificate_response: value.delete_certificate_response?,
                get_certificate_error400: value.get_certificate_error400?,
                get_certificate_error403: value.get_certificate_error403?,
                get_certificate_error404: value.get_certificate_error404?,
                get_certificate_error429: value.get_certificate_error429?,
                get_certificate_params: value.get_certificate_params?,
                get_certificate_response: value.get_certificate_response?,
                list_organization_certificates_error400: value
                    .list_organization_certificates_error400?,
                list_organization_certificates_error403: value
                    .list_organization_certificates_error403?,
                list_organization_certificates_error429: value
                    .list_organization_certificates_error429?,
                list_organization_certificates_params: value
                    .list_organization_certificates_params?,
                list_organization_certificates_response: value
                    .list_organization_certificates_response?,
                list_project_certificates_error400: value.list_project_certificates_error400?,
                list_project_certificates_error403: value.list_project_certificates_error403?,
                list_project_certificates_error429: value.list_project_certificates_error429?,
                list_project_certificates_params: value.list_project_certificates_params?,
                list_project_certificates_response: value.list_project_certificates_response?,
                modify_certificate_error400: value.modify_certificate_error400?,
                modify_certificate_error403: value.modify_certificate_error403?,
                modify_certificate_error404: value.modify_certificate_error404?,
                modify_certificate_error429: value.modify_certificate_error429?,
                modify_certificate_error500: value.modify_certificate_error500?,
                modify_certificate_params: value.modify_certificate_params?,
                modify_certificate_request: value.modify_certificate_request?,
                modify_certificate_response: value.modify_certificate_response?,
                upload_certificate_error400: value.upload_certificate_error400?,
                upload_certificate_error403: value.upload_certificate_error403?,
                upload_certificate_error429: value.upload_certificate_error429?,
                upload_certificate_request: value.upload_certificate_request?,
                upload_certificate_response: value.upload_certificate_response?,
            })
        }
    }
    impl ::std::convert::From<super::CertificatesApiSchema> for CertificatesApiSchema {
        fn from(value: super::CertificatesApiSchema) -> Self {
            Self {
                activate_organization_certificates_error400: Ok(
                    value.activate_organization_certificates_error400
                ),
                activate_organization_certificates_error403: Ok(
                    value.activate_organization_certificates_error403
                ),
                activate_organization_certificates_error404: Ok(
                    value.activate_organization_certificates_error404
                ),
                activate_organization_certificates_error429: Ok(
                    value.activate_organization_certificates_error429
                ),
                activate_organization_certificates_error500: Ok(
                    value.activate_organization_certificates_error500
                ),
                activate_organization_certificates_request: Ok(
                    value.activate_organization_certificates_request
                ),
                activate_organization_certificates_response: Ok(
                    value.activate_organization_certificates_response
                ),
                activate_project_certificates_error400: Ok(
                    value.activate_project_certificates_error400
                ),
                activate_project_certificates_error403: Ok(
                    value.activate_project_certificates_error403
                ),
                activate_project_certificates_error404: Ok(
                    value.activate_project_certificates_error404
                ),
                activate_project_certificates_error429: Ok(
                    value.activate_project_certificates_error429
                ),
                activate_project_certificates_error500: Ok(
                    value.activate_project_certificates_error500
                ),
                activate_project_certificates_params: Ok(value.activate_project_certificates_params),
                activate_project_certificates_request: Ok(
                    value.activate_project_certificates_request
                ),
                activate_project_certificates_response: Ok(
                    value.activate_project_certificates_response
                ),
                deactivate_organization_certificates_error400: Ok(
                    value.deactivate_organization_certificates_error400
                ),
                deactivate_organization_certificates_error403: Ok(
                    value.deactivate_organization_certificates_error403
                ),
                deactivate_organization_certificates_error404: Ok(
                    value.deactivate_organization_certificates_error404
                ),
                deactivate_organization_certificates_error429: Ok(
                    value.deactivate_organization_certificates_error429
                ),
                deactivate_organization_certificates_error500: Ok(
                    value.deactivate_organization_certificates_error500
                ),
                deactivate_organization_certificates_request: Ok(
                    value.deactivate_organization_certificates_request
                ),
                deactivate_organization_certificates_response: Ok(
                    value.deactivate_organization_certificates_response
                ),
                deactivate_project_certificates_error400: Ok(
                    value.deactivate_project_certificates_error400
                ),
                deactivate_project_certificates_error403: Ok(
                    value.deactivate_project_certificates_error403
                ),
                deactivate_project_certificates_error404: Ok(
                    value.deactivate_project_certificates_error404
                ),
                deactivate_project_certificates_error429: Ok(
                    value.deactivate_project_certificates_error429
                ),
                deactivate_project_certificates_error500: Ok(
                    value.deactivate_project_certificates_error500
                ),
                deactivate_project_certificates_params: Ok(
                    value.deactivate_project_certificates_params
                ),
                deactivate_project_certificates_request: Ok(
                    value.deactivate_project_certificates_request
                ),
                deactivate_project_certificates_response: Ok(
                    value.deactivate_project_certificates_response
                ),
                delete_certificate_error400: Ok(value.delete_certificate_error400),
                delete_certificate_error403: Ok(value.delete_certificate_error403),
                delete_certificate_error404: Ok(value.delete_certificate_error404),
                delete_certificate_error429: Ok(value.delete_certificate_error429),
                delete_certificate_params: Ok(value.delete_certificate_params),
                delete_certificate_response: Ok(value.delete_certificate_response),
                get_certificate_error400: Ok(value.get_certificate_error400),
                get_certificate_error403: Ok(value.get_certificate_error403),
                get_certificate_error404: Ok(value.get_certificate_error404),
                get_certificate_error429: Ok(value.get_certificate_error429),
                get_certificate_params: Ok(value.get_certificate_params),
                get_certificate_response: Ok(value.get_certificate_response),
                list_organization_certificates_error400: Ok(
                    value.list_organization_certificates_error400
                ),
                list_organization_certificates_error403: Ok(
                    value.list_organization_certificates_error403
                ),
                list_organization_certificates_error429: Ok(
                    value.list_organization_certificates_error429
                ),
                list_organization_certificates_params: Ok(
                    value.list_organization_certificates_params
                ),
                list_organization_certificates_response: Ok(
                    value.list_organization_certificates_response
                ),
                list_project_certificates_error400: Ok(value.list_project_certificates_error400),
                list_project_certificates_error403: Ok(value.list_project_certificates_error403),
                list_project_certificates_error429: Ok(value.list_project_certificates_error429),
                list_project_certificates_params: Ok(value.list_project_certificates_params),
                list_project_certificates_response: Ok(value.list_project_certificates_response),
                modify_certificate_error400: Ok(value.modify_certificate_error400),
                modify_certificate_error403: Ok(value.modify_certificate_error403),
                modify_certificate_error404: Ok(value.modify_certificate_error404),
                modify_certificate_error429: Ok(value.modify_certificate_error429),
                modify_certificate_error500: Ok(value.modify_certificate_error500),
                modify_certificate_params: Ok(value.modify_certificate_params),
                modify_certificate_request: Ok(value.modify_certificate_request),
                modify_certificate_response: Ok(value.modify_certificate_response),
                upload_certificate_error400: Ok(value.upload_certificate_error400),
                upload_certificate_error403: Ok(value.upload_certificate_error403),
                upload_certificate_error429: Ok(value.upload_certificate_error429),
                upload_certificate_request: Ok(value.upload_certificate_request),
                upload_certificate_response: Ok(value.upload_certificate_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeactivateProjectCertificatesParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeactivateProjectCertificatesParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl DeactivateProjectCertificatesParams {
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
    impl ::std::convert::TryFrom<DeactivateProjectCertificatesParams>
        for super::DeactivateProjectCertificatesParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeactivateProjectCertificatesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeactivateProjectCertificatesParams>
        for DeactivateProjectCertificatesParams
    {
        fn from(value: super::DeactivateProjectCertificatesParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteCertificateParams {
        certificate_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteCertificateParams {
        fn default() -> Self {
            Self {
                certificate_id: Err("no value supplied for certificate_id".to_string()),
            }
        }
    }
    impl DeleteCertificateParams {
        pub fn certificate_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for certificate_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteCertificateParams> for super::DeleteCertificateParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteCertificateParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                certificate_id: value.certificate_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteCertificateParams> for DeleteCertificateParams {
        fn from(value: super::DeleteCertificateParams) -> Self {
            Self {
                certificate_id: Ok(value.certificate_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteCertificateResponse {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::DeleteCertificateResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteCertificateResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteCertificateResponse {
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
            T: ::std::convert::TryInto<super::DeleteCertificateResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteCertificateResponse> for super::DeleteCertificateResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteCertificateResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteCertificateResponse> for DeleteCertificateResponse {
        fn from(value: super::DeleteCertificateResponse) -> Self {
            Self {
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetCertificateParams {
        certificate_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        include: ::std::result::Result<
            ::std::vec::Vec<super::GetCertificateParamsIncludeItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetCertificateParams {
        fn default() -> Self {
            Self {
                certificate_id: Err("no value supplied for certificate_id".to_string()),
                include: Ok(Default::default()),
            }
        }
    }
    impl GetCertificateParams {
        pub fn certificate_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for certificate_id: {e}"));
            self
        }
        pub fn include<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::GetCertificateParamsIncludeItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.include = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetCertificateParams> for super::GetCertificateParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetCertificateParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                certificate_id: value.certificate_id?,
                include: value.include?,
            })
        }
    }
    impl ::std::convert::From<super::GetCertificateParams> for GetCertificateParams {
        fn from(value: super::GetCertificateParams) -> Self {
            Self {
                certificate_id: Ok(value.certificate_id),
                include: Ok(value.include),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListCertificatesResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::OrganizationCertificate>,
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
        object: ::std::result::Result<super::ListCertificatesResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for ListCertificatesResponse {
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
    impl ListCertificatesResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrganizationCertificate>>,
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
            T: ::std::convert::TryInto<super::ListCertificatesResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListCertificatesResponse> for super::ListCertificatesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListCertificatesResponse,
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
    impl ::std::convert::From<super::ListCertificatesResponse> for ListCertificatesResponse {
        fn from(value: super::ListCertificatesResponse) -> Self {
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
    pub struct ListOrganizationCertificatesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListOrganizationCertificatesParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListOrganizationCertificatesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListOrganizationCertificatesParams {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListOrganizationCertificatesParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListOrganizationCertificatesParams>
        for super::ListOrganizationCertificatesParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListOrganizationCertificatesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListOrganizationCertificatesParams>
        for ListOrganizationCertificatesParams
    {
        fn from(value: super::ListOrganizationCertificatesParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectCertificatesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListProjectCertificatesParamsOrder>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectCertificatesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ListProjectCertificatesParams {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListProjectCertificatesParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
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
    impl ::std::convert::TryFrom<ListProjectCertificatesParams>
        for super::ListProjectCertificatesParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectCertificatesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectCertificatesParams> for ListProjectCertificatesParams {
        fn from(value: super::ListProjectCertificatesParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectCertificatesResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::OrganizationProjectCertificate>,
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
        object: ::std::result::Result<
            super::ListProjectCertificatesResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListProjectCertificatesResponse {
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
    impl ListProjectCertificatesResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrganizationProjectCertificate>>,
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
            T: ::std::convert::TryInto<super::ListProjectCertificatesResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListProjectCertificatesResponse>
        for super::ListProjectCertificatesResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectCertificatesResponse,
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
    impl ::std::convert::From<super::ListProjectCertificatesResponse>
        for ListProjectCertificatesResponse
    {
        fn from(value: super::ListProjectCertificatesResponse) -> Self {
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
    pub struct ModifyCertificateParams {
        certificate_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyCertificateParams {
        fn default() -> Self {
            Self {
                certificate_id: Err("no value supplied for certificate_id".to_string()),
            }
        }
    }
    impl ModifyCertificateParams {
        pub fn certificate_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for certificate_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyCertificateParams> for super::ModifyCertificateParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyCertificateParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                certificate_id: value.certificate_id?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyCertificateParams> for ModifyCertificateParams {
        fn from(value: super::ModifyCertificateParams) -> Self {
            Self {
                certificate_id: Ok(value.certificate_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyCertificateRequest {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModifyCertificateRequest {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
            }
        }
    }
    impl ModifyCertificateRequest {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyCertificateRequest> for super::ModifyCertificateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyCertificateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::ModifyCertificateRequest> for ModifyCertificateRequest {
        fn from(value: super::ModifyCertificateRequest) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationCertificate {
        active: ::std::result::Result<bool, ::std::string::String>,
        certificate_details: ::std::result::Result<
            super::OrganizationCertificateCertificateDetails,
            ::std::string::String,
        >,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::OrganizationCertificateObject, ::std::string::String>,
    }
    impl ::std::default::Default for OrganizationCertificate {
        fn default() -> Self {
            Self {
                active: Err("no value supplied for active".to_string()),
                certificate_details: Err("no value supplied for certificate_details".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl OrganizationCertificate {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active: {e}"));
            self
        }
        pub fn certificate_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationCertificateCertificateDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for certificate_details: {e}")
            });
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationCertificateObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationCertificate> for super::OrganizationCertificate {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationCertificate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                certificate_details: value.certificate_details?,
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationCertificate> for OrganizationCertificate {
        fn from(value: super::OrganizationCertificate) -> Self {
            Self {
                active: Ok(value.active),
                certificate_details: Ok(value.certificate_details),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationCertificateActivationResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::OrganizationCertificate>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            super::OrganizationCertificateActivationResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationCertificateActivationResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl OrganizationCertificateActivationResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrganizationCertificate>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationCertificateActivationResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationCertificateActivationResponse>
        for super::OrganizationCertificateActivationResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationCertificateActivationResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationCertificateActivationResponse>
        for OrganizationCertificateActivationResponse
    {
        fn from(value: super::OrganizationCertificateActivationResponse) -> Self {
            Self {
                data: Ok(value.data),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationCertificateCertificateDetails {
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        valid_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for OrganizationCertificateCertificateDetails {
        fn default() -> Self {
            Self {
                expires_at: Ok(Default::default()),
                valid_at: Ok(Default::default()),
            }
        }
    }
    impl OrganizationCertificateCertificateDetails {
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
        pub fn valid_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_at: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationCertificateCertificateDetails>
        for super::OrganizationCertificateCertificateDetails
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationCertificateCertificateDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_at: value.expires_at?,
                valid_at: value.valid_at?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationCertificateCertificateDetails>
        for OrganizationCertificateCertificateDetails
    {
        fn from(value: super::OrganizationCertificateCertificateDetails) -> Self {
            Self {
                expires_at: Ok(value.expires_at),
                valid_at: Ok(value.valid_at),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationCertificateDeactivationResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::OrganizationCertificate>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            super::OrganizationCertificateDeactivationResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationCertificateDeactivationResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl OrganizationCertificateDeactivationResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrganizationCertificate>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationCertificateDeactivationResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationCertificateDeactivationResponse>
        for super::OrganizationCertificateDeactivationResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationCertificateDeactivationResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationCertificateDeactivationResponse>
        for OrganizationCertificateDeactivationResponse
    {
        fn from(value: super::OrganizationCertificateDeactivationResponse) -> Self {
            Self {
                data: Ok(value.data),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationProjectCertificate {
        active: ::std::result::Result<bool, ::std::string::String>,
        certificate_details: ::std::result::Result<
            super::OrganizationProjectCertificateCertificateDetails,
            ::std::string::String,
        >,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            super::OrganizationProjectCertificateObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationProjectCertificate {
        fn default() -> Self {
            Self {
                active: Err("no value supplied for active".to_string()),
                certificate_details: Err("no value supplied for certificate_details".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl OrganizationProjectCertificate {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active: {e}"));
            self
        }
        pub fn certificate_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationProjectCertificateCertificateDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for certificate_details: {e}")
            });
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationProjectCertificateObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationProjectCertificate>
        for super::OrganizationProjectCertificate
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationProjectCertificate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                certificate_details: value.certificate_details?,
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationProjectCertificate>
        for OrganizationProjectCertificate
    {
        fn from(value: super::OrganizationProjectCertificate) -> Self {
            Self {
                active: Ok(value.active),
                certificate_details: Ok(value.certificate_details),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationProjectCertificateActivationResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::OrganizationProjectCertificate>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            super::OrganizationProjectCertificateActivationResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationProjectCertificateActivationResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl OrganizationProjectCertificateActivationResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrganizationProjectCertificate>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::OrganizationProjectCertificateActivationResponseObject,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationProjectCertificateActivationResponse>
        for super::OrganizationProjectCertificateActivationResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationProjectCertificateActivationResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationProjectCertificateActivationResponse>
        for OrganizationProjectCertificateActivationResponse
    {
        fn from(value: super::OrganizationProjectCertificateActivationResponse) -> Self {
            Self {
                data: Ok(value.data),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationProjectCertificateCertificateDetails {
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        valid_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for OrganizationProjectCertificateCertificateDetails {
        fn default() -> Self {
            Self {
                expires_at: Ok(Default::default()),
                valid_at: Ok(Default::default()),
            }
        }
    }
    impl OrganizationProjectCertificateCertificateDetails {
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
        pub fn valid_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_at: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationProjectCertificateCertificateDetails>
        for super::OrganizationProjectCertificateCertificateDetails
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationProjectCertificateCertificateDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_at: value.expires_at?,
                valid_at: value.valid_at?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationProjectCertificateCertificateDetails>
        for OrganizationProjectCertificateCertificateDetails
    {
        fn from(value: super::OrganizationProjectCertificateCertificateDetails) -> Self {
            Self {
                expires_at: Ok(value.expires_at),
                valid_at: Ok(value.valid_at),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationProjectCertificateDeactivationResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::OrganizationProjectCertificate>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            super::OrganizationProjectCertificateDeactivationResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationProjectCertificateDeactivationResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl OrganizationProjectCertificateDeactivationResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrganizationProjectCertificate>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::OrganizationProjectCertificateDeactivationResponseObject,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationProjectCertificateDeactivationResponse>
        for super::OrganizationProjectCertificateDeactivationResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationProjectCertificateDeactivationResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationProjectCertificateDeactivationResponse>
        for OrganizationProjectCertificateDeactivationResponse
    {
        fn from(value: super::OrganizationProjectCertificateDeactivationResponse) -> Self {
            Self {
                data: Ok(value.data),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToggleCertificatesRequest {
        certificate_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ToggleCertificatesRequest {
        fn default() -> Self {
            Self {
                certificate_ids: Err("no value supplied for certificate_ids".to_string()),
            }
        }
    }
    impl ToggleCertificatesRequest {
        pub fn certificate_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for certificate_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToggleCertificatesRequest> for super::ToggleCertificatesRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToggleCertificatesRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                certificate_ids: value.certificate_ids?,
            })
        }
    }
    impl ::std::convert::From<super::ToggleCertificatesRequest> for ToggleCertificatesRequest {
        fn from(value: super::ToggleCertificatesRequest) -> Self {
            Self {
                certificate_ids: Ok(value.certificate_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UploadCertificateRequest {
        certificate: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UploadCertificateRequest {
        fn default() -> Self {
            Self {
                certificate: Err("no value supplied for certificate".to_string()),
                name: Ok(Default::default()),
            }
        }
    }
    impl UploadCertificateRequest {
        pub fn certificate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for certificate: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UploadCertificateRequest> for super::UploadCertificateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UploadCertificateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                certificate: value.certificate?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::UploadCertificateRequest> for UploadCertificateRequest {
        fn from(value: super::UploadCertificateRequest) -> Self {
            Self {
                certificate: Ok(value.certificate),
                name: Ok(value.name),
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
