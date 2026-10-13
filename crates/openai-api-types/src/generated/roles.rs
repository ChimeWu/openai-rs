pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, Role, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`CreateProjectRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateProjectRoleParams {
    pub project_id: ::std::string::String,
}
impl CreateProjectRoleParams {
    pub fn builder() -> builder::CreateProjectRoleParams {
        Default::default()
    }
}
#[doc = "`DeleteProjectRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteProjectRoleParams {
    pub project_id: ::std::string::String,
    pub role_id: ::std::string::String,
}
impl DeleteProjectRoleParams {
    pub fn builder() -> builder::DeleteProjectRoleParams {
        Default::default()
    }
}
#[doc = "`DeleteRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteRoleParams {
    pub role_id: ::std::string::String,
}
impl DeleteRoleParams {
    pub fn builder() -> builder::DeleteRoleParams {
        Default::default()
    }
}
#[doc = "`ListProjectRolesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectRolesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListProjectRolesParamsOrder>,
    pub project_id: ::std::string::String,
}
impl ListProjectRolesParams {
    pub fn builder() -> builder::ListProjectRolesParams {
        Default::default()
    }
}
#[doc = "`ListProjectRolesParamsOrder`"]
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
pub enum ListProjectRolesParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListProjectRolesParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListProjectRolesParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListProjectRolesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListProjectRolesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListRolesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListRolesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListRolesParamsOrder>,
}
impl ListRolesParams {
    pub fn builder() -> builder::ListRolesParams {
        Default::default()
    }
}
#[doc = "`ListRolesParamsOrder`"]
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
pub enum ListRolesParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListRolesParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListRolesParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListRolesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListRolesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Request payload for creating a custom role."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PublicCreateOrganizationRoleBody {
    #[doc = "Optional description of the role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Permissions to grant to the role."]
    pub permissions: ::std::vec::Vec<::std::string::String>,
    #[doc = "Unique name for the role."]
    pub role_name: ::std::string::String,
}
impl PublicCreateOrganizationRoleBody {
    pub fn builder() -> builder::PublicCreateOrganizationRoleBody {
        Default::default()
    }
}
#[doc = "Paginated list of roles available on an organization or project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PublicRoleListResource {
    #[doc = "Roles returned in the current page."]
    pub data: ::std::vec::Vec<::openai_api_types::common::Role>,
    #[doc = "Whether more roles are available when paginating."]
    pub has_more: bool,
    #[doc = "Cursor to fetch the next page of results, or `null` when there are no additional roles."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub next: ::std::option::Option<::std::string::String>,
    #[doc = "Always `list`."]
    pub object: PublicRoleListResourceObject,
}
impl PublicRoleListResource {
    pub fn builder() -> builder::PublicRoleListResource {
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
pub enum PublicRoleListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for PublicRoleListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for PublicRoleListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PublicRoleListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicRoleListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Request payload for updating an existing role."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct PublicUpdateOrganizationRoleBody {
    #[doc = "New description for the role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Updated set of permissions for the role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "New name for the role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role_name: ::std::option::Option<::std::string::String>,
}
impl PublicUpdateOrganizationRoleBody {
    pub fn builder() -> builder::PublicUpdateOrganizationRoleBody {
        Default::default()
    }
}
#[doc = "`RetrieveProjectRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectRoleParams {
    pub project_id: ::std::string::String,
    pub role_id: ::std::string::String,
}
impl RetrieveProjectRoleParams {
    pub fn builder() -> builder::RetrieveProjectRoleParams {
        Default::default()
    }
}
#[doc = "`RetrieveRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveRoleParams {
    pub role_id: ::std::string::String,
}
impl RetrieveRoleParams {
    pub fn builder() -> builder::RetrieveRoleParams {
        Default::default()
    }
}
#[doc = "Confirmation payload returned after deleting a role."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RoleDeletedResource {
    #[doc = "Whether the role was deleted."]
    pub deleted: bool,
    #[doc = "Identifier of the deleted role."]
    pub id: ::std::string::String,
    #[doc = "Always `role.deleted`."]
    pub object: RoleDeletedResourceObject,
}
impl RoleDeletedResource {
    pub fn builder() -> builder::RoleDeletedResource {
        Default::default()
    }
}
#[doc = "Always `role.deleted`."]
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
pub enum RoleDeletedResourceObject {
    #[serde(rename = "role.deleted")]
    RoleDeleted,
}
impl ::std::fmt::Display for RoleDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RoleDeleted => f.write_str("role.deleted"),
        }
    }
}
impl ::std::str::FromStr for RoleDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "role.deleted" => Ok(Self::RoleDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RoleDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RoleDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RolesApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RolesApiSchema {
    #[serde(rename = "CreateProjectRoleError400")]
    pub create_project_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectRoleError403")]
    pub create_project_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "CreateProjectRoleError404")]
    pub create_project_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectRoleError429")]
    pub create_project_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectRoleParams")]
    pub create_project_role_params: CreateProjectRoleParams,
    #[serde(rename = "CreateProjectRoleRequest")]
    pub create_project_role_request: PublicCreateOrganizationRoleBody,
    #[serde(rename = "CreateProjectRoleResponse")]
    pub create_project_role_response: ::openai_api_types::common::Role,
    #[serde(rename = "CreateRoleError400")]
    pub create_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateRoleError403")]
    pub create_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "CreateRoleError404")]
    pub create_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateRoleError429")]
    pub create_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateRoleRequest")]
    pub create_role_request: PublicCreateOrganizationRoleBody,
    #[serde(rename = "CreateRoleResponse")]
    pub create_role_response: ::openai_api_types::common::Role,
    #[serde(rename = "DeleteProjectRoleError400")]
    pub delete_project_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectRoleError403")]
    pub delete_project_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteProjectRoleError404")]
    pub delete_project_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectRoleError429")]
    pub delete_project_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectRoleParams")]
    pub delete_project_role_params: DeleteProjectRoleParams,
    #[serde(rename = "DeleteProjectRoleResponse")]
    pub delete_project_role_response: RoleDeletedResource,
    #[serde(rename = "DeleteRoleError400")]
    pub delete_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteRoleError403")]
    pub delete_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteRoleError404")]
    pub delete_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteRoleError429")]
    pub delete_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteRoleParams")]
    pub delete_role_params: DeleteRoleParams,
    #[serde(rename = "DeleteRoleResponse")]
    pub delete_role_response: RoleDeletedResource,
    #[serde(rename = "ListProjectRolesError400")]
    pub list_project_roles_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectRolesError403")]
    pub list_project_roles_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectRolesError404")]
    pub list_project_roles_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectRolesError429")]
    pub list_project_roles_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectRolesParams")]
    pub list_project_roles_params: ListProjectRolesParams,
    #[serde(rename = "ListProjectRolesResponse")]
    pub list_project_roles_response: PublicRoleListResource,
    #[serde(rename = "ListRolesError400")]
    pub list_roles_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListRolesError403")]
    pub list_roles_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListRolesError404")]
    pub list_roles_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListRolesError429")]
    pub list_roles_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListRolesParams")]
    pub list_roles_params: ListRolesParams,
    #[serde(rename = "ListRolesResponse")]
    pub list_roles_response: PublicRoleListResource,
    #[serde(rename = "RetrieveProjectRoleError403")]
    pub retrieve_project_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectRoleError404")]
    pub retrieve_project_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectRoleError429")]
    pub retrieve_project_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectRoleParams")]
    pub retrieve_project_role_params: RetrieveProjectRoleParams,
    #[serde(rename = "RetrieveProjectRoleResponse")]
    pub retrieve_project_role_response: ::openai_api_types::common::Role,
    #[serde(rename = "RetrieveRoleError403")]
    pub retrieve_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveRoleError404")]
    pub retrieve_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveRoleError429")]
    pub retrieve_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveRoleParams")]
    pub retrieve_role_params: RetrieveRoleParams,
    #[serde(rename = "RetrieveRoleResponse")]
    pub retrieve_role_response: ::openai_api_types::common::Role,
    #[serde(rename = "UpdateProjectRoleError400")]
    pub update_project_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectRoleError403")]
    pub update_project_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateProjectRoleError404")]
    pub update_project_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectRoleError429")]
    pub update_project_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectRoleParams")]
    pub update_project_role_params: UpdateProjectRoleParams,
    #[serde(rename = "UpdateProjectRoleRequest")]
    pub update_project_role_request: PublicUpdateOrganizationRoleBody,
    #[serde(rename = "UpdateProjectRoleResponse")]
    pub update_project_role_response: ::openai_api_types::common::Role,
    #[serde(rename = "UpdateRoleError400")]
    pub update_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateRoleError403")]
    pub update_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateRoleError404")]
    pub update_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateRoleError429")]
    pub update_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateRoleParams")]
    pub update_role_params: UpdateRoleParams,
    #[serde(rename = "UpdateRoleRequest")]
    pub update_role_request: PublicUpdateOrganizationRoleBody,
    #[serde(rename = "UpdateRoleResponse")]
    pub update_role_response: ::openai_api_types::common::Role,
}
impl RolesApiSchema {
    pub fn builder() -> builder::RolesApiSchema {
        Default::default()
    }
}
#[doc = "`UpdateProjectRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectRoleParams {
    pub project_id: ::std::string::String,
    pub role_id: ::std::string::String,
}
impl UpdateProjectRoleParams {
    pub fn builder() -> builder::UpdateProjectRoleParams {
        Default::default()
    }
}
#[doc = "`UpdateRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateRoleParams {
    pub role_id: ::std::string::String,
}
impl UpdateRoleParams {
    pub fn builder() -> builder::UpdateRoleParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateProjectRoleParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateProjectRoleParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl CreateProjectRoleParams {
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
    impl ::std::convert::TryFrom<CreateProjectRoleParams> for super::CreateProjectRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateProjectRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateProjectRoleParams> for CreateProjectRoleParams {
        fn from(value: super::CreateProjectRoleParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteProjectRoleParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteProjectRoleParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl DeleteProjectRoleParams {
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
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteProjectRoleParams> for super::DeleteProjectRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteProjectRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteProjectRoleParams> for DeleteProjectRoleParams {
        fn from(value: super::DeleteProjectRoleParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                role_id: Ok(value.role_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteRoleParams {
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteRoleParams {
        fn default() -> Self {
            Self {
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl DeleteRoleParams {
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteRoleParams> for super::DeleteRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteRoleParams> for DeleteRoleParams {
        fn from(value: super::DeleteRoleParams) -> Self {
            Self {
                role_id: Ok(value.role_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectRolesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListProjectRolesParamsOrder>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectRolesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ListProjectRolesParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListProjectRolesParamsOrder>>,
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
    impl ::std::convert::TryFrom<ListProjectRolesParams> for super::ListProjectRolesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectRolesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectRolesParams> for ListProjectRolesParams {
        fn from(value: super::ListProjectRolesParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRolesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListRolesParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListRolesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListRolesParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListRolesParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRolesParams> for super::ListRolesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRolesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListRolesParams> for ListRolesParams {
        fn from(value: super::ListRolesParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicCreateOrganizationRoleBody {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        permissions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        role_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PublicCreateOrganizationRoleBody {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                permissions: Err("no value supplied for permissions".to_string()),
                role_name: Err("no value supplied for role_name".to_string()),
            }
        }
    }
    impl PublicCreateOrganizationRoleBody {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn permissions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.permissions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for permissions: {e}"));
            self
        }
        pub fn role_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicCreateOrganizationRoleBody>
        for super::PublicCreateOrganizationRoleBody
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicCreateOrganizationRoleBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                permissions: value.permissions?,
                role_name: value.role_name?,
            })
        }
    }
    impl ::std::convert::From<super::PublicCreateOrganizationRoleBody>
        for PublicCreateOrganizationRoleBody
    {
        fn from(value: super::PublicCreateOrganizationRoleBody) -> Self {
            Self {
                description: Ok(value.description),
                permissions: Ok(value.permissions),
                role_name: Ok(value.role_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicRoleListResource {
        data: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::Role>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        next: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::PublicRoleListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for PublicRoleListResource {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                next: Err("no value supplied for next".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl PublicRoleListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::openai_api_types::common::Role>>,
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
            T: ::std::convert::TryInto<super::PublicRoleListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicRoleListResource> for super::PublicRoleListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicRoleListResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                next: value.next?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::PublicRoleListResource> for PublicRoleListResource {
        fn from(value: super::PublicRoleListResource) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                next: Ok(value.next),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicUpdateOrganizationRoleBody {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        permissions: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        role_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PublicUpdateOrganizationRoleBody {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                permissions: Ok(Default::default()),
                role_name: Ok(Default::default()),
            }
        }
    }
    impl PublicUpdateOrganizationRoleBody {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn permissions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.permissions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for permissions: {e}"));
            self
        }
        pub fn role_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicUpdateOrganizationRoleBody>
        for super::PublicUpdateOrganizationRoleBody
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicUpdateOrganizationRoleBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                permissions: value.permissions?,
                role_name: value.role_name?,
            })
        }
    }
    impl ::std::convert::From<super::PublicUpdateOrganizationRoleBody>
        for PublicUpdateOrganizationRoleBody
    {
        fn from(value: super::PublicUpdateOrganizationRoleBody) -> Self {
            Self {
                description: Ok(value.description),
                permissions: Ok(value.permissions),
                role_name: Ok(value.role_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectRoleParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectRoleParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl RetrieveProjectRoleParams {
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
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveProjectRoleParams> for super::RetrieveProjectRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectRoleParams> for RetrieveProjectRoleParams {
        fn from(value: super::RetrieveProjectRoleParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                role_id: Ok(value.role_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveRoleParams {
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveRoleParams {
        fn default() -> Self {
            Self {
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl RetrieveRoleParams {
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveRoleParams> for super::RetrieveRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveRoleParams> for RetrieveRoleParams {
        fn from(value: super::RetrieveRoleParams) -> Self {
            Self {
                role_id: Ok(value.role_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RoleDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::RoleDeletedResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for RoleDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl RoleDeletedResource {
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
            T: ::std::convert::TryInto<super::RoleDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RoleDeletedResource> for super::RoleDeletedResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RoleDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::RoleDeletedResource> for RoleDeletedResource {
        fn from(value: super::RoleDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RolesApiSchema {
        create_project_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        create_project_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_role_params:
            ::std::result::Result<super::CreateProjectRoleParams, ::std::string::String>,
        create_project_role_request:
            ::std::result::Result<super::PublicCreateOrganizationRoleBody, ::std::string::String>,
        create_project_role_response:
            ::std::result::Result<::openai_api_types::common::Role, ::std::string::String>,
        create_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        create_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_role_request:
            ::std::result::Result<super::PublicCreateOrganizationRoleBody, ::std::string::String>,
        create_role_response:
            ::std::result::Result<::openai_api_types::common::Role, ::std::string::String>,
        delete_project_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_project_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_role_params:
            ::std::result::Result<super::DeleteProjectRoleParams, ::std::string::String>,
        delete_project_role_response:
            ::std::result::Result<super::RoleDeletedResource, ::std::string::String>,
        delete_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_role_params: ::std::result::Result<super::DeleteRoleParams, ::std::string::String>,
        delete_role_response:
            ::std::result::Result<super::RoleDeletedResource, ::std::string::String>,
        list_project_roles_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_roles_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_roles_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_roles_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_roles_params:
            ::std::result::Result<super::ListProjectRolesParams, ::std::string::String>,
        list_project_roles_response:
            ::std::result::Result<super::PublicRoleListResource, ::std::string::String>,
        list_roles_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_roles_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_roles_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_roles_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_roles_params: ::std::result::Result<super::ListRolesParams, ::std::string::String>,
        list_roles_response:
            ::std::result::Result<super::PublicRoleListResource, ::std::string::String>,
        retrieve_project_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_role_params:
            ::std::result::Result<super::RetrieveProjectRoleParams, ::std::string::String>,
        retrieve_project_role_response:
            ::std::result::Result<::openai_api_types::common::Role, ::std::string::String>,
        retrieve_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_role_params:
            ::std::result::Result<super::RetrieveRoleParams, ::std::string::String>,
        retrieve_role_response:
            ::std::result::Result<::openai_api_types::common::Role, ::std::string::String>,
        update_project_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_project_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_role_params:
            ::std::result::Result<super::UpdateProjectRoleParams, ::std::string::String>,
        update_project_role_request:
            ::std::result::Result<super::PublicUpdateOrganizationRoleBody, ::std::string::String>,
        update_project_role_response:
            ::std::result::Result<::openai_api_types::common::Role, ::std::string::String>,
        update_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_role_params: ::std::result::Result<super::UpdateRoleParams, ::std::string::String>,
        update_role_request:
            ::std::result::Result<super::PublicUpdateOrganizationRoleBody, ::std::string::String>,
        update_role_response:
            ::std::result::Result<::openai_api_types::common::Role, ::std::string::String>,
    }
    impl ::std::default::Default for RolesApiSchema {
        fn default() -> Self {
            Self {
                create_project_role_error400: Err(
                    "no value supplied for create_project_role_error400".to_string(),
                ),
                create_project_role_error403: Err(
                    "no value supplied for create_project_role_error403".to_string(),
                ),
                create_project_role_error404: Err(
                    "no value supplied for create_project_role_error404".to_string(),
                ),
                create_project_role_error429: Err(
                    "no value supplied for create_project_role_error429".to_string(),
                ),
                create_project_role_params: Err(
                    "no value supplied for create_project_role_params".to_string()
                ),
                create_project_role_request: Err(
                    "no value supplied for create_project_role_request".to_string(),
                ),
                create_project_role_response: Err(
                    "no value supplied for create_project_role_response".to_string(),
                ),
                create_role_error400: Err("no value supplied for create_role_error400".to_string()),
                create_role_error403: Err("no value supplied for create_role_error403".to_string()),
                create_role_error404: Err("no value supplied for create_role_error404".to_string()),
                create_role_error429: Err("no value supplied for create_role_error429".to_string()),
                create_role_request: Err("no value supplied for create_role_request".to_string()),
                create_role_response: Err("no value supplied for create_role_response".to_string()),
                delete_project_role_error400: Err(
                    "no value supplied for delete_project_role_error400".to_string(),
                ),
                delete_project_role_error403: Err(
                    "no value supplied for delete_project_role_error403".to_string(),
                ),
                delete_project_role_error404: Err(
                    "no value supplied for delete_project_role_error404".to_string(),
                ),
                delete_project_role_error429: Err(
                    "no value supplied for delete_project_role_error429".to_string(),
                ),
                delete_project_role_params: Err(
                    "no value supplied for delete_project_role_params".to_string()
                ),
                delete_project_role_response: Err(
                    "no value supplied for delete_project_role_response".to_string(),
                ),
                delete_role_error400: Err("no value supplied for delete_role_error400".to_string()),
                delete_role_error403: Err("no value supplied for delete_role_error403".to_string()),
                delete_role_error404: Err("no value supplied for delete_role_error404".to_string()),
                delete_role_error429: Err("no value supplied for delete_role_error429".to_string()),
                delete_role_params: Err("no value supplied for delete_role_params".to_string()),
                delete_role_response: Err("no value supplied for delete_role_response".to_string()),
                list_project_roles_error400: Err(
                    "no value supplied for list_project_roles_error400".to_string(),
                ),
                list_project_roles_error403: Err(
                    "no value supplied for list_project_roles_error403".to_string(),
                ),
                list_project_roles_error404: Err(
                    "no value supplied for list_project_roles_error404".to_string(),
                ),
                list_project_roles_error429: Err(
                    "no value supplied for list_project_roles_error429".to_string(),
                ),
                list_project_roles_params: Err(
                    "no value supplied for list_project_roles_params".to_string()
                ),
                list_project_roles_response: Err(
                    "no value supplied for list_project_roles_response".to_string(),
                ),
                list_roles_error400: Err("no value supplied for list_roles_error400".to_string()),
                list_roles_error403: Err("no value supplied for list_roles_error403".to_string()),
                list_roles_error404: Err("no value supplied for list_roles_error404".to_string()),
                list_roles_error429: Err("no value supplied for list_roles_error429".to_string()),
                list_roles_params: Err("no value supplied for list_roles_params".to_string()),
                list_roles_response: Err("no value supplied for list_roles_response".to_string()),
                retrieve_project_role_error403: Err(
                    "no value supplied for retrieve_project_role_error403".to_string(),
                ),
                retrieve_project_role_error404: Err(
                    "no value supplied for retrieve_project_role_error404".to_string(),
                ),
                retrieve_project_role_error429: Err(
                    "no value supplied for retrieve_project_role_error429".to_string(),
                ),
                retrieve_project_role_params: Err(
                    "no value supplied for retrieve_project_role_params".to_string(),
                ),
                retrieve_project_role_response: Err(
                    "no value supplied for retrieve_project_role_response".to_string(),
                ),
                retrieve_role_error403: Err(
                    "no value supplied for retrieve_role_error403".to_string()
                ),
                retrieve_role_error404: Err(
                    "no value supplied for retrieve_role_error404".to_string()
                ),
                retrieve_role_error429: Err(
                    "no value supplied for retrieve_role_error429".to_string()
                ),
                retrieve_role_params: Err("no value supplied for retrieve_role_params".to_string()),
                retrieve_role_response: Err(
                    "no value supplied for retrieve_role_response".to_string()
                ),
                update_project_role_error400: Err(
                    "no value supplied for update_project_role_error400".to_string(),
                ),
                update_project_role_error403: Err(
                    "no value supplied for update_project_role_error403".to_string(),
                ),
                update_project_role_error404: Err(
                    "no value supplied for update_project_role_error404".to_string(),
                ),
                update_project_role_error429: Err(
                    "no value supplied for update_project_role_error429".to_string(),
                ),
                update_project_role_params: Err(
                    "no value supplied for update_project_role_params".to_string()
                ),
                update_project_role_request: Err(
                    "no value supplied for update_project_role_request".to_string(),
                ),
                update_project_role_response: Err(
                    "no value supplied for update_project_role_response".to_string(),
                ),
                update_role_error400: Err("no value supplied for update_role_error400".to_string()),
                update_role_error403: Err("no value supplied for update_role_error403".to_string()),
                update_role_error404: Err("no value supplied for update_role_error404".to_string()),
                update_role_error429: Err("no value supplied for update_role_error429".to_string()),
                update_role_params: Err("no value supplied for update_role_params".to_string()),
                update_role_request: Err("no value supplied for update_role_request".to_string()),
                update_role_response: Err("no value supplied for update_role_response".to_string()),
            }
        }
    }
    impl RolesApiSchema {
        pub fn create_project_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_role_error400: {e}")
            });
            self
        }
        pub fn create_project_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_role_error403: {e}")
            });
            self
        }
        pub fn create_project_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_role_error404: {e}")
            });
            self
        }
        pub fn create_project_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_role_error429: {e}")
            });
            self
        }
        pub fn create_project_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateProjectRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_role_params: {e}")
            });
            self
        }
        pub fn create_project_role_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicCreateOrganizationRoleBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_role_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_role_request: {e}")
            });
            self
        }
        pub fn create_project_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_role_response: {e}")
            });
            self
        }
        pub fn create_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_role_error400: {e}")
            });
            self
        }
        pub fn create_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_role_error403: {e}")
            });
            self
        }
        pub fn create_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_role_error404: {e}")
            });
            self
        }
        pub fn create_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_role_error429: {e}")
            });
            self
        }
        pub fn create_role_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicCreateOrganizationRoleBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_role_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_role_request: {e}")
            });
            self
        }
        pub fn create_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.create_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_role_response: {e}")
            });
            self
        }
        pub fn delete_project_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_role_error400: {e}")
            });
            self
        }
        pub fn delete_project_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_role_error403: {e}")
            });
            self
        }
        pub fn delete_project_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_role_error404: {e}")
            });
            self
        }
        pub fn delete_project_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_role_error429: {e}")
            });
            self
        }
        pub fn delete_project_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteProjectRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_role_params: {e}")
            });
            self
        }
        pub fn delete_project_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RoleDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_role_response: {e}")
            });
            self
        }
        pub fn delete_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_role_error400: {e}")
            });
            self
        }
        pub fn delete_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_role_error403: {e}")
            });
            self
        }
        pub fn delete_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_role_error404: {e}")
            });
            self
        }
        pub fn delete_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_role_error429: {e}")
            });
            self
        }
        pub fn delete_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_role_params: {e}")
            });
            self
        }
        pub fn delete_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RoleDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_role_response: {e}")
            });
            self
        }
        pub fn list_project_roles_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_roles_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_roles_error400: {e}")
            });
            self
        }
        pub fn list_project_roles_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_roles_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_roles_error403: {e}")
            });
            self
        }
        pub fn list_project_roles_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_roles_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_roles_error404: {e}")
            });
            self
        }
        pub fn list_project_roles_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_roles_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_roles_error429: {e}")
            });
            self
        }
        pub fn list_project_roles_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectRolesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_roles_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_roles_params: {e}")
            });
            self
        }
        pub fn list_project_roles_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicRoleListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_roles_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_roles_response: {e}")
            });
            self
        }
        pub fn list_roles_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_roles_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_roles_error400: {e}")
            });
            self
        }
        pub fn list_roles_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_roles_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_roles_error403: {e}")
            });
            self
        }
        pub fn list_roles_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_roles_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_roles_error404: {e}")
            });
            self
        }
        pub fn list_roles_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_roles_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_roles_error429: {e}")
            });
            self
        }
        pub fn list_roles_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListRolesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_roles_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_roles_params: {e}"));
            self
        }
        pub fn list_roles_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicRoleListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_roles_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_roles_response: {e}")
            });
            self
        }
        pub fn retrieve_project_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_role_error403: {e}")
            });
            self
        }
        pub fn retrieve_project_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_role_error404: {e}")
            });
            self
        }
        pub fn retrieve_project_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_role_error429: {e}")
            });
            self
        }
        pub fn retrieve_project_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_role_params: {e}")
            });
            self
        }
        pub fn retrieve_project_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_role_response: {e}")
            });
            self
        }
        pub fn retrieve_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_role_error403: {e}")
            });
            self
        }
        pub fn retrieve_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_role_error404: {e}")
            });
            self
        }
        pub fn retrieve_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_role_error429: {e}")
            });
            self
        }
        pub fn retrieve_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_role_params: {e}")
            });
            self
        }
        pub fn retrieve_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_role_response: {e}")
            });
            self
        }
        pub fn update_project_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_role_error400: {e}")
            });
            self
        }
        pub fn update_project_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_role_error403: {e}")
            });
            self
        }
        pub fn update_project_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_role_error404: {e}")
            });
            self
        }
        pub fn update_project_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_role_error429: {e}")
            });
            self
        }
        pub fn update_project_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_role_params: {e}")
            });
            self
        }
        pub fn update_project_role_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicUpdateOrganizationRoleBody>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_role_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_role_request: {e}")
            });
            self
        }
        pub fn update_project_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_role_response: {e}")
            });
            self
        }
        pub fn update_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_role_error400: {e}")
            });
            self
        }
        pub fn update_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_role_error403: {e}")
            });
            self
        }
        pub fn update_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_role_error404: {e}")
            });
            self
        }
        pub fn update_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_role_error429: {e}")
            });
            self
        }
        pub fn update_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_role_params: {e}")
            });
            self
        }
        pub fn update_role_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicUpdateOrganizationRoleBody>,
            T::Error: ::std::fmt::Display,
        {
            self.update_role_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_role_request: {e}")
            });
            self
        }
        pub fn update_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.update_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_role_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<RolesApiSchema> for super::RolesApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RolesApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_project_role_error400: value.create_project_role_error400?,
                create_project_role_error403: value.create_project_role_error403?,
                create_project_role_error404: value.create_project_role_error404?,
                create_project_role_error429: value.create_project_role_error429?,
                create_project_role_params: value.create_project_role_params?,
                create_project_role_request: value.create_project_role_request?,
                create_project_role_response: value.create_project_role_response?,
                create_role_error400: value.create_role_error400?,
                create_role_error403: value.create_role_error403?,
                create_role_error404: value.create_role_error404?,
                create_role_error429: value.create_role_error429?,
                create_role_request: value.create_role_request?,
                create_role_response: value.create_role_response?,
                delete_project_role_error400: value.delete_project_role_error400?,
                delete_project_role_error403: value.delete_project_role_error403?,
                delete_project_role_error404: value.delete_project_role_error404?,
                delete_project_role_error429: value.delete_project_role_error429?,
                delete_project_role_params: value.delete_project_role_params?,
                delete_project_role_response: value.delete_project_role_response?,
                delete_role_error400: value.delete_role_error400?,
                delete_role_error403: value.delete_role_error403?,
                delete_role_error404: value.delete_role_error404?,
                delete_role_error429: value.delete_role_error429?,
                delete_role_params: value.delete_role_params?,
                delete_role_response: value.delete_role_response?,
                list_project_roles_error400: value.list_project_roles_error400?,
                list_project_roles_error403: value.list_project_roles_error403?,
                list_project_roles_error404: value.list_project_roles_error404?,
                list_project_roles_error429: value.list_project_roles_error429?,
                list_project_roles_params: value.list_project_roles_params?,
                list_project_roles_response: value.list_project_roles_response?,
                list_roles_error400: value.list_roles_error400?,
                list_roles_error403: value.list_roles_error403?,
                list_roles_error404: value.list_roles_error404?,
                list_roles_error429: value.list_roles_error429?,
                list_roles_params: value.list_roles_params?,
                list_roles_response: value.list_roles_response?,
                retrieve_project_role_error403: value.retrieve_project_role_error403?,
                retrieve_project_role_error404: value.retrieve_project_role_error404?,
                retrieve_project_role_error429: value.retrieve_project_role_error429?,
                retrieve_project_role_params: value.retrieve_project_role_params?,
                retrieve_project_role_response: value.retrieve_project_role_response?,
                retrieve_role_error403: value.retrieve_role_error403?,
                retrieve_role_error404: value.retrieve_role_error404?,
                retrieve_role_error429: value.retrieve_role_error429?,
                retrieve_role_params: value.retrieve_role_params?,
                retrieve_role_response: value.retrieve_role_response?,
                update_project_role_error400: value.update_project_role_error400?,
                update_project_role_error403: value.update_project_role_error403?,
                update_project_role_error404: value.update_project_role_error404?,
                update_project_role_error429: value.update_project_role_error429?,
                update_project_role_params: value.update_project_role_params?,
                update_project_role_request: value.update_project_role_request?,
                update_project_role_response: value.update_project_role_response?,
                update_role_error400: value.update_role_error400?,
                update_role_error403: value.update_role_error403?,
                update_role_error404: value.update_role_error404?,
                update_role_error429: value.update_role_error429?,
                update_role_params: value.update_role_params?,
                update_role_request: value.update_role_request?,
                update_role_response: value.update_role_response?,
            })
        }
    }
    impl ::std::convert::From<super::RolesApiSchema> for RolesApiSchema {
        fn from(value: super::RolesApiSchema) -> Self {
            Self {
                create_project_role_error400: Ok(value.create_project_role_error400),
                create_project_role_error403: Ok(value.create_project_role_error403),
                create_project_role_error404: Ok(value.create_project_role_error404),
                create_project_role_error429: Ok(value.create_project_role_error429),
                create_project_role_params: Ok(value.create_project_role_params),
                create_project_role_request: Ok(value.create_project_role_request),
                create_project_role_response: Ok(value.create_project_role_response),
                create_role_error400: Ok(value.create_role_error400),
                create_role_error403: Ok(value.create_role_error403),
                create_role_error404: Ok(value.create_role_error404),
                create_role_error429: Ok(value.create_role_error429),
                create_role_request: Ok(value.create_role_request),
                create_role_response: Ok(value.create_role_response),
                delete_project_role_error400: Ok(value.delete_project_role_error400),
                delete_project_role_error403: Ok(value.delete_project_role_error403),
                delete_project_role_error404: Ok(value.delete_project_role_error404),
                delete_project_role_error429: Ok(value.delete_project_role_error429),
                delete_project_role_params: Ok(value.delete_project_role_params),
                delete_project_role_response: Ok(value.delete_project_role_response),
                delete_role_error400: Ok(value.delete_role_error400),
                delete_role_error403: Ok(value.delete_role_error403),
                delete_role_error404: Ok(value.delete_role_error404),
                delete_role_error429: Ok(value.delete_role_error429),
                delete_role_params: Ok(value.delete_role_params),
                delete_role_response: Ok(value.delete_role_response),
                list_project_roles_error400: Ok(value.list_project_roles_error400),
                list_project_roles_error403: Ok(value.list_project_roles_error403),
                list_project_roles_error404: Ok(value.list_project_roles_error404),
                list_project_roles_error429: Ok(value.list_project_roles_error429),
                list_project_roles_params: Ok(value.list_project_roles_params),
                list_project_roles_response: Ok(value.list_project_roles_response),
                list_roles_error400: Ok(value.list_roles_error400),
                list_roles_error403: Ok(value.list_roles_error403),
                list_roles_error404: Ok(value.list_roles_error404),
                list_roles_error429: Ok(value.list_roles_error429),
                list_roles_params: Ok(value.list_roles_params),
                list_roles_response: Ok(value.list_roles_response),
                retrieve_project_role_error403: Ok(value.retrieve_project_role_error403),
                retrieve_project_role_error404: Ok(value.retrieve_project_role_error404),
                retrieve_project_role_error429: Ok(value.retrieve_project_role_error429),
                retrieve_project_role_params: Ok(value.retrieve_project_role_params),
                retrieve_project_role_response: Ok(value.retrieve_project_role_response),
                retrieve_role_error403: Ok(value.retrieve_role_error403),
                retrieve_role_error404: Ok(value.retrieve_role_error404),
                retrieve_role_error429: Ok(value.retrieve_role_error429),
                retrieve_role_params: Ok(value.retrieve_role_params),
                retrieve_role_response: Ok(value.retrieve_role_response),
                update_project_role_error400: Ok(value.update_project_role_error400),
                update_project_role_error403: Ok(value.update_project_role_error403),
                update_project_role_error404: Ok(value.update_project_role_error404),
                update_project_role_error429: Ok(value.update_project_role_error429),
                update_project_role_params: Ok(value.update_project_role_params),
                update_project_role_request: Ok(value.update_project_role_request),
                update_project_role_response: Ok(value.update_project_role_response),
                update_role_error400: Ok(value.update_role_error400),
                update_role_error403: Ok(value.update_role_error403),
                update_role_error404: Ok(value.update_role_error404),
                update_role_error429: Ok(value.update_role_error429),
                update_role_params: Ok(value.update_role_params),
                update_role_request: Ok(value.update_role_request),
                update_role_response: Ok(value.update_role_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectRoleParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectRoleParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl UpdateProjectRoleParams {
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
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateProjectRoleParams> for super::UpdateProjectRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectRoleParams> for UpdateProjectRoleParams {
        fn from(value: super::UpdateProjectRoleParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                role_id: Ok(value.role_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateRoleParams {
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateRoleParams {
        fn default() -> Self {
            Self {
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl UpdateRoleParams {
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateRoleParams> for super::UpdateRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateRoleParams> for UpdateRoleParams {
        fn from(value: super::UpdateRoleParams) -> Self {
            Self {
                role_id: Ok(value.role_id),
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
