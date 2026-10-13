pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "Request payload for creating a new group in the organization."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateGroupBody {
    #[doc = "Human readable name for the group."]
    pub name: CreateGroupBodyName,
}
impl CreateGroupBody {
    pub fn builder() -> builder::CreateGroupBody {
        Default::default()
    }
}
#[doc = "Human readable name for the group."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateGroupBodyName(::std::string::String);
impl ::std::ops::Deref for CreateGroupBodyName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateGroupBodyName> for ::std::string::String {
    fn from(value: CreateGroupBodyName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateGroupBodyName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateGroupBodyName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateGroupBodyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateGroupBodyName {
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
#[doc = "`DeleteGroupParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteGroupParams {
    pub group_id: ::std::string::String,
}
impl DeleteGroupParams {
    pub fn builder() -> builder::DeleteGroupParams {
        Default::default()
    }
}
#[doc = "Confirmation payload returned after deleting a group."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupDeletedResource {
    #[doc = "Whether the group was deleted."]
    pub deleted: bool,
    #[doc = "Identifier of the deleted group."]
    pub id: ::std::string::String,
    #[doc = "Always `group.deleted`."]
    pub object: GroupDeletedResourceObject,
}
impl GroupDeletedResource {
    pub fn builder() -> builder::GroupDeletedResource {
        Default::default()
    }
}
#[doc = "Always `group.deleted`."]
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
pub enum GroupDeletedResourceObject {
    #[serde(rename = "group.deleted")]
    GroupDeleted,
}
impl ::std::fmt::Display for GroupDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::GroupDeleted => f.write_str("group.deleted"),
        }
    }
}
impl ::std::str::FromStr for GroupDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "group.deleted" => Ok(Self::GroupDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GroupDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GroupDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Paginated list of organization groups."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupListResource {
    #[doc = "Groups returned in the current page."]
    pub data: ::std::vec::Vec<GroupResponse>,
    #[doc = "Whether additional groups are available when paginating."]
    pub has_more: bool,
    #[doc = "Cursor to fetch the next page of results, or `null` if there are no more results."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub next: ::std::option::Option<::std::string::String>,
    #[doc = "Always `list`."]
    pub object: GroupListResourceObject,
}
impl GroupListResource {
    pub fn builder() -> builder::GroupListResource {
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
pub enum GroupListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for GroupListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for GroupListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GroupListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GroupListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Response returned after updating a group."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupResourceWithSuccess {
    #[doc = "Unix timestamp (in seconds) when the group was created."]
    pub created_at: i64,
    #[doc = "Identifier for the group."]
    pub id: ::std::string::String,
    #[doc = "Whether the group is managed through SCIM and controlled by your identity provider."]
    pub is_scim_managed: bool,
    #[doc = "Updated display name for the group."]
    pub name: ::std::string::String,
}
impl GroupResourceWithSuccess {
    pub fn builder() -> builder::GroupResourceWithSuccess {
        Default::default()
    }
}
#[doc = "Details about an organization group."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupResponse {
    #[doc = "Unix timestamp (in seconds) when the group was created."]
    pub created_at: i64,
    #[doc = "The type of the group."]
    pub group_type: GroupResponseGroupType,
    #[doc = "Identifier for the group."]
    pub id: ::std::string::String,
    #[doc = "Whether the group is managed through SCIM and controlled by your identity provider."]
    pub is_scim_managed: bool,
    #[doc = "Display name of the group."]
    pub name: ::std::string::String,
}
impl GroupResponse {
    pub fn builder() -> builder::GroupResponse {
        Default::default()
    }
}
#[doc = "The type of the group."]
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
pub enum GroupResponseGroupType {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "tenant_group")]
    TenantGroup,
}
impl ::std::fmt::Display for GroupResponseGroupType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Group => f.write_str("group"),
            Self::TenantGroup => f.write_str("tenant_group"),
        }
    }
}
impl ::std::str::FromStr for GroupResponseGroupType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "group" => Ok(Self::Group),
            "tenant_group" => Ok(Self::TenantGroup),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GroupResponseGroupType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GroupResponseGroupType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GroupsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupsApiSchema {
    #[serde(rename = "CreateGroupError400")]
    pub create_group_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateGroupError403")]
    pub create_group_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "CreateGroupError404")]
    pub create_group_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateGroupError422")]
    pub create_group_error422: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateGroupError429")]
    pub create_group_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateGroupError500")]
    pub create_group_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateGroupRequest")]
    pub create_group_request: CreateGroupBody,
    #[serde(rename = "CreateGroupResponse")]
    pub create_group_response: GroupResponse,
    #[serde(rename = "DeleteGroupError400")]
    pub delete_group_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteGroupError403")]
    pub delete_group_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteGroupError404")]
    pub delete_group_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteGroupError429")]
    pub delete_group_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteGroupParams")]
    pub delete_group_params: DeleteGroupParams,
    #[serde(rename = "DeleteGroupResponse")]
    pub delete_group_response: GroupDeletedResource,
    #[serde(rename = "ListGroupsError400")]
    pub list_groups_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupsError403")]
    pub list_groups_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListGroupsError404")]
    pub list_groups_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupsError429")]
    pub list_groups_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupsParams")]
    pub list_groups_params: ListGroupsParams,
    #[serde(rename = "ListGroupsResponse")]
    pub list_groups_response: GroupListResource,
    #[serde(rename = "RetrieveGroupError400")]
    pub retrieve_group_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupError403")]
    pub retrieve_group_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveGroupError404")]
    pub retrieve_group_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupError429")]
    pub retrieve_group_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupError500")]
    pub retrieve_group_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupParams")]
    pub retrieve_group_params: RetrieveGroupParams,
    #[serde(rename = "RetrieveGroupResponse")]
    pub retrieve_group_response: GroupResponse,
    #[serde(rename = "UpdateGroupError400")]
    pub update_group_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateGroupError403")]
    pub update_group_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateGroupError404")]
    pub update_group_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateGroupError429")]
    pub update_group_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateGroupError500")]
    pub update_group_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateGroupParams")]
    pub update_group_params: UpdateGroupParams,
    #[serde(rename = "UpdateGroupRequest")]
    pub update_group_request: UpdateGroupBody,
    #[serde(rename = "UpdateGroupResponse")]
    pub update_group_response: GroupResourceWithSuccess,
}
impl GroupsApiSchema {
    pub fn builder() -> builder::GroupsApiSchema {
        Default::default()
    }
}
#[doc = "`ListGroupsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListGroupsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListGroupsParamsOrder>,
}
impl ListGroupsParams {
    pub fn builder() -> builder::ListGroupsParams {
        Default::default()
    }
}
#[doc = "`ListGroupsParamsOrder`"]
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
pub enum ListGroupsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListGroupsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListGroupsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListGroupsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListGroupsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RetrieveGroupParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveGroupParams {
    pub group_id: ::std::string::String,
}
impl RetrieveGroupParams {
    pub fn builder() -> builder::RetrieveGroupParams {
        Default::default()
    }
}
#[doc = "Request payload for updating the details of an existing group."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateGroupBody {
    #[doc = "New display name for the group."]
    pub name: UpdateGroupBodyName,
}
impl UpdateGroupBody {
    pub fn builder() -> builder::UpdateGroupBody {
        Default::default()
    }
}
#[doc = "New display name for the group."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UpdateGroupBodyName(::std::string::String);
impl ::std::ops::Deref for UpdateGroupBodyName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UpdateGroupBodyName> for ::std::string::String {
    fn from(value: UpdateGroupBodyName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UpdateGroupBodyName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UpdateGroupBodyName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateGroupBodyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UpdateGroupBodyName {
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
#[doc = "`UpdateGroupParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateGroupParams {
    pub group_id: ::std::string::String,
}
impl UpdateGroupParams {
    pub fn builder() -> builder::UpdateGroupParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateGroupBody {
        name: ::std::result::Result<super::CreateGroupBodyName, ::std::string::String>,
    }
    impl ::std::default::Default for CreateGroupBody {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl CreateGroupBody {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateGroupBodyName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateGroupBody> for super::CreateGroupBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateGroupBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::CreateGroupBody> for CreateGroupBody {
        fn from(value: super::CreateGroupBody) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteGroupParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteGroupParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
            }
        }
    }
    impl DeleteGroupParams {
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteGroupParams> for super::DeleteGroupParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteGroupParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteGroupParams> for DeleteGroupParams {
        fn from(value: super::DeleteGroupParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::GroupDeletedResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for GroupDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl GroupDeletedResource {
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
            T: ::std::convert::TryInto<super::GroupDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupDeletedResource> for super::GroupDeletedResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::GroupDeletedResource> for GroupDeletedResource {
        fn from(value: super::GroupDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupListResource {
        data: ::std::result::Result<::std::vec::Vec<super::GroupResponse>, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        next: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::GroupListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for GroupListResource {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                next: Err("no value supplied for next".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl GroupListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::GroupResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupListResource> for super::GroupListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupListResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                next: value.next?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::GroupListResource> for GroupListResource {
        fn from(value: super::GroupListResource) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                next: Ok(value.next),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupResourceWithSuccess {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        is_scim_managed: ::std::result::Result<bool, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GroupResourceWithSuccess {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                is_scim_managed: Err("no value supplied for is_scim_managed".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl GroupResourceWithSuccess {
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
        pub fn is_scim_managed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_scim_managed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_scim_managed: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupResourceWithSuccess> for super::GroupResourceWithSuccess {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupResourceWithSuccess,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                is_scim_managed: value.is_scim_managed?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::GroupResourceWithSuccess> for GroupResourceWithSuccess {
        fn from(value: super::GroupResourceWithSuccess) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                is_scim_managed: Ok(value.is_scim_managed),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupResponse {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        group_type: ::std::result::Result<super::GroupResponseGroupType, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        is_scim_managed: ::std::result::Result<bool, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GroupResponse {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                group_type: Err("no value supplied for group_type".to_string()),
                id: Err("no value supplied for id".to_string()),
                is_scim_managed: Err("no value supplied for is_scim_managed".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl GroupResponse {
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
        pub fn group_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupResponseGroupType>,
            T::Error: ::std::fmt::Display,
        {
            self.group_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_type: {e}"));
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
        pub fn is_scim_managed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_scim_managed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_scim_managed: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupResponse> for super::GroupResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                group_type: value.group_type?,
                id: value.id?,
                is_scim_managed: value.is_scim_managed?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::GroupResponse> for GroupResponse {
        fn from(value: super::GroupResponse) -> Self {
            Self {
                created_at: Ok(value.created_at),
                group_type: Ok(value.group_type),
                id: Ok(value.id),
                is_scim_managed: Ok(value.is_scim_managed),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupsApiSchema {
        create_group_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_group_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        create_group_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_group_error422:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_group_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_group_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_group_request: ::std::result::Result<super::CreateGroupBody, ::std::string::String>,
        create_group_response: ::std::result::Result<super::GroupResponse, ::std::string::String>,
        delete_group_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_group_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_group_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_group_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_group_params: ::std::result::Result<super::DeleteGroupParams, ::std::string::String>,
        delete_group_response:
            ::std::result::Result<super::GroupDeletedResource, ::std::string::String>,
        list_groups_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_groups_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_groups_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_groups_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_groups_params: ::std::result::Result<super::ListGroupsParams, ::std::string::String>,
        list_groups_response:
            ::std::result::Result<super::GroupListResource, ::std::string::String>,
        retrieve_group_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_group_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_params:
            ::std::result::Result<super::RetrieveGroupParams, ::std::string::String>,
        retrieve_group_response: ::std::result::Result<super::GroupResponse, ::std::string::String>,
        update_group_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_group_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_group_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_group_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_group_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_group_params: ::std::result::Result<super::UpdateGroupParams, ::std::string::String>,
        update_group_request: ::std::result::Result<super::UpdateGroupBody, ::std::string::String>,
        update_group_response:
            ::std::result::Result<super::GroupResourceWithSuccess, ::std::string::String>,
    }
    impl ::std::default::Default for GroupsApiSchema {
        fn default() -> Self {
            Self {
                create_group_error400: Err(
                    "no value supplied for create_group_error400".to_string()
                ),
                create_group_error403: Err(
                    "no value supplied for create_group_error403".to_string()
                ),
                create_group_error404: Err(
                    "no value supplied for create_group_error404".to_string()
                ),
                create_group_error422: Err(
                    "no value supplied for create_group_error422".to_string()
                ),
                create_group_error429: Err(
                    "no value supplied for create_group_error429".to_string()
                ),
                create_group_error500: Err(
                    "no value supplied for create_group_error500".to_string()
                ),
                create_group_request: Err("no value supplied for create_group_request".to_string()),
                create_group_response: Err(
                    "no value supplied for create_group_response".to_string()
                ),
                delete_group_error400: Err(
                    "no value supplied for delete_group_error400".to_string()
                ),
                delete_group_error403: Err(
                    "no value supplied for delete_group_error403".to_string()
                ),
                delete_group_error404: Err(
                    "no value supplied for delete_group_error404".to_string()
                ),
                delete_group_error429: Err(
                    "no value supplied for delete_group_error429".to_string()
                ),
                delete_group_params: Err("no value supplied for delete_group_params".to_string()),
                delete_group_response: Err(
                    "no value supplied for delete_group_response".to_string()
                ),
                list_groups_error400: Err("no value supplied for list_groups_error400".to_string()),
                list_groups_error403: Err("no value supplied for list_groups_error403".to_string()),
                list_groups_error404: Err("no value supplied for list_groups_error404".to_string()),
                list_groups_error429: Err("no value supplied for list_groups_error429".to_string()),
                list_groups_params: Err("no value supplied for list_groups_params".to_string()),
                list_groups_response: Err("no value supplied for list_groups_response".to_string()),
                retrieve_group_error400: Err(
                    "no value supplied for retrieve_group_error400".to_string()
                ),
                retrieve_group_error403: Err(
                    "no value supplied for retrieve_group_error403".to_string()
                ),
                retrieve_group_error404: Err(
                    "no value supplied for retrieve_group_error404".to_string()
                ),
                retrieve_group_error429: Err(
                    "no value supplied for retrieve_group_error429".to_string()
                ),
                retrieve_group_error500: Err(
                    "no value supplied for retrieve_group_error500".to_string()
                ),
                retrieve_group_params: Err(
                    "no value supplied for retrieve_group_params".to_string()
                ),
                retrieve_group_response: Err(
                    "no value supplied for retrieve_group_response".to_string()
                ),
                update_group_error400: Err(
                    "no value supplied for update_group_error400".to_string()
                ),
                update_group_error403: Err(
                    "no value supplied for update_group_error403".to_string()
                ),
                update_group_error404: Err(
                    "no value supplied for update_group_error404".to_string()
                ),
                update_group_error429: Err(
                    "no value supplied for update_group_error429".to_string()
                ),
                update_group_error500: Err(
                    "no value supplied for update_group_error500".to_string()
                ),
                update_group_params: Err("no value supplied for update_group_params".to_string()),
                update_group_request: Err("no value supplied for update_group_request".to_string()),
                update_group_response: Err(
                    "no value supplied for update_group_response".to_string()
                ),
            }
        }
    }
    impl GroupsApiSchema {
        pub fn create_group_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_group_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_group_error400: {e}")
            });
            self
        }
        pub fn create_group_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_group_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_group_error403: {e}")
            });
            self
        }
        pub fn create_group_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_group_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_group_error404: {e}")
            });
            self
        }
        pub fn create_group_error422<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_group_error422 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_group_error422: {e}")
            });
            self
        }
        pub fn create_group_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_group_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_group_error429: {e}")
            });
            self
        }
        pub fn create_group_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_group_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_group_error500: {e}")
            });
            self
        }
        pub fn create_group_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateGroupBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_group_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_group_request: {e}")
            });
            self
        }
        pub fn create_group_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_group_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_group_response: {e}")
            });
            self
        }
        pub fn delete_group_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_group_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_group_error400: {e}")
            });
            self
        }
        pub fn delete_group_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_group_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_group_error403: {e}")
            });
            self
        }
        pub fn delete_group_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_group_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_group_error404: {e}")
            });
            self
        }
        pub fn delete_group_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_group_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_group_error429: {e}")
            });
            self
        }
        pub fn delete_group_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteGroupParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_group_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_group_params: {e}")
            });
            self
        }
        pub fn delete_group_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_group_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_group_response: {e}")
            });
            self
        }
        pub fn list_groups_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_groups_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_groups_error400: {e}")
            });
            self
        }
        pub fn list_groups_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_groups_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_groups_error403: {e}")
            });
            self
        }
        pub fn list_groups_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_groups_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_groups_error404: {e}")
            });
            self
        }
        pub fn list_groups_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_groups_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_groups_error429: {e}")
            });
            self
        }
        pub fn list_groups_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListGroupsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_groups_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_groups_params: {e}")
            });
            self
        }
        pub fn list_groups_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_groups_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_groups_response: {e}")
            });
            self
        }
        pub fn retrieve_group_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_error400: {e}")
            });
            self
        }
        pub fn retrieve_group_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_error403: {e}")
            });
            self
        }
        pub fn retrieve_group_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_error404: {e}")
            });
            self
        }
        pub fn retrieve_group_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_error429: {e}")
            });
            self
        }
        pub fn retrieve_group_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_error500: {e}")
            });
            self
        }
        pub fn retrieve_group_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveGroupParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_params: {e}")
            });
            self
        }
        pub fn retrieve_group_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_response: {e}")
            });
            self
        }
        pub fn update_group_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_group_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_group_error400: {e}")
            });
            self
        }
        pub fn update_group_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_group_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_group_error403: {e}")
            });
            self
        }
        pub fn update_group_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_group_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_group_error404: {e}")
            });
            self
        }
        pub fn update_group_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_group_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_group_error429: {e}")
            });
            self
        }
        pub fn update_group_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_group_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_group_error500: {e}")
            });
            self
        }
        pub fn update_group_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateGroupParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_group_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_group_params: {e}")
            });
            self
        }
        pub fn update_group_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateGroupBody>,
            T::Error: ::std::fmt::Display,
        {
            self.update_group_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_group_request: {e}")
            });
            self
        }
        pub fn update_group_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupResourceWithSuccess>,
            T::Error: ::std::fmt::Display,
        {
            self.update_group_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_group_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<GroupsApiSchema> for super::GroupsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_group_error400: value.create_group_error400?,
                create_group_error403: value.create_group_error403?,
                create_group_error404: value.create_group_error404?,
                create_group_error422: value.create_group_error422?,
                create_group_error429: value.create_group_error429?,
                create_group_error500: value.create_group_error500?,
                create_group_request: value.create_group_request?,
                create_group_response: value.create_group_response?,
                delete_group_error400: value.delete_group_error400?,
                delete_group_error403: value.delete_group_error403?,
                delete_group_error404: value.delete_group_error404?,
                delete_group_error429: value.delete_group_error429?,
                delete_group_params: value.delete_group_params?,
                delete_group_response: value.delete_group_response?,
                list_groups_error400: value.list_groups_error400?,
                list_groups_error403: value.list_groups_error403?,
                list_groups_error404: value.list_groups_error404?,
                list_groups_error429: value.list_groups_error429?,
                list_groups_params: value.list_groups_params?,
                list_groups_response: value.list_groups_response?,
                retrieve_group_error400: value.retrieve_group_error400?,
                retrieve_group_error403: value.retrieve_group_error403?,
                retrieve_group_error404: value.retrieve_group_error404?,
                retrieve_group_error429: value.retrieve_group_error429?,
                retrieve_group_error500: value.retrieve_group_error500?,
                retrieve_group_params: value.retrieve_group_params?,
                retrieve_group_response: value.retrieve_group_response?,
                update_group_error400: value.update_group_error400?,
                update_group_error403: value.update_group_error403?,
                update_group_error404: value.update_group_error404?,
                update_group_error429: value.update_group_error429?,
                update_group_error500: value.update_group_error500?,
                update_group_params: value.update_group_params?,
                update_group_request: value.update_group_request?,
                update_group_response: value.update_group_response?,
            })
        }
    }
    impl ::std::convert::From<super::GroupsApiSchema> for GroupsApiSchema {
        fn from(value: super::GroupsApiSchema) -> Self {
            Self {
                create_group_error400: Ok(value.create_group_error400),
                create_group_error403: Ok(value.create_group_error403),
                create_group_error404: Ok(value.create_group_error404),
                create_group_error422: Ok(value.create_group_error422),
                create_group_error429: Ok(value.create_group_error429),
                create_group_error500: Ok(value.create_group_error500),
                create_group_request: Ok(value.create_group_request),
                create_group_response: Ok(value.create_group_response),
                delete_group_error400: Ok(value.delete_group_error400),
                delete_group_error403: Ok(value.delete_group_error403),
                delete_group_error404: Ok(value.delete_group_error404),
                delete_group_error429: Ok(value.delete_group_error429),
                delete_group_params: Ok(value.delete_group_params),
                delete_group_response: Ok(value.delete_group_response),
                list_groups_error400: Ok(value.list_groups_error400),
                list_groups_error403: Ok(value.list_groups_error403),
                list_groups_error404: Ok(value.list_groups_error404),
                list_groups_error429: Ok(value.list_groups_error429),
                list_groups_params: Ok(value.list_groups_params),
                list_groups_response: Ok(value.list_groups_response),
                retrieve_group_error400: Ok(value.retrieve_group_error400),
                retrieve_group_error403: Ok(value.retrieve_group_error403),
                retrieve_group_error404: Ok(value.retrieve_group_error404),
                retrieve_group_error429: Ok(value.retrieve_group_error429),
                retrieve_group_error500: Ok(value.retrieve_group_error500),
                retrieve_group_params: Ok(value.retrieve_group_params),
                retrieve_group_response: Ok(value.retrieve_group_response),
                update_group_error400: Ok(value.update_group_error400),
                update_group_error403: Ok(value.update_group_error403),
                update_group_error404: Ok(value.update_group_error404),
                update_group_error429: Ok(value.update_group_error429),
                update_group_error500: Ok(value.update_group_error500),
                update_group_params: Ok(value.update_group_params),
                update_group_request: Ok(value.update_group_request),
                update_group_response: Ok(value.update_group_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListGroupsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListGroupsParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListGroupsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListGroupsParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListGroupsParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListGroupsParams> for super::ListGroupsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListGroupsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListGroupsParams> for ListGroupsParams {
        fn from(value: super::ListGroupsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveGroupParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveGroupParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
            }
        }
    }
    impl RetrieveGroupParams {
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveGroupParams> for super::RetrieveGroupParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveGroupParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveGroupParams> for RetrieveGroupParams {
        fn from(value: super::RetrieveGroupParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateGroupBody {
        name: ::std::result::Result<super::UpdateGroupBodyName, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateGroupBody {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl UpdateGroupBody {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateGroupBodyName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateGroupBody> for super::UpdateGroupBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateGroupBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::UpdateGroupBody> for UpdateGroupBody {
        fn from(value: super::UpdateGroupBody) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateGroupParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateGroupParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
            }
        }
    }
    impl UpdateGroupParams {
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateGroupParams> for super::UpdateGroupParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateGroupParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateGroupParams> for UpdateGroupParams {
        fn from(value: super::UpdateGroupParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
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
