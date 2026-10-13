pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`AddProjectGroupParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AddProjectGroupParams {
    pub project_id: ::std::string::String,
}
impl AddProjectGroupParams {
    pub fn builder() -> builder::AddProjectGroupParams {
        Default::default()
    }
}
#[doc = "Request payload for granting a group access to a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InviteProjectGroupBody {
    #[doc = "Identifier of the group to add to the project."]
    pub group_id: ::std::string::String,
    #[doc = "Identifier of the project role to grant to the group."]
    pub role: ::std::string::String,
}
impl InviteProjectGroupBody {
    pub fn builder() -> builder::InviteProjectGroupBody {
        Default::default()
    }
}
#[doc = "`ListProjectGroupsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectGroupsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListProjectGroupsParamsOrder>,
    pub project_id: ::std::string::String,
}
impl ListProjectGroupsParams {
    pub fn builder() -> builder::ListProjectGroupsParams {
        Default::default()
    }
}
#[doc = "`ListProjectGroupsParamsOrder`"]
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
pub enum ListProjectGroupsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListProjectGroupsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListProjectGroupsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListProjectGroupsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListProjectGroupsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details about a group's membership in a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectGroup {
    #[doc = "Unix timestamp (in seconds) when the group was granted project access."]
    pub created_at: i64,
    #[doc = "Identifier of the group that has access to the project."]
    pub group_id: ::std::string::String,
    #[doc = "Display name of the group."]
    pub group_name: ::std::string::String,
    #[doc = "The type of the group."]
    pub group_type: ProjectGroupGroupType,
    #[doc = "Always `project.group`."]
    pub object: ProjectGroupObject,
    #[doc = "Identifier of the project."]
    pub project_id: ::std::string::String,
}
impl ProjectGroup {
    pub fn builder() -> builder::ProjectGroup {
        Default::default()
    }
}
#[doc = "Confirmation payload returned after removing a group from a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectGroupDeletedResource {
    #[doc = "Whether the group membership in the project was removed."]
    pub deleted: bool,
    #[doc = "Always `project.group.deleted`."]
    pub object: ProjectGroupDeletedResourceObject,
}
impl ProjectGroupDeletedResource {
    pub fn builder() -> builder::ProjectGroupDeletedResource {
        Default::default()
    }
}
#[doc = "Always `project.group.deleted`."]
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
pub enum ProjectGroupDeletedResourceObject {
    #[serde(rename = "project.group.deleted")]
    ProjectGroupDeleted,
}
impl ::std::fmt::Display for ProjectGroupDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectGroupDeleted => f.write_str("project.group.deleted"),
        }
    }
}
impl ::std::str::FromStr for ProjectGroupDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.group.deleted" => Ok(Self::ProjectGroupDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectGroupDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectGroupDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
pub enum ProjectGroupGroupType {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "tenant_group")]
    TenantGroup,
}
impl ::std::fmt::Display for ProjectGroupGroupType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Group => f.write_str("group"),
            Self::TenantGroup => f.write_str("tenant_group"),
        }
    }
}
impl ::std::str::FromStr for ProjectGroupGroupType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "group" => Ok(Self::Group),
            "tenant_group" => Ok(Self::TenantGroup),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectGroupGroupType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectGroupGroupType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Paginated list of groups that have access to a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectGroupListResource {
    #[doc = "Project group memberships returned in the current page."]
    pub data: ::std::vec::Vec<ProjectGroup>,
    #[doc = "Whether additional project group memberships are available."]
    pub has_more: bool,
    #[doc = "Cursor to fetch the next page of results, or `null` when there are no more results."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub next: ::std::option::Option<::std::string::String>,
    #[doc = "Always `list`."]
    pub object: ProjectGroupListResourceObject,
}
impl ProjectGroupListResource {
    pub fn builder() -> builder::ProjectGroupListResource {
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
pub enum ProjectGroupListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ProjectGroupListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ProjectGroupListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectGroupListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectGroupListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Always `project.group`."]
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
pub enum ProjectGroupObject {
    #[serde(rename = "project.group")]
    ProjectGroup,
}
impl ::std::fmt::Display for ProjectGroupObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectGroup => f.write_str("project.group"),
        }
    }
}
impl ::std::str::FromStr for ProjectGroupObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.group" => Ok(Self::ProjectGroup),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectGroupObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectGroupObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectGroupsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectGroupsApiSchema {
    #[serde(rename = "AddProjectGroupError400")]
    pub add_project_group_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddProjectGroupError403")]
    pub add_project_group_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AddProjectGroupError404")]
    pub add_project_group_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddProjectGroupError429")]
    pub add_project_group_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddProjectGroupError500")]
    pub add_project_group_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddProjectGroupParams")]
    pub add_project_group_params: AddProjectGroupParams,
    #[serde(rename = "AddProjectGroupRequest")]
    pub add_project_group_request: InviteProjectGroupBody,
    #[serde(rename = "AddProjectGroupResponse")]
    pub add_project_group_response: ProjectGroup,
    #[serde(rename = "ListProjectGroupsError400")]
    pub list_project_groups_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectGroupsError403")]
    pub list_project_groups_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectGroupsError404")]
    pub list_project_groups_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectGroupsError429")]
    pub list_project_groups_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectGroupsParams")]
    pub list_project_groups_params: ListProjectGroupsParams,
    #[serde(rename = "ListProjectGroupsResponse")]
    pub list_project_groups_response: ProjectGroupListResource,
    #[serde(rename = "RemoveProjectGroupError400")]
    pub remove_project_group_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RemoveProjectGroupError403")]
    pub remove_project_group_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RemoveProjectGroupError404")]
    pub remove_project_group_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RemoveProjectGroupError429")]
    pub remove_project_group_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RemoveProjectGroupParams")]
    pub remove_project_group_params: RemoveProjectGroupParams,
    #[serde(rename = "RemoveProjectGroupResponse")]
    pub remove_project_group_response: ProjectGroupDeletedResource,
    #[serde(rename = "RetrieveProjectGroupError400")]
    pub retrieve_project_group_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectGroupError403")]
    pub retrieve_project_group_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectGroupError404")]
    pub retrieve_project_group_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectGroupError429")]
    pub retrieve_project_group_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectGroupParams")]
    pub retrieve_project_group_params: RetrieveProjectGroupParams,
    #[serde(rename = "RetrieveProjectGroupResponse")]
    pub retrieve_project_group_response: ProjectGroup,
}
impl ProjectGroupsApiSchema {
    pub fn builder() -> builder::ProjectGroupsApiSchema {
        Default::default()
    }
}
#[doc = "`RemoveProjectGroupParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RemoveProjectGroupParams {
    pub group_id: ::std::string::String,
    pub project_id: ::std::string::String,
}
impl RemoveProjectGroupParams {
    pub fn builder() -> builder::RemoveProjectGroupParams {
        Default::default()
    }
}
#[doc = "`RetrieveProjectGroupParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectGroupParams {
    pub group_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub group_type: ::std::option::Option<RetrieveProjectGroupParamsGroupType>,
    pub project_id: ::std::string::String,
}
impl RetrieveProjectGroupParams {
    pub fn builder() -> builder::RetrieveProjectGroupParams {
        Default::default()
    }
}
#[doc = "`RetrieveProjectGroupParamsGroupType`"]
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
pub enum RetrieveProjectGroupParamsGroupType {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "tenant_group")]
    TenantGroup,
}
impl ::std::fmt::Display for RetrieveProjectGroupParamsGroupType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Group => f.write_str("group"),
            Self::TenantGroup => f.write_str("tenant_group"),
        }
    }
}
impl ::std::str::FromStr for RetrieveProjectGroupParamsGroupType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "group" => Ok(Self::Group),
            "tenant_group" => Ok(Self::TenantGroup),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RetrieveProjectGroupParamsGroupType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RetrieveProjectGroupParamsGroupType {
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
    pub struct AddProjectGroupParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AddProjectGroupParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl AddProjectGroupParams {
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
    impl ::std::convert::TryFrom<AddProjectGroupParams> for super::AddProjectGroupParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AddProjectGroupParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::AddProjectGroupParams> for AddProjectGroupParams {
        fn from(value: super::AddProjectGroupParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InviteProjectGroupBody {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InviteProjectGroupBody {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl InviteProjectGroupBody {
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InviteProjectGroupBody> for super::InviteProjectGroupBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InviteProjectGroupBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::InviteProjectGroupBody> for InviteProjectGroupBody {
        fn from(value: super::InviteProjectGroupBody) -> Self {
            Self {
                group_id: Ok(value.group_id),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectGroupsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListProjectGroupsParamsOrder>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectGroupsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ListProjectGroupsParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListProjectGroupsParamsOrder>>,
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
    impl ::std::convert::TryFrom<ListProjectGroupsParams> for super::ListProjectGroupsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectGroupsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectGroupsParams> for ListProjectGroupsParams {
        fn from(value: super::ListProjectGroupsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectGroup {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        group_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        group_type: ::std::result::Result<super::ProjectGroupGroupType, ::std::string::String>,
        object: ::std::result::Result<super::ProjectGroupObject, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectGroup {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                group_id: Err("no value supplied for group_id".to_string()),
                group_name: Err("no value supplied for group_name".to_string()),
                group_type: Err("no value supplied for group_type".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ProjectGroup {
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
        pub fn group_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.group_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_name: {e}"));
            self
        }
        pub fn group_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectGroupGroupType>,
            T::Error: ::std::fmt::Display,
        {
            self.group_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_type: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectGroupObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
    impl ::std::convert::TryFrom<ProjectGroup> for super::ProjectGroup {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectGroup,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                group_id: value.group_id?,
                group_name: value.group_name?,
                group_type: value.group_type?,
                object: value.object?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectGroup> for ProjectGroup {
        fn from(value: super::ProjectGroup) -> Self {
            Self {
                created_at: Ok(value.created_at),
                group_id: Ok(value.group_id),
                group_name: Ok(value.group_name),
                group_type: Ok(value.group_type),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectGroupDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        object:
            ::std::result::Result<super::ProjectGroupDeletedResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectGroupDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectGroupDeletedResource {
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectGroupDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectGroupDeletedResource> for super::ProjectGroupDeletedResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectGroupDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectGroupDeletedResource> for ProjectGroupDeletedResource {
        fn from(value: super::ProjectGroupDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectGroupListResource {
        data: ::std::result::Result<::std::vec::Vec<super::ProjectGroup>, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        next: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ProjectGroupListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectGroupListResource {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                next: Err("no value supplied for next".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectGroupListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProjectGroup>>,
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
            T: ::std::convert::TryInto<super::ProjectGroupListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectGroupListResource> for super::ProjectGroupListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectGroupListResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                next: value.next?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectGroupListResource> for ProjectGroupListResource {
        fn from(value: super::ProjectGroupListResource) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                next: Ok(value.next),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectGroupsApiSchema {
        add_project_group_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_project_group_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        add_project_group_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_project_group_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_project_group_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_project_group_params:
            ::std::result::Result<super::AddProjectGroupParams, ::std::string::String>,
        add_project_group_request:
            ::std::result::Result<super::InviteProjectGroupBody, ::std::string::String>,
        add_project_group_response:
            ::std::result::Result<super::ProjectGroup, ::std::string::String>,
        list_project_groups_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_groups_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_groups_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_groups_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_groups_params:
            ::std::result::Result<super::ListProjectGroupsParams, ::std::string::String>,
        list_project_groups_response:
            ::std::result::Result<super::ProjectGroupListResource, ::std::string::String>,
        remove_project_group_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        remove_project_group_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        remove_project_group_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        remove_project_group_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        remove_project_group_params:
            ::std::result::Result<super::RemoveProjectGroupParams, ::std::string::String>,
        remove_project_group_response:
            ::std::result::Result<super::ProjectGroupDeletedResource, ::std::string::String>,
        retrieve_project_group_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_group_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_group_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_group_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_group_params:
            ::std::result::Result<super::RetrieveProjectGroupParams, ::std::string::String>,
        retrieve_project_group_response:
            ::std::result::Result<super::ProjectGroup, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectGroupsApiSchema {
        fn default() -> Self {
            Self {
                add_project_group_error400: Err(
                    "no value supplied for add_project_group_error400".to_string()
                ),
                add_project_group_error403: Err(
                    "no value supplied for add_project_group_error403".to_string()
                ),
                add_project_group_error404: Err(
                    "no value supplied for add_project_group_error404".to_string()
                ),
                add_project_group_error429: Err(
                    "no value supplied for add_project_group_error429".to_string()
                ),
                add_project_group_error500: Err(
                    "no value supplied for add_project_group_error500".to_string()
                ),
                add_project_group_params: Err(
                    "no value supplied for add_project_group_params".to_string()
                ),
                add_project_group_request: Err(
                    "no value supplied for add_project_group_request".to_string()
                ),
                add_project_group_response: Err(
                    "no value supplied for add_project_group_response".to_string()
                ),
                list_project_groups_error400: Err(
                    "no value supplied for list_project_groups_error400".to_string(),
                ),
                list_project_groups_error403: Err(
                    "no value supplied for list_project_groups_error403".to_string(),
                ),
                list_project_groups_error404: Err(
                    "no value supplied for list_project_groups_error404".to_string(),
                ),
                list_project_groups_error429: Err(
                    "no value supplied for list_project_groups_error429".to_string(),
                ),
                list_project_groups_params: Err(
                    "no value supplied for list_project_groups_params".to_string()
                ),
                list_project_groups_response: Err(
                    "no value supplied for list_project_groups_response".to_string(),
                ),
                remove_project_group_error400: Err(
                    "no value supplied for remove_project_group_error400".to_string(),
                ),
                remove_project_group_error403: Err(
                    "no value supplied for remove_project_group_error403".to_string(),
                ),
                remove_project_group_error404: Err(
                    "no value supplied for remove_project_group_error404".to_string(),
                ),
                remove_project_group_error429: Err(
                    "no value supplied for remove_project_group_error429".to_string(),
                ),
                remove_project_group_params: Err(
                    "no value supplied for remove_project_group_params".to_string(),
                ),
                remove_project_group_response: Err(
                    "no value supplied for remove_project_group_response".to_string(),
                ),
                retrieve_project_group_error400: Err(
                    "no value supplied for retrieve_project_group_error400".to_string(),
                ),
                retrieve_project_group_error403: Err(
                    "no value supplied for retrieve_project_group_error403".to_string(),
                ),
                retrieve_project_group_error404: Err(
                    "no value supplied for retrieve_project_group_error404".to_string(),
                ),
                retrieve_project_group_error429: Err(
                    "no value supplied for retrieve_project_group_error429".to_string(),
                ),
                retrieve_project_group_params: Err(
                    "no value supplied for retrieve_project_group_params".to_string(),
                ),
                retrieve_project_group_response: Err(
                    "no value supplied for retrieve_project_group_response".to_string(),
                ),
            }
        }
    }
    impl ProjectGroupsApiSchema {
        pub fn add_project_group_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_project_group_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_project_group_error400: {e}")
            });
            self
        }
        pub fn add_project_group_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_project_group_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_project_group_error403: {e}")
            });
            self
        }
        pub fn add_project_group_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_project_group_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_project_group_error404: {e}")
            });
            self
        }
        pub fn add_project_group_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_project_group_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_project_group_error429: {e}")
            });
            self
        }
        pub fn add_project_group_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_project_group_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_project_group_error500: {e}")
            });
            self
        }
        pub fn add_project_group_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddProjectGroupParams>,
            T::Error: ::std::fmt::Display,
        {
            self.add_project_group_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_project_group_params: {e}")
            });
            self
        }
        pub fn add_project_group_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteProjectGroupBody>,
            T::Error: ::std::fmt::Display,
        {
            self.add_project_group_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_project_group_request: {e}")
            });
            self
        }
        pub fn add_project_group_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectGroup>,
            T::Error: ::std::fmt::Display,
        {
            self.add_project_group_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_project_group_response: {e}")
            });
            self
        }
        pub fn list_project_groups_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_groups_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_groups_error400: {e}")
            });
            self
        }
        pub fn list_project_groups_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_groups_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_groups_error403: {e}")
            });
            self
        }
        pub fn list_project_groups_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_groups_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_groups_error404: {e}")
            });
            self
        }
        pub fn list_project_groups_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_groups_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_groups_error429: {e}")
            });
            self
        }
        pub fn list_project_groups_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectGroupsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_groups_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_groups_params: {e}")
            });
            self
        }
        pub fn list_project_groups_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectGroupListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_groups_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_groups_response: {e}")
            });
            self
        }
        pub fn remove_project_group_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_project_group_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_project_group_error400: {e}")
            });
            self
        }
        pub fn remove_project_group_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_project_group_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_project_group_error403: {e}")
            });
            self
        }
        pub fn remove_project_group_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_project_group_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_project_group_error404: {e}")
            });
            self
        }
        pub fn remove_project_group_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_project_group_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_project_group_error429: {e}")
            });
            self
        }
        pub fn remove_project_group_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RemoveProjectGroupParams>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_project_group_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_project_group_params: {e}")
            });
            self
        }
        pub fn remove_project_group_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectGroupDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_project_group_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_project_group_response: {e}")
            });
            self
        }
        pub fn retrieve_project_group_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_group_error400: {e}")
            });
            self
        }
        pub fn retrieve_project_group_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_group_error403: {e}")
            });
            self
        }
        pub fn retrieve_project_group_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_group_error404: {e}")
            });
            self
        }
        pub fn retrieve_project_group_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_group_error429: {e}")
            });
            self
        }
        pub fn retrieve_project_group_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectGroupParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_group_params: {e}")
            });
            self
        }
        pub fn retrieve_project_group_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectGroup>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_group_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectGroupsApiSchema> for super::ProjectGroupsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectGroupsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_project_group_error400: value.add_project_group_error400?,
                add_project_group_error403: value.add_project_group_error403?,
                add_project_group_error404: value.add_project_group_error404?,
                add_project_group_error429: value.add_project_group_error429?,
                add_project_group_error500: value.add_project_group_error500?,
                add_project_group_params: value.add_project_group_params?,
                add_project_group_request: value.add_project_group_request?,
                add_project_group_response: value.add_project_group_response?,
                list_project_groups_error400: value.list_project_groups_error400?,
                list_project_groups_error403: value.list_project_groups_error403?,
                list_project_groups_error404: value.list_project_groups_error404?,
                list_project_groups_error429: value.list_project_groups_error429?,
                list_project_groups_params: value.list_project_groups_params?,
                list_project_groups_response: value.list_project_groups_response?,
                remove_project_group_error400: value.remove_project_group_error400?,
                remove_project_group_error403: value.remove_project_group_error403?,
                remove_project_group_error404: value.remove_project_group_error404?,
                remove_project_group_error429: value.remove_project_group_error429?,
                remove_project_group_params: value.remove_project_group_params?,
                remove_project_group_response: value.remove_project_group_response?,
                retrieve_project_group_error400: value.retrieve_project_group_error400?,
                retrieve_project_group_error403: value.retrieve_project_group_error403?,
                retrieve_project_group_error404: value.retrieve_project_group_error404?,
                retrieve_project_group_error429: value.retrieve_project_group_error429?,
                retrieve_project_group_params: value.retrieve_project_group_params?,
                retrieve_project_group_response: value.retrieve_project_group_response?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectGroupsApiSchema> for ProjectGroupsApiSchema {
        fn from(value: super::ProjectGroupsApiSchema) -> Self {
            Self {
                add_project_group_error400: Ok(value.add_project_group_error400),
                add_project_group_error403: Ok(value.add_project_group_error403),
                add_project_group_error404: Ok(value.add_project_group_error404),
                add_project_group_error429: Ok(value.add_project_group_error429),
                add_project_group_error500: Ok(value.add_project_group_error500),
                add_project_group_params: Ok(value.add_project_group_params),
                add_project_group_request: Ok(value.add_project_group_request),
                add_project_group_response: Ok(value.add_project_group_response),
                list_project_groups_error400: Ok(value.list_project_groups_error400),
                list_project_groups_error403: Ok(value.list_project_groups_error403),
                list_project_groups_error404: Ok(value.list_project_groups_error404),
                list_project_groups_error429: Ok(value.list_project_groups_error429),
                list_project_groups_params: Ok(value.list_project_groups_params),
                list_project_groups_response: Ok(value.list_project_groups_response),
                remove_project_group_error400: Ok(value.remove_project_group_error400),
                remove_project_group_error403: Ok(value.remove_project_group_error403),
                remove_project_group_error404: Ok(value.remove_project_group_error404),
                remove_project_group_error429: Ok(value.remove_project_group_error429),
                remove_project_group_params: Ok(value.remove_project_group_params),
                remove_project_group_response: Ok(value.remove_project_group_response),
                retrieve_project_group_error400: Ok(value.retrieve_project_group_error400),
                retrieve_project_group_error403: Ok(value.retrieve_project_group_error403),
                retrieve_project_group_error404: Ok(value.retrieve_project_group_error404),
                retrieve_project_group_error429: Ok(value.retrieve_project_group_error429),
                retrieve_project_group_params: Ok(value.retrieve_project_group_params),
                retrieve_project_group_response: Ok(value.retrieve_project_group_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RemoveProjectGroupParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RemoveProjectGroupParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl RemoveProjectGroupParams {
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
    impl ::std::convert::TryFrom<RemoveProjectGroupParams> for super::RemoveProjectGroupParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RemoveProjectGroupParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::RemoveProjectGroupParams> for RemoveProjectGroupParams {
        fn from(value: super::RemoveProjectGroupParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectGroupParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        group_type: ::std::result::Result<
            ::std::option::Option<super::RetrieveProjectGroupParamsGroupType>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectGroupParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                group_type: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl RetrieveProjectGroupParams {
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
        pub fn group_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RetrieveProjectGroupParamsGroupType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.group_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_type: {e}"));
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
    impl ::std::convert::TryFrom<RetrieveProjectGroupParams> for super::RetrieveProjectGroupParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectGroupParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                group_type: value.group_type?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectGroupParams> for RetrieveProjectGroupParams {
        fn from(value: super::RetrieveProjectGroupParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
                group_type: Ok(value.group_type),
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
