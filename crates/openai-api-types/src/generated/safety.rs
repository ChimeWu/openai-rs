pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`GetprojectsafetyalertParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetprojectsafetyalertParams {
    #[doc = "Project safety alert ID"]
    pub id: GetprojectsafetyalertParamsId,
}
impl GetprojectsafetyalertParams {
    pub fn builder() -> builder::GetprojectsafetyalertParams {
        Default::default()
    }
}
#[doc = "Project safety alert ID"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetprojectsafetyalertParamsId(::std::string::String);
impl ::std::ops::Deref for GetprojectsafetyalertParamsId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<GetprojectsafetyalertParamsId> for ::std::string::String {
    fn from(value: GetprojectsafetyalertParamsId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for GetprojectsafetyalertParamsId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 38usize {
            return Err("longer than 38 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for GetprojectsafetyalertParamsId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetprojectsafetyalertParamsId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for GetprojectsafetyalertParamsId {
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
#[doc = "`GetsafetycaseParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetsafetycaseParams {
    #[doc = "Safety case ID"]
    pub id: GetsafetycaseParamsId,
}
impl GetsafetycaseParams {
    pub fn builder() -> builder::GetsafetycaseParams {
        Default::default()
    }
}
#[doc = "Safety case ID"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetsafetycaseParamsId(::std::string::String);
impl ::std::ops::Deref for GetsafetycaseParamsId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<GetsafetycaseParamsId> for ::std::string::String {
    fn from(value: GetsafetycaseParamsId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for GetsafetycaseParamsId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for GetsafetycaseParamsId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetsafetycaseParamsId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for GetsafetycaseParamsId {
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
#[doc = "`SafetyAlertErrorType`"]
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
pub enum SafetyAlertErrorType {
    #[serde(rename = "potentially_unintended_data_transfer")]
    PotentiallyUnintendedDataTransfer,
    #[serde(rename = "potentially_unintended_data_access")]
    PotentiallyUnintendedDataAccess,
    #[serde(rename = "potentially_unintended_destructive_activity")]
    PotentiallyUnintendedDestructiveActivity,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for SafetyAlertErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PotentiallyUnintendedDataTransfer => {
                f.write_str("potentially_unintended_data_transfer")
            }
            Self::PotentiallyUnintendedDataAccess => {
                f.write_str("potentially_unintended_data_access")
            }
            Self::PotentiallyUnintendedDestructiveActivity => {
                f.write_str("potentially_unintended_destructive_activity")
            }
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for SafetyAlertErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "potentially_unintended_data_transfer" => Ok(Self::PotentiallyUnintendedDataTransfer),
            "potentially_unintended_data_access" => Ok(Self::PotentiallyUnintendedDataAccess),
            "potentially_unintended_destructive_activity" => {
                Ok(Self::PotentiallyUnintendedDestructiveActivity)
            }
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SafetyAlertErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SafetyAlertErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SafetyAlertResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SafetyAlertResource {
    pub created_at: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detailed_explanation: ::std::option::Option<::std::string::String>,
    pub error_type: SafetyAlertErrorType,
    pub id: ::std::string::String,
    pub model: ::std::string::String,
    pub object: SafetyAlertResourceObject,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub reason: ::std::option::Option<::std::string::String>,
    pub request_id: ::std::string::String,
    #[doc = "Whether block registration succeeded for this request. This does not confirm that response execution stopped."]
    pub request_paused: bool,
    pub response_id: ::std::string::String,
}
impl SafetyAlertResource {
    pub fn builder() -> builder::SafetyAlertResource {
        Default::default()
    }
}
#[doc = "`SafetyAlertResourceObject`"]
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
pub enum SafetyAlertResourceObject {
    #[serde(rename = "safety.alert")]
    SafetyAlert,
}
impl ::std::fmt::Display for SafetyAlertResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SafetyAlert => f.write_str("safety.alert"),
        }
    }
}
impl ::std::str::FromStr for SafetyAlertResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "safety.alert" => Ok(Self::SafetyAlert),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SafetyAlertResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SafetyAlertResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SafetyApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SafetyApiSchema {
    #[serde(rename = "GetprojectsafetyalertError400")]
    pub getprojectsafetyalert_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetprojectsafetyalertError404")]
    pub getprojectsafetyalert_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetprojectsafetyalertError429")]
    pub getprojectsafetyalert_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetprojectsafetyalertError503")]
    pub getprojectsafetyalert_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetprojectsafetyalertParams")]
    pub getprojectsafetyalert_params: GetprojectsafetyalertParams,
    #[serde(rename = "GetprojectsafetyalertResponse")]
    pub getprojectsafetyalert_response: SafetyAlertResource,
    #[serde(rename = "GetsafetycaseError400")]
    pub getsafetycase_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetsafetycaseError404")]
    pub getsafetycase_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetsafetycaseError429")]
    pub getsafetycase_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetsafetycaseError500")]
    pub getsafetycase_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetsafetycaseError503")]
    pub getsafetycase_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetsafetycaseParams")]
    pub getsafetycase_params: GetsafetycaseParams,
    #[serde(rename = "GetsafetycaseResponse")]
    pub getsafetycase_response: SafetyCaseResource,
}
impl SafetyApiSchema {
    pub fn builder() -> builder::SafetyApiSchema {
        Default::default()
    }
}
#[doc = "`SafetyCaseNotice`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SafetyCaseNotice {
    #[serde(rename = "type")]
    pub type_: SafetyCaseNoticeType,
}
impl SafetyCaseNotice {
    pub fn builder() -> builder::SafetyCaseNotice {
        Default::default()
    }
}
#[doc = "`SafetyCaseNoticeType`"]
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
pub enum SafetyCaseNoticeType {
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "deactivation")]
    Deactivation,
}
impl ::std::fmt::Display for SafetyCaseNoticeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Warning => f.write_str("warning"),
            Self::Deactivation => f.write_str("deactivation"),
        }
    }
}
impl ::std::str::FromStr for SafetyCaseNoticeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "warning" => Ok(Self::Warning),
            "deactivation" => Ok(Self::Deactivation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SafetyCaseNoticeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SafetyCaseNoticeType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SafetyCaseResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SafetyCaseResource {
    pub created_at: i64,
    pub entity_identifier: ::std::string::String,
    pub id: ::std::string::String,
    pub notice: SafetyCaseNotice,
    pub object: SafetyCaseResourceObject,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub reason: ::std::option::Option<::std::string::String>,
}
impl SafetyCaseResource {
    pub fn builder() -> builder::SafetyCaseResource {
        Default::default()
    }
}
#[doc = "`SafetyCaseResourceObject`"]
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
pub enum SafetyCaseResourceObject {
    #[serde(rename = "safety.case")]
    SafetyCase,
}
impl ::std::fmt::Display for SafetyCaseResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SafetyCase => f.write_str("safety.case"),
        }
    }
}
impl ::std::str::FromStr for SafetyCaseResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "safety.case" => Ok(Self::SafetyCase),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SafetyCaseResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SafetyCaseResourceObject {
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
    pub struct GetprojectsafetyalertParams {
        id: ::std::result::Result<super::GetprojectsafetyalertParamsId, ::std::string::String>,
    }
    impl ::std::default::Default for GetprojectsafetyalertParams {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl GetprojectsafetyalertParams {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetprojectsafetyalertParamsId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetprojectsafetyalertParams> for super::GetprojectsafetyalertParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetprojectsafetyalertParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::GetprojectsafetyalertParams> for GetprojectsafetyalertParams {
        fn from(value: super::GetprojectsafetyalertParams) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetsafetycaseParams {
        id: ::std::result::Result<super::GetsafetycaseParamsId, ::std::string::String>,
    }
    impl ::std::default::Default for GetsafetycaseParams {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl GetsafetycaseParams {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetsafetycaseParamsId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetsafetycaseParams> for super::GetsafetycaseParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetsafetycaseParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::GetsafetycaseParams> for GetsafetycaseParams {
        fn from(value: super::GetsafetycaseParams) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SafetyAlertResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        detailed_explanation: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        error_type: ::std::result::Result<super::SafetyAlertErrorType, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::SafetyAlertResourceObject, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_paused: ::std::result::Result<bool, ::std::string::String>,
        response_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SafetyAlertResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                detailed_explanation: Ok(Default::default()),
                error_type: Err("no value supplied for error_type".to_string()),
                id: Err("no value supplied for id".to_string()),
                model: Err("no value supplied for model".to_string()),
                object: Err("no value supplied for object".to_string()),
                reason: Err("no value supplied for reason".to_string()),
                request_id: Err("no value supplied for request_id".to_string()),
                request_paused: Err("no value supplied for request_paused".to_string()),
                response_id: Err("no value supplied for response_id".to_string()),
            }
        }
    }
    impl SafetyAlertResource {
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
        pub fn detailed_explanation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.detailed_explanation = value.try_into().map_err(|e| {
                format!("error converting supplied value for detailed_explanation: {e}")
            });
            self
        }
        pub fn error_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SafetyAlertErrorType>,
            T::Error: ::std::fmt::Display,
        {
            self.error_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_type: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SafetyAlertResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_id: {e}"));
            self
        }
        pub fn request_paused<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.request_paused = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_paused: {e}"));
            self
        }
        pub fn response_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.response_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SafetyAlertResource> for super::SafetyAlertResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SafetyAlertResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                detailed_explanation: value.detailed_explanation?,
                error_type: value.error_type?,
                id: value.id?,
                model: value.model?,
                object: value.object?,
                reason: value.reason?,
                request_id: value.request_id?,
                request_paused: value.request_paused?,
                response_id: value.response_id?,
            })
        }
    }
    impl ::std::convert::From<super::SafetyAlertResource> for SafetyAlertResource {
        fn from(value: super::SafetyAlertResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                detailed_explanation: Ok(value.detailed_explanation),
                error_type: Ok(value.error_type),
                id: Ok(value.id),
                model: Ok(value.model),
                object: Ok(value.object),
                reason: Ok(value.reason),
                request_id: Ok(value.request_id),
                request_paused: Ok(value.request_paused),
                response_id: Ok(value.response_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SafetyApiSchema {
        getprojectsafetyalert_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getprojectsafetyalert_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getprojectsafetyalert_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getprojectsafetyalert_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getprojectsafetyalert_params:
            ::std::result::Result<super::GetprojectsafetyalertParams, ::std::string::String>,
        getprojectsafetyalert_response:
            ::std::result::Result<super::SafetyAlertResource, ::std::string::String>,
        getsafetycase_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getsafetycase_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getsafetycase_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getsafetycase_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getsafetycase_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getsafetycase_params:
            ::std::result::Result<super::GetsafetycaseParams, ::std::string::String>,
        getsafetycase_response:
            ::std::result::Result<super::SafetyCaseResource, ::std::string::String>,
    }
    impl ::std::default::Default for SafetyApiSchema {
        fn default() -> Self {
            Self {
                getprojectsafetyalert_error400: Err(
                    "no value supplied for getprojectsafetyalert_error400".to_string(),
                ),
                getprojectsafetyalert_error404: Err(
                    "no value supplied for getprojectsafetyalert_error404".to_string(),
                ),
                getprojectsafetyalert_error429: Err(
                    "no value supplied for getprojectsafetyalert_error429".to_string(),
                ),
                getprojectsafetyalert_error503: Err(
                    "no value supplied for getprojectsafetyalert_error503".to_string(),
                ),
                getprojectsafetyalert_params: Err(
                    "no value supplied for getprojectsafetyalert_params".to_string(),
                ),
                getprojectsafetyalert_response: Err(
                    "no value supplied for getprojectsafetyalert_response".to_string(),
                ),
                getsafetycase_error400: Err(
                    "no value supplied for getsafetycase_error400".to_string()
                ),
                getsafetycase_error404: Err(
                    "no value supplied for getsafetycase_error404".to_string()
                ),
                getsafetycase_error429: Err(
                    "no value supplied for getsafetycase_error429".to_string()
                ),
                getsafetycase_error500: Err(
                    "no value supplied for getsafetycase_error500".to_string()
                ),
                getsafetycase_error503: Err(
                    "no value supplied for getsafetycase_error503".to_string()
                ),
                getsafetycase_params: Err("no value supplied for getsafetycase_params".to_string()),
                getsafetycase_response: Err(
                    "no value supplied for getsafetycase_response".to_string()
                ),
            }
        }
    }
    impl SafetyApiSchema {
        pub fn getprojectsafetyalert_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectsafetyalert_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectsafetyalert_error400: {e}")
            });
            self
        }
        pub fn getprojectsafetyalert_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectsafetyalert_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectsafetyalert_error404: {e}")
            });
            self
        }
        pub fn getprojectsafetyalert_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectsafetyalert_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectsafetyalert_error429: {e}")
            });
            self
        }
        pub fn getprojectsafetyalert_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectsafetyalert_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectsafetyalert_error503: {e}")
            });
            self
        }
        pub fn getprojectsafetyalert_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetprojectsafetyalertParams>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectsafetyalert_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectsafetyalert_params: {e}")
            });
            self
        }
        pub fn getprojectsafetyalert_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SafetyAlertResource>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectsafetyalert_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectsafetyalert_response: {e}")
            });
            self
        }
        pub fn getsafetycase_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getsafetycase_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getsafetycase_error400: {e}")
            });
            self
        }
        pub fn getsafetycase_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getsafetycase_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getsafetycase_error404: {e}")
            });
            self
        }
        pub fn getsafetycase_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getsafetycase_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getsafetycase_error429: {e}")
            });
            self
        }
        pub fn getsafetycase_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getsafetycase_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getsafetycase_error500: {e}")
            });
            self
        }
        pub fn getsafetycase_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getsafetycase_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getsafetycase_error503: {e}")
            });
            self
        }
        pub fn getsafetycase_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetsafetycaseParams>,
            T::Error: ::std::fmt::Display,
        {
            self.getsafetycase_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for getsafetycase_params: {e}")
            });
            self
        }
        pub fn getsafetycase_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SafetyCaseResource>,
            T::Error: ::std::fmt::Display,
        {
            self.getsafetycase_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for getsafetycase_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<SafetyApiSchema> for super::SafetyApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SafetyApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                getprojectsafetyalert_error400: value.getprojectsafetyalert_error400?,
                getprojectsafetyalert_error404: value.getprojectsafetyalert_error404?,
                getprojectsafetyalert_error429: value.getprojectsafetyalert_error429?,
                getprojectsafetyalert_error503: value.getprojectsafetyalert_error503?,
                getprojectsafetyalert_params: value.getprojectsafetyalert_params?,
                getprojectsafetyalert_response: value.getprojectsafetyalert_response?,
                getsafetycase_error400: value.getsafetycase_error400?,
                getsafetycase_error404: value.getsafetycase_error404?,
                getsafetycase_error429: value.getsafetycase_error429?,
                getsafetycase_error500: value.getsafetycase_error500?,
                getsafetycase_error503: value.getsafetycase_error503?,
                getsafetycase_params: value.getsafetycase_params?,
                getsafetycase_response: value.getsafetycase_response?,
            })
        }
    }
    impl ::std::convert::From<super::SafetyApiSchema> for SafetyApiSchema {
        fn from(value: super::SafetyApiSchema) -> Self {
            Self {
                getprojectsafetyalert_error400: Ok(value.getprojectsafetyalert_error400),
                getprojectsafetyalert_error404: Ok(value.getprojectsafetyalert_error404),
                getprojectsafetyalert_error429: Ok(value.getprojectsafetyalert_error429),
                getprojectsafetyalert_error503: Ok(value.getprojectsafetyalert_error503),
                getprojectsafetyalert_params: Ok(value.getprojectsafetyalert_params),
                getprojectsafetyalert_response: Ok(value.getprojectsafetyalert_response),
                getsafetycase_error400: Ok(value.getsafetycase_error400),
                getsafetycase_error404: Ok(value.getsafetycase_error404),
                getsafetycase_error429: Ok(value.getsafetycase_error429),
                getsafetycase_error500: Ok(value.getsafetycase_error500),
                getsafetycase_error503: Ok(value.getsafetycase_error503),
                getsafetycase_params: Ok(value.getsafetycase_params),
                getsafetycase_response: Ok(value.getsafetycase_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SafetyCaseNotice {
        type_: ::std::result::Result<super::SafetyCaseNoticeType, ::std::string::String>,
    }
    impl ::std::default::Default for SafetyCaseNotice {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SafetyCaseNotice {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SafetyCaseNoticeType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SafetyCaseNotice> for super::SafetyCaseNotice {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SafetyCaseNotice,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SafetyCaseNotice> for SafetyCaseNotice {
        fn from(value: super::SafetyCaseNotice) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SafetyCaseResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        entity_identifier: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        notice: ::std::result::Result<super::SafetyCaseNotice, ::std::string::String>,
        object: ::std::result::Result<super::SafetyCaseResourceObject, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SafetyCaseResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                entity_identifier: Err("no value supplied for entity_identifier".to_string()),
                id: Err("no value supplied for id".to_string()),
                notice: Err("no value supplied for notice".to_string()),
                object: Err("no value supplied for object".to_string()),
                reason: Err("no value supplied for reason".to_string()),
            }
        }
    }
    impl SafetyCaseResource {
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
        pub fn entity_identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.entity_identifier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity_identifier: {e}"));
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
        pub fn notice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SafetyCaseNotice>,
            T::Error: ::std::fmt::Display,
        {
            self.notice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notice: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SafetyCaseResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SafetyCaseResource> for super::SafetyCaseResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SafetyCaseResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                entity_identifier: value.entity_identifier?,
                id: value.id?,
                notice: value.notice?,
                object: value.object?,
                reason: value.reason?,
            })
        }
    }
    impl ::std::convert::From<super::SafetyCaseResource> for SafetyCaseResource {
        fn from(value: super::SafetyCaseResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                entity_identifier: Ok(value.entity_identifier),
                id: Ok(value.id),
                notice: Ok(value.notice),
                object: Ok(value.object),
                reason: Ok(value.reason),
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
