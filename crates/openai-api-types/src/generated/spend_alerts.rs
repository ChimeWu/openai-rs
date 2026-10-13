pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`CreateProjectSpendAlertParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateProjectSpendAlertParams {
    pub project_id: ::std::string::String,
}
impl CreateProjectSpendAlertParams {
    pub fn builder() -> builder::CreateProjectSpendAlertParams {
        Default::default()
    }
}
#[doc = "Parameters for creating or updating a spend alert."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateSpendAlertBody {
    #[doc = "The currency for the threshold amount."]
    pub currency: CreateSpendAlertBodyCurrency,
    #[doc = "The time interval for evaluating spend against the threshold."]
    pub interval: CreateSpendAlertBodyInterval,
    pub notification_channel: SpendAlertNotificationChannel,
    #[doc = "The alert threshold amount, in cents."]
    pub threshold_amount: u64,
}
impl CreateSpendAlertBody {
    pub fn builder() -> builder::CreateSpendAlertBody {
        Default::default()
    }
}
#[doc = "The currency for the threshold amount."]
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
pub enum CreateSpendAlertBodyCurrency {
    #[serde(rename = "USD")]
    Usd,
}
impl ::std::fmt::Display for CreateSpendAlertBodyCurrency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Usd => f.write_str("USD"),
        }
    }
}
impl ::std::str::FromStr for CreateSpendAlertBodyCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "USD" => Ok(Self::Usd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateSpendAlertBodyCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateSpendAlertBodyCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The time interval for evaluating spend against the threshold."]
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
pub enum CreateSpendAlertBodyInterval {
    #[serde(rename = "month")]
    Month,
}
impl ::std::fmt::Display for CreateSpendAlertBodyInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Month => f.write_str("month"),
        }
    }
}
impl ::std::str::FromStr for CreateSpendAlertBodyInterval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "month" => Ok(Self::Month),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateSpendAlertBodyInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateSpendAlertBodyInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeleteOrganizationSpendAlertParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteOrganizationSpendAlertParams {
    pub alert_id: ::std::string::String,
}
impl DeleteOrganizationSpendAlertParams {
    pub fn builder() -> builder::DeleteOrganizationSpendAlertParams {
        Default::default()
    }
}
#[doc = "`DeleteProjectSpendAlertParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteProjectSpendAlertParams {
    pub alert_id: ::std::string::String,
    pub project_id: ::std::string::String,
}
impl DeleteProjectSpendAlertParams {
    pub fn builder() -> builder::DeleteProjectSpendAlertParams {
        Default::default()
    }
}
#[doc = "`ListOrganizationSpendAlertsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListOrganizationSpendAlertsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListOrganizationSpendAlertsParamsOrder>,
}
impl ListOrganizationSpendAlertsParams {
    pub fn builder() -> builder::ListOrganizationSpendAlertsParams {
        Default::default()
    }
}
#[doc = "`ListOrganizationSpendAlertsParamsOrder`"]
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
pub enum ListOrganizationSpendAlertsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListOrganizationSpendAlertsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListOrganizationSpendAlertsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListOrganizationSpendAlertsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListOrganizationSpendAlertsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListProjectSpendAlertsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectSpendAlertsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListProjectSpendAlertsParamsOrder>,
    pub project_id: ::std::string::String,
}
impl ListProjectSpendAlertsParams {
    pub fn builder() -> builder::ListProjectSpendAlertsParams {
        Default::default()
    }
}
#[doc = "`ListProjectSpendAlertsParamsOrder`"]
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
pub enum ListProjectSpendAlertsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListProjectSpendAlertsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListProjectSpendAlertsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListProjectSpendAlertsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListProjectSpendAlertsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a spend alert configured at the organization level."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationSpendAlert {
    #[doc = "The currency for the threshold amount."]
    pub currency: OrganizationSpendAlertCurrency,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The time interval for evaluating spend against the threshold."]
    pub interval: OrganizationSpendAlertInterval,
    pub notification_channel: SpendAlertNotificationChannel,
    #[doc = "The object type, which is always `organization.spend_alert`."]
    pub object: OrganizationSpendAlertObject,
    #[doc = "The alert threshold amount, in cents."]
    pub threshold_amount: i64,
}
impl OrganizationSpendAlert {
    pub fn builder() -> builder::OrganizationSpendAlert {
        Default::default()
    }
}
#[doc = "The currency for the threshold amount."]
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
pub enum OrganizationSpendAlertCurrency {
    #[serde(rename = "USD")]
    Usd,
}
impl ::std::fmt::Display for OrganizationSpendAlertCurrency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Usd => f.write_str("USD"),
        }
    }
}
impl ::std::str::FromStr for OrganizationSpendAlertCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "USD" => Ok(Self::Usd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationSpendAlertCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationSpendAlertCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Confirmation payload returned after deleting an organization spend alert."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationSpendAlertDeletedResource {
    #[doc = "Whether the spend alert was deleted."]
    pub deleted: bool,
    #[doc = "The deleted spend alert ID."]
    pub id: ::std::string::String,
    #[doc = "Always `organization.spend_alert.deleted`."]
    pub object: OrganizationSpendAlertDeletedResourceObject,
}
impl OrganizationSpendAlertDeletedResource {
    pub fn builder() -> builder::OrganizationSpendAlertDeletedResource {
        Default::default()
    }
}
#[doc = "Always `organization.spend_alert.deleted`."]
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
pub enum OrganizationSpendAlertDeletedResourceObject {
    #[serde(rename = "organization.spend_alert.deleted")]
    OrganizationSpendAlertDeleted,
}
impl ::std::fmt::Display for OrganizationSpendAlertDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationSpendAlertDeleted => f.write_str("organization.spend_alert.deleted"),
        }
    }
}
impl ::std::str::FromStr for OrganizationSpendAlertDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.spend_alert.deleted" => Ok(Self::OrganizationSpendAlertDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationSpendAlertDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for OrganizationSpendAlertDeletedResourceObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The time interval for evaluating spend against the threshold."]
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
pub enum OrganizationSpendAlertInterval {
    #[serde(rename = "month")]
    Month,
}
impl ::std::fmt::Display for OrganizationSpendAlertInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Month => f.write_str("month"),
        }
    }
}
impl ::std::str::FromStr for OrganizationSpendAlertInterval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "month" => Ok(Self::Month),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationSpendAlertInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationSpendAlertInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Paginated list of organization spend alerts."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationSpendAlertListResource {
    #[doc = "Spend alerts returned in the current page."]
    pub data: ::std::vec::Vec<OrganizationSpendAlert>,
    #[doc = "The ID of the first spend alert in this page."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether more spend alerts are available when paginating."]
    pub has_more: bool,
    #[doc = "The ID of the last spend alert in this page."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "Always `list`."]
    pub object: OrganizationSpendAlertListResourceObject,
}
impl OrganizationSpendAlertListResource {
    pub fn builder() -> builder::OrganizationSpendAlertListResource {
        Default::default()
    }
}
#[doc = "Always `list`."]
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
pub enum OrganizationSpendAlertListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for OrganizationSpendAlertListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for OrganizationSpendAlertListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationSpendAlertListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationSpendAlertListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `organization.spend_alert`."]
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
pub enum OrganizationSpendAlertObject {
    #[serde(rename = "organization.spend_alert")]
    OrganizationSpendAlert,
}
impl ::std::fmt::Display for OrganizationSpendAlertObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationSpendAlert => f.write_str("organization.spend_alert"),
        }
    }
}
impl ::std::str::FromStr for OrganizationSpendAlertObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.spend_alert" => Ok(Self::OrganizationSpendAlert),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationSpendAlertObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationSpendAlertObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a spend alert configured at the project level."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectSpendAlert {
    #[doc = "The currency for the threshold amount."]
    pub currency: ProjectSpendAlertCurrency,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The time interval for evaluating spend against the threshold."]
    pub interval: ProjectSpendAlertInterval,
    pub notification_channel: SpendAlertNotificationChannel,
    #[doc = "The object type, which is always `project.spend_alert`."]
    pub object: ProjectSpendAlertObject,
    #[doc = "The alert threshold amount, in cents."]
    pub threshold_amount: i64,
}
impl ProjectSpendAlert {
    pub fn builder() -> builder::ProjectSpendAlert {
        Default::default()
    }
}
#[doc = "The currency for the threshold amount."]
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
pub enum ProjectSpendAlertCurrency {
    #[serde(rename = "USD")]
    Usd,
}
impl ::std::fmt::Display for ProjectSpendAlertCurrency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Usd => f.write_str("USD"),
        }
    }
}
impl ::std::str::FromStr for ProjectSpendAlertCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "USD" => Ok(Self::Usd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectSpendAlertCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectSpendAlertCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Confirmation payload returned after deleting a project spend alert."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectSpendAlertDeletedResource {
    #[doc = "Whether the spend alert was deleted."]
    pub deleted: bool,
    #[doc = "The deleted spend alert ID."]
    pub id: ::std::string::String,
    #[doc = "Always `project.spend_alert.deleted`."]
    pub object: ProjectSpendAlertDeletedResourceObject,
}
impl ProjectSpendAlertDeletedResource {
    pub fn builder() -> builder::ProjectSpendAlertDeletedResource {
        Default::default()
    }
}
#[doc = "Always `project.spend_alert.deleted`."]
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
pub enum ProjectSpendAlertDeletedResourceObject {
    #[serde(rename = "project.spend_alert.deleted")]
    ProjectSpendAlertDeleted,
}
impl ::std::fmt::Display for ProjectSpendAlertDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectSpendAlertDeleted => f.write_str("project.spend_alert.deleted"),
        }
    }
}
impl ::std::str::FromStr for ProjectSpendAlertDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.spend_alert.deleted" => Ok(Self::ProjectSpendAlertDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectSpendAlertDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectSpendAlertDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The time interval for evaluating spend against the threshold."]
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
pub enum ProjectSpendAlertInterval {
    #[serde(rename = "month")]
    Month,
}
impl ::std::fmt::Display for ProjectSpendAlertInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Month => f.write_str("month"),
        }
    }
}
impl ::std::str::FromStr for ProjectSpendAlertInterval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "month" => Ok(Self::Month),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectSpendAlertInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectSpendAlertInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Paginated list of project spend alerts."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectSpendAlertListResource {
    #[doc = "Spend alerts returned in the current page."]
    pub data: ::std::vec::Vec<ProjectSpendAlert>,
    #[doc = "The ID of the first spend alert in this page."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether more spend alerts are available when paginating."]
    pub has_more: bool,
    #[doc = "The ID of the last spend alert in this page."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "Always `list`."]
    pub object: ProjectSpendAlertListResourceObject,
}
impl ProjectSpendAlertListResource {
    pub fn builder() -> builder::ProjectSpendAlertListResource {
        Default::default()
    }
}
#[doc = "Always `list`."]
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
pub enum ProjectSpendAlertListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ProjectSpendAlertListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ProjectSpendAlertListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectSpendAlertListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectSpendAlertListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `project.spend_alert`."]
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
pub enum ProjectSpendAlertObject {
    #[serde(rename = "project.spend_alert")]
    ProjectSpendAlert,
}
impl ::std::fmt::Display for ProjectSpendAlertObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectSpendAlert => f.write_str("project.spend_alert"),
        }
    }
}
impl ::std::str::FromStr for ProjectSpendAlertObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.spend_alert" => Ok(Self::ProjectSpendAlert),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectSpendAlertObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectSpendAlertObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RetrieveOrganizationSpendAlertParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveOrganizationSpendAlertParams {
    pub alert_id: ::std::string::String,
}
impl RetrieveOrganizationSpendAlertParams {
    pub fn builder() -> builder::RetrieveOrganizationSpendAlertParams {
        Default::default()
    }
}
#[doc = "`RetrieveProjectSpendAlertParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectSpendAlertParams {
    pub alert_id: ::std::string::String,
    pub project_id: ::std::string::String,
}
impl RetrieveProjectSpendAlertParams {
    pub fn builder() -> builder::RetrieveProjectSpendAlertParams {
        Default::default()
    }
}
#[doc = "Email notification settings for a spend alert."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SpendAlertNotificationChannel {
    #[doc = "Email addresses that receive the spend alert notification."]
    pub recipients: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional subject prefix for alert emails."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subject_prefix: ::std::option::Option<::std::string::String>,
    #[doc = "The notification channel type. Currently only `email` is supported."]
    #[serde(rename = "type")]
    pub type_: SpendAlertNotificationChannelType,
}
impl SpendAlertNotificationChannel {
    pub fn builder() -> builder::SpendAlertNotificationChannel {
        Default::default()
    }
}
#[doc = "The notification channel type. Currently only `email` is supported."]
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
pub enum SpendAlertNotificationChannelType {
    #[serde(rename = "email")]
    Email,
}
impl ::std::fmt::Display for SpendAlertNotificationChannelType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Email => f.write_str("email"),
        }
    }
}
impl ::std::str::FromStr for SpendAlertNotificationChannelType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "email" => Ok(Self::Email),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SpendAlertNotificationChannelType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpendAlertNotificationChannelType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SpendAlertsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SpendAlertsApiSchema {
    #[serde(rename = "CreateOrganizationSpendAlertError400")]
    pub create_organization_spend_alert_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateOrganizationSpendAlertError403")]
    pub create_organization_spend_alert_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "CreateOrganizationSpendAlertRequest")]
    pub create_organization_spend_alert_request: CreateSpendAlertBody,
    #[serde(rename = "CreateOrganizationSpendAlertResponse")]
    pub create_organization_spend_alert_response: OrganizationSpendAlert,
    #[serde(rename = "CreateProjectSpendAlertError400")]
    pub create_project_spend_alert_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectSpendAlertError403")]
    pub create_project_spend_alert_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "CreateProjectSpendAlertError404")]
    pub create_project_spend_alert_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectSpendAlertParams")]
    pub create_project_spend_alert_params: CreateProjectSpendAlertParams,
    #[serde(rename = "CreateProjectSpendAlertRequest")]
    pub create_project_spend_alert_request: CreateSpendAlertBody,
    #[serde(rename = "CreateProjectSpendAlertResponse")]
    pub create_project_spend_alert_response: ProjectSpendAlert,
    #[serde(rename = "DeleteOrganizationSpendAlertError403")]
    pub delete_organization_spend_alert_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteOrganizationSpendAlertError404")]
    pub delete_organization_spend_alert_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteOrganizationSpendAlertParams")]
    pub delete_organization_spend_alert_params: DeleteOrganizationSpendAlertParams,
    #[serde(rename = "DeleteOrganizationSpendAlertResponse")]
    pub delete_organization_spend_alert_response: OrganizationSpendAlertDeletedResource,
    #[serde(rename = "DeleteProjectSpendAlertError400")]
    pub delete_project_spend_alert_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectSpendAlertError403")]
    pub delete_project_spend_alert_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteProjectSpendAlertError404")]
    pub delete_project_spend_alert_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectSpendAlertParams")]
    pub delete_project_spend_alert_params: DeleteProjectSpendAlertParams,
    #[serde(rename = "DeleteProjectSpendAlertResponse")]
    pub delete_project_spend_alert_response: ProjectSpendAlertDeletedResource,
    #[serde(rename = "ListOrganizationSpendAlertsError400")]
    pub list_organization_spend_alerts_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListOrganizationSpendAlertsError403")]
    pub list_organization_spend_alerts_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListOrganizationSpendAlertsError404")]
    pub list_organization_spend_alerts_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListOrganizationSpendAlertsParams")]
    pub list_organization_spend_alerts_params: ListOrganizationSpendAlertsParams,
    #[serde(rename = "ListOrganizationSpendAlertsResponse")]
    pub list_organization_spend_alerts_response: OrganizationSpendAlertListResource,
    #[serde(rename = "ListProjectSpendAlertsError400")]
    pub list_project_spend_alerts_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectSpendAlertsError403")]
    pub list_project_spend_alerts_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectSpendAlertsError404")]
    pub list_project_spend_alerts_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectSpendAlertsParams")]
    pub list_project_spend_alerts_params: ListProjectSpendAlertsParams,
    #[serde(rename = "ListProjectSpendAlertsResponse")]
    pub list_project_spend_alerts_response: ProjectSpendAlertListResource,
    #[serde(rename = "RetrieveOrganizationSpendAlertError403")]
    pub retrieve_organization_spend_alert_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveOrganizationSpendAlertError404")]
    pub retrieve_organization_spend_alert_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveOrganizationSpendAlertParams")]
    pub retrieve_organization_spend_alert_params: RetrieveOrganizationSpendAlertParams,
    #[serde(rename = "RetrieveOrganizationSpendAlertResponse")]
    pub retrieve_organization_spend_alert_response: OrganizationSpendAlert,
    #[serde(rename = "RetrieveProjectSpendAlertError400")]
    pub retrieve_project_spend_alert_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectSpendAlertError403")]
    pub retrieve_project_spend_alert_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectSpendAlertError404")]
    pub retrieve_project_spend_alert_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectSpendAlertParams")]
    pub retrieve_project_spend_alert_params: RetrieveProjectSpendAlertParams,
    #[serde(rename = "RetrieveProjectSpendAlertResponse")]
    pub retrieve_project_spend_alert_response: ProjectSpendAlert,
    #[serde(rename = "UpdateOrganizationSpendAlertError400")]
    pub update_organization_spend_alert_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateOrganizationSpendAlertError403")]
    pub update_organization_spend_alert_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateOrganizationSpendAlertError404")]
    pub update_organization_spend_alert_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateOrganizationSpendAlertParams")]
    pub update_organization_spend_alert_params: UpdateOrganizationSpendAlertParams,
    #[serde(rename = "UpdateOrganizationSpendAlertRequest")]
    pub update_organization_spend_alert_request: CreateSpendAlertBody,
    #[serde(rename = "UpdateOrganizationSpendAlertResponse")]
    pub update_organization_spend_alert_response: OrganizationSpendAlert,
    #[serde(rename = "UpdateProjectSpendAlertError400")]
    pub update_project_spend_alert_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectSpendAlertError403")]
    pub update_project_spend_alert_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateProjectSpendAlertError404")]
    pub update_project_spend_alert_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectSpendAlertParams")]
    pub update_project_spend_alert_params: UpdateProjectSpendAlertParams,
    #[serde(rename = "UpdateProjectSpendAlertRequest")]
    pub update_project_spend_alert_request: CreateSpendAlertBody,
    #[serde(rename = "UpdateProjectSpendAlertResponse")]
    pub update_project_spend_alert_response: ProjectSpendAlert,
}
impl SpendAlertsApiSchema {
    pub fn builder() -> builder::SpendAlertsApiSchema {
        Default::default()
    }
}
#[doc = "`UpdateOrganizationSpendAlertParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateOrganizationSpendAlertParams {
    pub alert_id: ::std::string::String,
}
impl UpdateOrganizationSpendAlertParams {
    pub fn builder() -> builder::UpdateOrganizationSpendAlertParams {
        Default::default()
    }
}
#[doc = "`UpdateProjectSpendAlertParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectSpendAlertParams {
    pub alert_id: ::std::string::String,
    pub project_id: ::std::string::String,
}
impl UpdateProjectSpendAlertParams {
    pub fn builder() -> builder::UpdateProjectSpendAlertParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateProjectSpendAlertParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateProjectSpendAlertParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl CreateProjectSpendAlertParams {
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
    impl ::std::convert::TryFrom<CreateProjectSpendAlertParams>
        for super::CreateProjectSpendAlertParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateProjectSpendAlertParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateProjectSpendAlertParams> for CreateProjectSpendAlertParams {
        fn from(value: super::CreateProjectSpendAlertParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateSpendAlertBody {
        currency: ::std::result::Result<super::CreateSpendAlertBodyCurrency, ::std::string::String>,
        interval: ::std::result::Result<super::CreateSpendAlertBodyInterval, ::std::string::String>,
        notification_channel:
            ::std::result::Result<super::SpendAlertNotificationChannel, ::std::string::String>,
        threshold_amount: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for CreateSpendAlertBody {
        fn default() -> Self {
            Self {
                currency: Err("no value supplied for currency".to_string()),
                interval: Err("no value supplied for interval".to_string()),
                notification_channel: Err("no value supplied for notification_channel".to_string()),
                threshold_amount: Err("no value supplied for threshold_amount".to_string()),
            }
        }
    }
    impl CreateSpendAlertBody {
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSpendAlertBodyCurrency>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for currency: {e}"));
            self
        }
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSpendAlertBodyInterval>,
            T::Error: ::std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {e}"));
            self
        }
        pub fn notification_channel<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendAlertNotificationChannel>,
            T::Error: ::std::fmt::Display,
        {
            self.notification_channel = value.try_into().map_err(|e| {
                format!("error converting supplied value for notification_channel: {e}")
            });
            self
        }
        pub fn threshold_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_amount: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateSpendAlertBody> for super::CreateSpendAlertBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateSpendAlertBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                currency: value.currency?,
                interval: value.interval?,
                notification_channel: value.notification_channel?,
                threshold_amount: value.threshold_amount?,
            })
        }
    }
    impl ::std::convert::From<super::CreateSpendAlertBody> for CreateSpendAlertBody {
        fn from(value: super::CreateSpendAlertBody) -> Self {
            Self {
                currency: Ok(value.currency),
                interval: Ok(value.interval),
                notification_channel: Ok(value.notification_channel),
                threshold_amount: Ok(value.threshold_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteOrganizationSpendAlertParams {
        alert_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteOrganizationSpendAlertParams {
        fn default() -> Self {
            Self {
                alert_id: Err("no value supplied for alert_id".to_string()),
            }
        }
    }
    impl DeleteOrganizationSpendAlertParams {
        pub fn alert_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.alert_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alert_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteOrganizationSpendAlertParams>
        for super::DeleteOrganizationSpendAlertParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteOrganizationSpendAlertParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alert_id: value.alert_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteOrganizationSpendAlertParams>
        for DeleteOrganizationSpendAlertParams
    {
        fn from(value: super::DeleteOrganizationSpendAlertParams) -> Self {
            Self {
                alert_id: Ok(value.alert_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteProjectSpendAlertParams {
        alert_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteProjectSpendAlertParams {
        fn default() -> Self {
            Self {
                alert_id: Err("no value supplied for alert_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl DeleteProjectSpendAlertParams {
        pub fn alert_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.alert_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alert_id: {e}"));
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
    impl ::std::convert::TryFrom<DeleteProjectSpendAlertParams>
        for super::DeleteProjectSpendAlertParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteProjectSpendAlertParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alert_id: value.alert_id?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteProjectSpendAlertParams> for DeleteProjectSpendAlertParams {
        fn from(value: super::DeleteProjectSpendAlertParams) -> Self {
            Self {
                alert_id: Ok(value.alert_id),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListOrganizationSpendAlertsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        before: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListOrganizationSpendAlertsParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListOrganizationSpendAlertsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListOrganizationSpendAlertsParams {
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
        pub fn before<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.before = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for before: {e}"));
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
                ::std::option::Option<super::ListOrganizationSpendAlertsParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListOrganizationSpendAlertsParams>
        for super::ListOrganizationSpendAlertsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListOrganizationSpendAlertsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListOrganizationSpendAlertsParams>
        for ListOrganizationSpendAlertsParams
    {
        fn from(value: super::ListOrganizationSpendAlertsParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectSpendAlertsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        before: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListProjectSpendAlertsParamsOrder>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectSpendAlertsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ListProjectSpendAlertsParams {
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
        pub fn before<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.before = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for before: {e}"));
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
                ::std::option::Option<super::ListProjectSpendAlertsParamsOrder>,
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
    impl ::std::convert::TryFrom<ListProjectSpendAlertsParams> for super::ListProjectSpendAlertsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectSpendAlertsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                limit: value.limit?,
                order: value.order?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectSpendAlertsParams> for ListProjectSpendAlertsParams {
        fn from(value: super::ListProjectSpendAlertsParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                limit: Ok(value.limit),
                order: Ok(value.order),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationSpendAlert {
        currency:
            ::std::result::Result<super::OrganizationSpendAlertCurrency, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        interval:
            ::std::result::Result<super::OrganizationSpendAlertInterval, ::std::string::String>,
        notification_channel:
            ::std::result::Result<super::SpendAlertNotificationChannel, ::std::string::String>,
        object: ::std::result::Result<super::OrganizationSpendAlertObject, ::std::string::String>,
        threshold_amount: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for OrganizationSpendAlert {
        fn default() -> Self {
            Self {
                currency: Err("no value supplied for currency".to_string()),
                id: Err("no value supplied for id".to_string()),
                interval: Err("no value supplied for interval".to_string()),
                notification_channel: Err("no value supplied for notification_channel".to_string()),
                object: Err("no value supplied for object".to_string()),
                threshold_amount: Err("no value supplied for threshold_amount".to_string()),
            }
        }
    }
    impl OrganizationSpendAlert {
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendAlertCurrency>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for currency: {e}"));
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
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendAlertInterval>,
            T::Error: ::std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {e}"));
            self
        }
        pub fn notification_channel<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendAlertNotificationChannel>,
            T::Error: ::std::fmt::Display,
        {
            self.notification_channel = value.try_into().map_err(|e| {
                format!("error converting supplied value for notification_channel: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendAlertObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn threshold_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_amount: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationSpendAlert> for super::OrganizationSpendAlert {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationSpendAlert,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                currency: value.currency?,
                id: value.id?,
                interval: value.interval?,
                notification_channel: value.notification_channel?,
                object: value.object?,
                threshold_amount: value.threshold_amount?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationSpendAlert> for OrganizationSpendAlert {
        fn from(value: super::OrganizationSpendAlert) -> Self {
            Self {
                currency: Ok(value.currency),
                id: Ok(value.id),
                interval: Ok(value.interval),
                notification_channel: Ok(value.notification_channel),
                object: Ok(value.object),
                threshold_amount: Ok(value.threshold_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationSpendAlertDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::OrganizationSpendAlertDeletedResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationSpendAlertDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl OrganizationSpendAlertDeletedResource {
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
            T: ::std::convert::TryInto<super::OrganizationSpendAlertDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationSpendAlertDeletedResource>
        for super::OrganizationSpendAlertDeletedResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationSpendAlertDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationSpendAlertDeletedResource>
        for OrganizationSpendAlertDeletedResource
    {
        fn from(value: super::OrganizationSpendAlertDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationSpendAlertListResource {
        data: ::std::result::Result<
            ::std::vec::Vec<super::OrganizationSpendAlert>,
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
            super::OrganizationSpendAlertListResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationSpendAlertListResource {
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
    impl OrganizationSpendAlertListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrganizationSpendAlert>>,
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
            T: ::std::convert::TryInto<super::OrganizationSpendAlertListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationSpendAlertListResource>
        for super::OrganizationSpendAlertListResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationSpendAlertListResource,
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
    impl ::std::convert::From<super::OrganizationSpendAlertListResource>
        for OrganizationSpendAlertListResource
    {
        fn from(value: super::OrganizationSpendAlertListResource) -> Self {
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
    pub struct ProjectSpendAlert {
        currency: ::std::result::Result<super::ProjectSpendAlertCurrency, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        interval: ::std::result::Result<super::ProjectSpendAlertInterval, ::std::string::String>,
        notification_channel:
            ::std::result::Result<super::SpendAlertNotificationChannel, ::std::string::String>,
        object: ::std::result::Result<super::ProjectSpendAlertObject, ::std::string::String>,
        threshold_amount: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectSpendAlert {
        fn default() -> Self {
            Self {
                currency: Err("no value supplied for currency".to_string()),
                id: Err("no value supplied for id".to_string()),
                interval: Err("no value supplied for interval".to_string()),
                notification_channel: Err("no value supplied for notification_channel".to_string()),
                object: Err("no value supplied for object".to_string()),
                threshold_amount: Err("no value supplied for threshold_amount".to_string()),
            }
        }
    }
    impl ProjectSpendAlert {
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendAlertCurrency>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for currency: {e}"));
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
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendAlertInterval>,
            T::Error: ::std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {e}"));
            self
        }
        pub fn notification_channel<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendAlertNotificationChannel>,
            T::Error: ::std::fmt::Display,
        {
            self.notification_channel = value.try_into().map_err(|e| {
                format!("error converting supplied value for notification_channel: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendAlertObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn threshold_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_amount: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectSpendAlert> for super::ProjectSpendAlert {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectSpendAlert,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                currency: value.currency?,
                id: value.id?,
                interval: value.interval?,
                notification_channel: value.notification_channel?,
                object: value.object?,
                threshold_amount: value.threshold_amount?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectSpendAlert> for ProjectSpendAlert {
        fn from(value: super::ProjectSpendAlert) -> Self {
            Self {
                currency: Ok(value.currency),
                id: Ok(value.id),
                interval: Ok(value.interval),
                notification_channel: Ok(value.notification_channel),
                object: Ok(value.object),
                threshold_amount: Ok(value.threshold_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectSpendAlertDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::ProjectSpendAlertDeletedResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectSpendAlertDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectSpendAlertDeletedResource {
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
            T: ::std::convert::TryInto<super::ProjectSpendAlertDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectSpendAlertDeletedResource>
        for super::ProjectSpendAlertDeletedResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectSpendAlertDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectSpendAlertDeletedResource>
        for ProjectSpendAlertDeletedResource
    {
        fn from(value: super::ProjectSpendAlertDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectSpendAlertListResource {
        data:
            ::std::result::Result<::std::vec::Vec<super::ProjectSpendAlert>, ::std::string::String>,
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
            super::ProjectSpendAlertListResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectSpendAlertListResource {
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
    impl ProjectSpendAlertListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProjectSpendAlert>>,
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
            T: ::std::convert::TryInto<super::ProjectSpendAlertListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectSpendAlertListResource>
        for super::ProjectSpendAlertListResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectSpendAlertListResource,
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
    impl ::std::convert::From<super::ProjectSpendAlertListResource> for ProjectSpendAlertListResource {
        fn from(value: super::ProjectSpendAlertListResource) -> Self {
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
    pub struct RetrieveOrganizationSpendAlertParams {
        alert_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveOrganizationSpendAlertParams {
        fn default() -> Self {
            Self {
                alert_id: Err("no value supplied for alert_id".to_string()),
            }
        }
    }
    impl RetrieveOrganizationSpendAlertParams {
        pub fn alert_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.alert_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alert_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveOrganizationSpendAlertParams>
        for super::RetrieveOrganizationSpendAlertParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveOrganizationSpendAlertParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alert_id: value.alert_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveOrganizationSpendAlertParams>
        for RetrieveOrganizationSpendAlertParams
    {
        fn from(value: super::RetrieveOrganizationSpendAlertParams) -> Self {
            Self {
                alert_id: Ok(value.alert_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectSpendAlertParams {
        alert_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectSpendAlertParams {
        fn default() -> Self {
            Self {
                alert_id: Err("no value supplied for alert_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl RetrieveProjectSpendAlertParams {
        pub fn alert_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.alert_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alert_id: {e}"));
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
    impl ::std::convert::TryFrom<RetrieveProjectSpendAlertParams>
        for super::RetrieveProjectSpendAlertParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectSpendAlertParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alert_id: value.alert_id?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectSpendAlertParams>
        for RetrieveProjectSpendAlertParams
    {
        fn from(value: super::RetrieveProjectSpendAlertParams) -> Self {
            Self {
                alert_id: Ok(value.alert_id),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpendAlertNotificationChannel {
        recipients:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        subject_prefix: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<super::SpendAlertNotificationChannelType, ::std::string::String>,
    }
    impl ::std::default::Default for SpendAlertNotificationChannel {
        fn default() -> Self {
            Self {
                recipients: Err("no value supplied for recipients".to_string()),
                subject_prefix: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SpendAlertNotificationChannel {
        pub fn recipients<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.recipients = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipients: {e}"));
            self
        }
        pub fn subject_prefix<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.subject_prefix = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subject_prefix: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendAlertNotificationChannelType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SpendAlertNotificationChannel>
        for super::SpendAlertNotificationChannel
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpendAlertNotificationChannel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                recipients: value.recipients?,
                subject_prefix: value.subject_prefix?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SpendAlertNotificationChannel> for SpendAlertNotificationChannel {
        fn from(value: super::SpendAlertNotificationChannel) -> Self {
            Self {
                recipients: Ok(value.recipients),
                subject_prefix: Ok(value.subject_prefix),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpendAlertsApiSchema {
        create_organization_spend_alert_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_organization_spend_alert_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        create_organization_spend_alert_request:
            ::std::result::Result<super::CreateSpendAlertBody, ::std::string::String>,
        create_organization_spend_alert_response:
            ::std::result::Result<super::OrganizationSpendAlert, ::std::string::String>,
        create_project_spend_alert_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_spend_alert_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        create_project_spend_alert_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_spend_alert_params:
            ::std::result::Result<super::CreateProjectSpendAlertParams, ::std::string::String>,
        create_project_spend_alert_request:
            ::std::result::Result<super::CreateSpendAlertBody, ::std::string::String>,
        create_project_spend_alert_response:
            ::std::result::Result<super::ProjectSpendAlert, ::std::string::String>,
        delete_organization_spend_alert_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_organization_spend_alert_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_organization_spend_alert_params:
            ::std::result::Result<super::DeleteOrganizationSpendAlertParams, ::std::string::String>,
        delete_organization_spend_alert_response: ::std::result::Result<
            super::OrganizationSpendAlertDeletedResource,
            ::std::string::String,
        >,
        delete_project_spend_alert_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_spend_alert_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_project_spend_alert_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_spend_alert_params:
            ::std::result::Result<super::DeleteProjectSpendAlertParams, ::std::string::String>,
        delete_project_spend_alert_response:
            ::std::result::Result<super::ProjectSpendAlertDeletedResource, ::std::string::String>,
        list_organization_spend_alerts_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_organization_spend_alerts_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_organization_spend_alerts_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_organization_spend_alerts_params:
            ::std::result::Result<super::ListOrganizationSpendAlertsParams, ::std::string::String>,
        list_organization_spend_alerts_response:
            ::std::result::Result<super::OrganizationSpendAlertListResource, ::std::string::String>,
        list_project_spend_alerts_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_spend_alerts_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_spend_alerts_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_spend_alerts_params:
            ::std::result::Result<super::ListProjectSpendAlertsParams, ::std::string::String>,
        list_project_spend_alerts_response:
            ::std::result::Result<super::ProjectSpendAlertListResource, ::std::string::String>,
        retrieve_organization_spend_alert_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_organization_spend_alert_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_organization_spend_alert_params: ::std::result::Result<
            super::RetrieveOrganizationSpendAlertParams,
            ::std::string::String,
        >,
        retrieve_organization_spend_alert_response:
            ::std::result::Result<super::OrganizationSpendAlert, ::std::string::String>,
        retrieve_project_spend_alert_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_spend_alert_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_spend_alert_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_spend_alert_params:
            ::std::result::Result<super::RetrieveProjectSpendAlertParams, ::std::string::String>,
        retrieve_project_spend_alert_response:
            ::std::result::Result<super::ProjectSpendAlert, ::std::string::String>,
        update_organization_spend_alert_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_organization_spend_alert_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_organization_spend_alert_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_organization_spend_alert_params:
            ::std::result::Result<super::UpdateOrganizationSpendAlertParams, ::std::string::String>,
        update_organization_spend_alert_request:
            ::std::result::Result<super::CreateSpendAlertBody, ::std::string::String>,
        update_organization_spend_alert_response:
            ::std::result::Result<super::OrganizationSpendAlert, ::std::string::String>,
        update_project_spend_alert_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_spend_alert_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_project_spend_alert_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_spend_alert_params:
            ::std::result::Result<super::UpdateProjectSpendAlertParams, ::std::string::String>,
        update_project_spend_alert_request:
            ::std::result::Result<super::CreateSpendAlertBody, ::std::string::String>,
        update_project_spend_alert_response:
            ::std::result::Result<super::ProjectSpendAlert, ::std::string::String>,
    }
    impl ::std::default::Default for SpendAlertsApiSchema {
        fn default() -> Self {
            Self {
                create_organization_spend_alert_error400: Err(
                    "no value supplied for create_organization_spend_alert_error400".to_string(),
                ),
                create_organization_spend_alert_error403: Err(
                    "no value supplied for create_organization_spend_alert_error403".to_string(),
                ),
                create_organization_spend_alert_request: Err(
                    "no value supplied for create_organization_spend_alert_request".to_string(),
                ),
                create_organization_spend_alert_response: Err(
                    "no value supplied for create_organization_spend_alert_response".to_string(),
                ),
                create_project_spend_alert_error400: Err(
                    "no value supplied for create_project_spend_alert_error400".to_string(),
                ),
                create_project_spend_alert_error403: Err(
                    "no value supplied for create_project_spend_alert_error403".to_string(),
                ),
                create_project_spend_alert_error404: Err(
                    "no value supplied for create_project_spend_alert_error404".to_string(),
                ),
                create_project_spend_alert_params: Err(
                    "no value supplied for create_project_spend_alert_params".to_string(),
                ),
                create_project_spend_alert_request: Err(
                    "no value supplied for create_project_spend_alert_request".to_string(),
                ),
                create_project_spend_alert_response: Err(
                    "no value supplied for create_project_spend_alert_response".to_string(),
                ),
                delete_organization_spend_alert_error403: Err(
                    "no value supplied for delete_organization_spend_alert_error403".to_string(),
                ),
                delete_organization_spend_alert_error404: Err(
                    "no value supplied for delete_organization_spend_alert_error404".to_string(),
                ),
                delete_organization_spend_alert_params: Err(
                    "no value supplied for delete_organization_spend_alert_params".to_string(),
                ),
                delete_organization_spend_alert_response: Err(
                    "no value supplied for delete_organization_spend_alert_response".to_string(),
                ),
                delete_project_spend_alert_error400: Err(
                    "no value supplied for delete_project_spend_alert_error400".to_string(),
                ),
                delete_project_spend_alert_error403: Err(
                    "no value supplied for delete_project_spend_alert_error403".to_string(),
                ),
                delete_project_spend_alert_error404: Err(
                    "no value supplied for delete_project_spend_alert_error404".to_string(),
                ),
                delete_project_spend_alert_params: Err(
                    "no value supplied for delete_project_spend_alert_params".to_string(),
                ),
                delete_project_spend_alert_response: Err(
                    "no value supplied for delete_project_spend_alert_response".to_string(),
                ),
                list_organization_spend_alerts_error400: Err(
                    "no value supplied for list_organization_spend_alerts_error400".to_string(),
                ),
                list_organization_spend_alerts_error403: Err(
                    "no value supplied for list_organization_spend_alerts_error403".to_string(),
                ),
                list_organization_spend_alerts_error404: Err(
                    "no value supplied for list_organization_spend_alerts_error404".to_string(),
                ),
                list_organization_spend_alerts_params: Err(
                    "no value supplied for list_organization_spend_alerts_params".to_string(),
                ),
                list_organization_spend_alerts_response: Err(
                    "no value supplied for list_organization_spend_alerts_response".to_string(),
                ),
                list_project_spend_alerts_error400: Err(
                    "no value supplied for list_project_spend_alerts_error400".to_string(),
                ),
                list_project_spend_alerts_error403: Err(
                    "no value supplied for list_project_spend_alerts_error403".to_string(),
                ),
                list_project_spend_alerts_error404: Err(
                    "no value supplied for list_project_spend_alerts_error404".to_string(),
                ),
                list_project_spend_alerts_params: Err(
                    "no value supplied for list_project_spend_alerts_params".to_string(),
                ),
                list_project_spend_alerts_response: Err(
                    "no value supplied for list_project_spend_alerts_response".to_string(),
                ),
                retrieve_organization_spend_alert_error403: Err(
                    "no value supplied for retrieve_organization_spend_alert_error403".to_string(),
                ),
                retrieve_organization_spend_alert_error404: Err(
                    "no value supplied for retrieve_organization_spend_alert_error404".to_string(),
                ),
                retrieve_organization_spend_alert_params: Err(
                    "no value supplied for retrieve_organization_spend_alert_params".to_string(),
                ),
                retrieve_organization_spend_alert_response: Err(
                    "no value supplied for retrieve_organization_spend_alert_response".to_string(),
                ),
                retrieve_project_spend_alert_error400: Err(
                    "no value supplied for retrieve_project_spend_alert_error400".to_string(),
                ),
                retrieve_project_spend_alert_error403: Err(
                    "no value supplied for retrieve_project_spend_alert_error403".to_string(),
                ),
                retrieve_project_spend_alert_error404: Err(
                    "no value supplied for retrieve_project_spend_alert_error404".to_string(),
                ),
                retrieve_project_spend_alert_params: Err(
                    "no value supplied for retrieve_project_spend_alert_params".to_string(),
                ),
                retrieve_project_spend_alert_response: Err(
                    "no value supplied for retrieve_project_spend_alert_response".to_string(),
                ),
                update_organization_spend_alert_error400: Err(
                    "no value supplied for update_organization_spend_alert_error400".to_string(),
                ),
                update_organization_spend_alert_error403: Err(
                    "no value supplied for update_organization_spend_alert_error403".to_string(),
                ),
                update_organization_spend_alert_error404: Err(
                    "no value supplied for update_organization_spend_alert_error404".to_string(),
                ),
                update_organization_spend_alert_params: Err(
                    "no value supplied for update_organization_spend_alert_params".to_string(),
                ),
                update_organization_spend_alert_request: Err(
                    "no value supplied for update_organization_spend_alert_request".to_string(),
                ),
                update_organization_spend_alert_response: Err(
                    "no value supplied for update_organization_spend_alert_response".to_string(),
                ),
                update_project_spend_alert_error400: Err(
                    "no value supplied for update_project_spend_alert_error400".to_string(),
                ),
                update_project_spend_alert_error403: Err(
                    "no value supplied for update_project_spend_alert_error403".to_string(),
                ),
                update_project_spend_alert_error404: Err(
                    "no value supplied for update_project_spend_alert_error404".to_string(),
                ),
                update_project_spend_alert_params: Err(
                    "no value supplied for update_project_spend_alert_params".to_string(),
                ),
                update_project_spend_alert_request: Err(
                    "no value supplied for update_project_spend_alert_request".to_string(),
                ),
                update_project_spend_alert_response: Err(
                    "no value supplied for update_project_spend_alert_response".to_string(),
                ),
            }
        }
    }
    impl SpendAlertsApiSchema {
        pub fn create_organization_spend_alert_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_organization_spend_alert_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_organization_spend_alert_error400: {e}")) ;
            self
        }
        pub fn create_organization_spend_alert_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_organization_spend_alert_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_organization_spend_alert_error403: {e}")) ;
            self
        }
        pub fn create_organization_spend_alert_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSpendAlertBody>,
            T::Error: ::std::fmt::Display,
        {
            self . create_organization_spend_alert_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_organization_spend_alert_request: {e}")) ;
            self
        }
        pub fn create_organization_spend_alert_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendAlert>,
            T::Error: ::std::fmt::Display,
        {
            self . create_organization_spend_alert_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_organization_spend_alert_response: {e}")) ;
            self
        }
        pub fn create_project_spend_alert_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_spend_alert_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_project_spend_alert_error400: {e}"
                )
            });
            self
        }
        pub fn create_project_spend_alert_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_spend_alert_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_project_spend_alert_error403: {e}"
                )
            });
            self
        }
        pub fn create_project_spend_alert_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_spend_alert_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_project_spend_alert_error404: {e}"
                )
            });
            self
        }
        pub fn create_project_spend_alert_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateProjectSpendAlertParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_spend_alert_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_project_spend_alert_params: {e}"
                )
            });
            self
        }
        pub fn create_project_spend_alert_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSpendAlertBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_spend_alert_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_project_spend_alert_request: {e}"
                )
            });
            self
        }
        pub fn create_project_spend_alert_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendAlert>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_spend_alert_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_project_spend_alert_response: {e}"
                )
            });
            self
        }
        pub fn delete_organization_spend_alert_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_organization_spend_alert_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_organization_spend_alert_error403: {e}")) ;
            self
        }
        pub fn delete_organization_spend_alert_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_organization_spend_alert_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_organization_spend_alert_error404: {e}")) ;
            self
        }
        pub fn delete_organization_spend_alert_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteOrganizationSpendAlertParams>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_organization_spend_alert_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_organization_spend_alert_params: {e}")) ;
            self
        }
        pub fn delete_organization_spend_alert_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendAlertDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_organization_spend_alert_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_organization_spend_alert_response: {e}")) ;
            self
        }
        pub fn delete_project_spend_alert_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_spend_alert_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_project_spend_alert_error400: {e}"
                )
            });
            self
        }
        pub fn delete_project_spend_alert_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_spend_alert_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_project_spend_alert_error403: {e}"
                )
            });
            self
        }
        pub fn delete_project_spend_alert_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_spend_alert_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_project_spend_alert_error404: {e}"
                )
            });
            self
        }
        pub fn delete_project_spend_alert_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteProjectSpendAlertParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_spend_alert_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_project_spend_alert_params: {e}"
                )
            });
            self
        }
        pub fn delete_project_spend_alert_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendAlertDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_spend_alert_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_project_spend_alert_response: {e}"
                )
            });
            self
        }
        pub fn list_organization_spend_alerts_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_spend_alerts_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_spend_alerts_error400: {e}")) ;
            self
        }
        pub fn list_organization_spend_alerts_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_spend_alerts_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_spend_alerts_error403: {e}")) ;
            self
        }
        pub fn list_organization_spend_alerts_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_spend_alerts_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_spend_alerts_error404: {e}")) ;
            self
        }
        pub fn list_organization_spend_alerts_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListOrganizationSpendAlertsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_spend_alerts_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_spend_alerts_params: {e}")) ;
            self
        }
        pub fn list_organization_spend_alerts_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendAlertListResource>,
            T::Error: ::std::fmt::Display,
        {
            self . list_organization_spend_alerts_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_organization_spend_alerts_response: {e}")) ;
            self
        }
        pub fn list_project_spend_alerts_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_spend_alerts_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_spend_alerts_error400: {e}"
                )
            });
            self
        }
        pub fn list_project_spend_alerts_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_spend_alerts_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_spend_alerts_error403: {e}"
                )
            });
            self
        }
        pub fn list_project_spend_alerts_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_spend_alerts_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_spend_alerts_error404: {e}"
                )
            });
            self
        }
        pub fn list_project_spend_alerts_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectSpendAlertsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_spend_alerts_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_spend_alerts_params: {e}")
            });
            self
        }
        pub fn list_project_spend_alerts_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendAlertListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_spend_alerts_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_spend_alerts_response: {e}"
                )
            });
            self
        }
        pub fn retrieve_organization_spend_alert_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_organization_spend_alert_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_organization_spend_alert_error403: {e}")) ;
            self
        }
        pub fn retrieve_organization_spend_alert_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_organization_spend_alert_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_organization_spend_alert_error404: {e}")) ;
            self
        }
        pub fn retrieve_organization_spend_alert_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveOrganizationSpendAlertParams>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_organization_spend_alert_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_organization_spend_alert_params: {e}")) ;
            self
        }
        pub fn retrieve_organization_spend_alert_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendAlert>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_organization_spend_alert_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_organization_spend_alert_response: {e}")) ;
            self
        }
        pub fn retrieve_project_spend_alert_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_spend_alert_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_spend_alert_error400: {e}")) ;
            self
        }
        pub fn retrieve_project_spend_alert_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_spend_alert_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_spend_alert_error403: {e}")) ;
            self
        }
        pub fn retrieve_project_spend_alert_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_spend_alert_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_spend_alert_error404: {e}")) ;
            self
        }
        pub fn retrieve_project_spend_alert_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectSpendAlertParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_spend_alert_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_spend_alert_params: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_spend_alert_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendAlert>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_spend_alert_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_spend_alert_response: {e}")) ;
            self
        }
        pub fn update_organization_spend_alert_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_spend_alert_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_spend_alert_error400: {e}")) ;
            self
        }
        pub fn update_organization_spend_alert_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_spend_alert_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_spend_alert_error403: {e}")) ;
            self
        }
        pub fn update_organization_spend_alert_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_spend_alert_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_spend_alert_error404: {e}")) ;
            self
        }
        pub fn update_organization_spend_alert_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateOrganizationSpendAlertParams>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_spend_alert_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_spend_alert_params: {e}")) ;
            self
        }
        pub fn update_organization_spend_alert_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSpendAlertBody>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_spend_alert_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_spend_alert_request: {e}")) ;
            self
        }
        pub fn update_organization_spend_alert_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendAlert>,
            T::Error: ::std::fmt::Display,
        {
            self . update_organization_spend_alert_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_organization_spend_alert_response: {e}")) ;
            self
        }
        pub fn update_project_spend_alert_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_spend_alert_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_spend_alert_error400: {e}"
                )
            });
            self
        }
        pub fn update_project_spend_alert_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_spend_alert_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_spend_alert_error403: {e}"
                )
            });
            self
        }
        pub fn update_project_spend_alert_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_spend_alert_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_spend_alert_error404: {e}"
                )
            });
            self
        }
        pub fn update_project_spend_alert_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectSpendAlertParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_spend_alert_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_spend_alert_params: {e}"
                )
            });
            self
        }
        pub fn update_project_spend_alert_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSpendAlertBody>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_spend_alert_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_spend_alert_request: {e}"
                )
            });
            self
        }
        pub fn update_project_spend_alert_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendAlert>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_spend_alert_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_spend_alert_response: {e}"
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<SpendAlertsApiSchema> for super::SpendAlertsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpendAlertsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_organization_spend_alert_error400: value
                    .create_organization_spend_alert_error400?,
                create_organization_spend_alert_error403: value
                    .create_organization_spend_alert_error403?,
                create_organization_spend_alert_request: value
                    .create_organization_spend_alert_request?,
                create_organization_spend_alert_response: value
                    .create_organization_spend_alert_response?,
                create_project_spend_alert_error400: value.create_project_spend_alert_error400?,
                create_project_spend_alert_error403: value.create_project_spend_alert_error403?,
                create_project_spend_alert_error404: value.create_project_spend_alert_error404?,
                create_project_spend_alert_params: value.create_project_spend_alert_params?,
                create_project_spend_alert_request: value.create_project_spend_alert_request?,
                create_project_spend_alert_response: value.create_project_spend_alert_response?,
                delete_organization_spend_alert_error403: value
                    .delete_organization_spend_alert_error403?,
                delete_organization_spend_alert_error404: value
                    .delete_organization_spend_alert_error404?,
                delete_organization_spend_alert_params: value
                    .delete_organization_spend_alert_params?,
                delete_organization_spend_alert_response: value
                    .delete_organization_spend_alert_response?,
                delete_project_spend_alert_error400: value.delete_project_spend_alert_error400?,
                delete_project_spend_alert_error403: value.delete_project_spend_alert_error403?,
                delete_project_spend_alert_error404: value.delete_project_spend_alert_error404?,
                delete_project_spend_alert_params: value.delete_project_spend_alert_params?,
                delete_project_spend_alert_response: value.delete_project_spend_alert_response?,
                list_organization_spend_alerts_error400: value
                    .list_organization_spend_alerts_error400?,
                list_organization_spend_alerts_error403: value
                    .list_organization_spend_alerts_error403?,
                list_organization_spend_alerts_error404: value
                    .list_organization_spend_alerts_error404?,
                list_organization_spend_alerts_params: value
                    .list_organization_spend_alerts_params?,
                list_organization_spend_alerts_response: value
                    .list_organization_spend_alerts_response?,
                list_project_spend_alerts_error400: value.list_project_spend_alerts_error400?,
                list_project_spend_alerts_error403: value.list_project_spend_alerts_error403?,
                list_project_spend_alerts_error404: value.list_project_spend_alerts_error404?,
                list_project_spend_alerts_params: value.list_project_spend_alerts_params?,
                list_project_spend_alerts_response: value.list_project_spend_alerts_response?,
                retrieve_organization_spend_alert_error403: value
                    .retrieve_organization_spend_alert_error403?,
                retrieve_organization_spend_alert_error404: value
                    .retrieve_organization_spend_alert_error404?,
                retrieve_organization_spend_alert_params: value
                    .retrieve_organization_spend_alert_params?,
                retrieve_organization_spend_alert_response: value
                    .retrieve_organization_spend_alert_response?,
                retrieve_project_spend_alert_error400: value
                    .retrieve_project_spend_alert_error400?,
                retrieve_project_spend_alert_error403: value
                    .retrieve_project_spend_alert_error403?,
                retrieve_project_spend_alert_error404: value
                    .retrieve_project_spend_alert_error404?,
                retrieve_project_spend_alert_params: value.retrieve_project_spend_alert_params?,
                retrieve_project_spend_alert_response: value
                    .retrieve_project_spend_alert_response?,
                update_organization_spend_alert_error400: value
                    .update_organization_spend_alert_error400?,
                update_organization_spend_alert_error403: value
                    .update_organization_spend_alert_error403?,
                update_organization_spend_alert_error404: value
                    .update_organization_spend_alert_error404?,
                update_organization_spend_alert_params: value
                    .update_organization_spend_alert_params?,
                update_organization_spend_alert_request: value
                    .update_organization_spend_alert_request?,
                update_organization_spend_alert_response: value
                    .update_organization_spend_alert_response?,
                update_project_spend_alert_error400: value.update_project_spend_alert_error400?,
                update_project_spend_alert_error403: value.update_project_spend_alert_error403?,
                update_project_spend_alert_error404: value.update_project_spend_alert_error404?,
                update_project_spend_alert_params: value.update_project_spend_alert_params?,
                update_project_spend_alert_request: value.update_project_spend_alert_request?,
                update_project_spend_alert_response: value.update_project_spend_alert_response?,
            })
        }
    }
    impl ::std::convert::From<super::SpendAlertsApiSchema> for SpendAlertsApiSchema {
        fn from(value: super::SpendAlertsApiSchema) -> Self {
            Self {
                create_organization_spend_alert_error400: Ok(
                    value.create_organization_spend_alert_error400
                ),
                create_organization_spend_alert_error403: Ok(
                    value.create_organization_spend_alert_error403
                ),
                create_organization_spend_alert_request: Ok(
                    value.create_organization_spend_alert_request
                ),
                create_organization_spend_alert_response: Ok(
                    value.create_organization_spend_alert_response
                ),
                create_project_spend_alert_error400: Ok(value.create_project_spend_alert_error400),
                create_project_spend_alert_error403: Ok(value.create_project_spend_alert_error403),
                create_project_spend_alert_error404: Ok(value.create_project_spend_alert_error404),
                create_project_spend_alert_params: Ok(value.create_project_spend_alert_params),
                create_project_spend_alert_request: Ok(value.create_project_spend_alert_request),
                create_project_spend_alert_response: Ok(value.create_project_spend_alert_response),
                delete_organization_spend_alert_error403: Ok(
                    value.delete_organization_spend_alert_error403
                ),
                delete_organization_spend_alert_error404: Ok(
                    value.delete_organization_spend_alert_error404
                ),
                delete_organization_spend_alert_params: Ok(
                    value.delete_organization_spend_alert_params
                ),
                delete_organization_spend_alert_response: Ok(
                    value.delete_organization_spend_alert_response
                ),
                delete_project_spend_alert_error400: Ok(value.delete_project_spend_alert_error400),
                delete_project_spend_alert_error403: Ok(value.delete_project_spend_alert_error403),
                delete_project_spend_alert_error404: Ok(value.delete_project_spend_alert_error404),
                delete_project_spend_alert_params: Ok(value.delete_project_spend_alert_params),
                delete_project_spend_alert_response: Ok(value.delete_project_spend_alert_response),
                list_organization_spend_alerts_error400: Ok(
                    value.list_organization_spend_alerts_error400
                ),
                list_organization_spend_alerts_error403: Ok(
                    value.list_organization_spend_alerts_error403
                ),
                list_organization_spend_alerts_error404: Ok(
                    value.list_organization_spend_alerts_error404
                ),
                list_organization_spend_alerts_params: Ok(
                    value.list_organization_spend_alerts_params
                ),
                list_organization_spend_alerts_response: Ok(
                    value.list_organization_spend_alerts_response
                ),
                list_project_spend_alerts_error400: Ok(value.list_project_spend_alerts_error400),
                list_project_spend_alerts_error403: Ok(value.list_project_spend_alerts_error403),
                list_project_spend_alerts_error404: Ok(value.list_project_spend_alerts_error404),
                list_project_spend_alerts_params: Ok(value.list_project_spend_alerts_params),
                list_project_spend_alerts_response: Ok(value.list_project_spend_alerts_response),
                retrieve_organization_spend_alert_error403: Ok(
                    value.retrieve_organization_spend_alert_error403
                ),
                retrieve_organization_spend_alert_error404: Ok(
                    value.retrieve_organization_spend_alert_error404
                ),
                retrieve_organization_spend_alert_params: Ok(
                    value.retrieve_organization_spend_alert_params
                ),
                retrieve_organization_spend_alert_response: Ok(
                    value.retrieve_organization_spend_alert_response
                ),
                retrieve_project_spend_alert_error400: Ok(
                    value.retrieve_project_spend_alert_error400
                ),
                retrieve_project_spend_alert_error403: Ok(
                    value.retrieve_project_spend_alert_error403
                ),
                retrieve_project_spend_alert_error404: Ok(
                    value.retrieve_project_spend_alert_error404
                ),
                retrieve_project_spend_alert_params: Ok(value.retrieve_project_spend_alert_params),
                retrieve_project_spend_alert_response: Ok(
                    value.retrieve_project_spend_alert_response
                ),
                update_organization_spend_alert_error400: Ok(
                    value.update_organization_spend_alert_error400
                ),
                update_organization_spend_alert_error403: Ok(
                    value.update_organization_spend_alert_error403
                ),
                update_organization_spend_alert_error404: Ok(
                    value.update_organization_spend_alert_error404
                ),
                update_organization_spend_alert_params: Ok(
                    value.update_organization_spend_alert_params
                ),
                update_organization_spend_alert_request: Ok(
                    value.update_organization_spend_alert_request
                ),
                update_organization_spend_alert_response: Ok(
                    value.update_organization_spend_alert_response
                ),
                update_project_spend_alert_error400: Ok(value.update_project_spend_alert_error400),
                update_project_spend_alert_error403: Ok(value.update_project_spend_alert_error403),
                update_project_spend_alert_error404: Ok(value.update_project_spend_alert_error404),
                update_project_spend_alert_params: Ok(value.update_project_spend_alert_params),
                update_project_spend_alert_request: Ok(value.update_project_spend_alert_request),
                update_project_spend_alert_response: Ok(value.update_project_spend_alert_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateOrganizationSpendAlertParams {
        alert_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateOrganizationSpendAlertParams {
        fn default() -> Self {
            Self {
                alert_id: Err("no value supplied for alert_id".to_string()),
            }
        }
    }
    impl UpdateOrganizationSpendAlertParams {
        pub fn alert_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.alert_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alert_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateOrganizationSpendAlertParams>
        for super::UpdateOrganizationSpendAlertParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateOrganizationSpendAlertParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alert_id: value.alert_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateOrganizationSpendAlertParams>
        for UpdateOrganizationSpendAlertParams
    {
        fn from(value: super::UpdateOrganizationSpendAlertParams) -> Self {
            Self {
                alert_id: Ok(value.alert_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectSpendAlertParams {
        alert_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectSpendAlertParams {
        fn default() -> Self {
            Self {
                alert_id: Err("no value supplied for alert_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl UpdateProjectSpendAlertParams {
        pub fn alert_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.alert_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alert_id: {e}"));
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
    impl ::std::convert::TryFrom<UpdateProjectSpendAlertParams>
        for super::UpdateProjectSpendAlertParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectSpendAlertParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alert_id: value.alert_id?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectSpendAlertParams> for UpdateProjectSpendAlertParams {
        fn from(value: super::UpdateProjectSpendAlertParams) -> Self {
            Self {
                alert_id: Ok(value.alert_id),
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
