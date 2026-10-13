pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`DeleteModelParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteModelParams {
    pub model: ::std::string::String,
}
impl DeleteModelParams {
    pub fn builder() -> builder::DeleteModelParams {
        Default::default()
    }
}
#[doc = "`DeleteModelResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteModelResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: ::std::string::String,
}
impl DeleteModelResponse {
    pub fn builder() -> builder::DeleteModelResponse {
        Default::default()
    }
}
#[doc = "`ListModelsResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListModelsResponse {
    pub data: ::std::vec::Vec<Model>,
    pub object: ListModelsResponseObject,
}
impl ListModelsResponse {
    pub fn builder() -> builder::ListModelsResponse {
        Default::default()
    }
}
#[doc = "`ListModelsResponseObject`"]
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
pub enum ListModelsResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ListModelsResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ListModelsResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListModelsResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListModelsResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Describes an OpenAI model offering that can be used with the API."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Model {
    #[doc = "The Unix timestamp (in seconds) when the model was created."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created: ::std::option::Option<i64>,
    #[doc = "The model identifier, which can be referenced in the API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always \"model\"."]
    pub object: ModelObject,
    #[doc = "The organization that owns the model."]
    pub owned_by: ::std::string::String,
    #[doc = "The date when the model will shut down, or null if not announced."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shutdown_date: ::std::option::Option<::std::string::String>,
}
impl Model {
    pub fn builder() -> builder::Model {
        Default::default()
    }
}
#[doc = "The object type, which is always \"model\"."]
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
pub enum ModelObject {
    #[serde(rename = "model")]
    Model,
}
impl ::std::fmt::Display for ModelObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Model => f.write_str("model"),
        }
    }
}
impl ::std::str::FromStr for ModelObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "model" => Ok(Self::Model),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModelObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModelObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ModelsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModelsApiSchema {
    #[serde(rename = "DeleteModelError403")]
    pub delete_model_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteModelError404")]
    pub delete_model_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteModelParams")]
    pub delete_model_params: DeleteModelParams,
    #[serde(rename = "DeleteModelResponse")]
    pub delete_model_response: DeleteModelResponse,
    #[serde(rename = "ListModelsError403")]
    pub list_models_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListModelsResponse")]
    pub list_models_response: ListModelsResponse,
    #[serde(rename = "RetrieveModelError403")]
    pub retrieve_model_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveModelError404")]
    pub retrieve_model_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveModelParams")]
    pub retrieve_model_params: RetrieveModelParams,
    #[serde(rename = "RetrieveModelResponse")]
    pub retrieve_model_response: Model,
}
impl ModelsApiSchema {
    pub fn builder() -> builder::ModelsApiSchema {
        Default::default()
    }
}
#[doc = "`RetrieveModelParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveModelParams {
    pub model: ::std::string::String,
}
impl RetrieveModelParams {
    pub fn builder() -> builder::RetrieveModelParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DeleteModelParams {
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteModelParams {
        fn default() -> Self {
            Self {
                model: Err("no value supplied for model".to_string()),
            }
        }
    }
    impl DeleteModelParams {
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
    }
    impl ::std::convert::TryFrom<DeleteModelParams> for super::DeleteModelParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteModelParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                model: value.model?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteModelParams> for DeleteModelParams {
        fn from(value: super::DeleteModelParams) -> Self {
            Self {
                model: Ok(value.model),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteModelResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteModelResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteModelResponse {
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteModelResponse> for super::DeleteModelResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteModelResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteModelResponse> for DeleteModelResponse {
        fn from(value: super::DeleteModelResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListModelsResponse {
        data: ::std::result::Result<::std::vec::Vec<super::Model>, ::std::string::String>,
        object: ::std::result::Result<super::ListModelsResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for ListModelsResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ListModelsResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Model>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListModelsResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListModelsResponse> for super::ListModelsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListModelsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ListModelsResponse> for ListModelsResponse {
        fn from(value: super::ListModelsResponse) -> Self {
            Self {
                data: Ok(value.data),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Model {
        created: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ModelObject, ::std::string::String>,
        owned_by: ::std::result::Result<::std::string::String, ::std::string::String>,
        shutdown_date: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Model {
        fn default() -> Self {
            Self {
                created: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                owned_by: Err("no value supplied for owned_by".to_string()),
                shutdown_date: Ok(Default::default()),
            }
        }
    }
    impl Model {
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
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
            T: ::std::convert::TryInto<super::ModelObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn owned_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.owned_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owned_by: {e}"));
            self
        }
        pub fn shutdown_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.shutdown_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shutdown_date: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Model> for super::Model {
        type Error = super::error::ConversionError;
        fn try_from(value: Model) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created: value.created?,
                id: value.id?,
                object: value.object?,
                owned_by: value.owned_by?,
                shutdown_date: value.shutdown_date?,
            })
        }
    }
    impl ::std::convert::From<super::Model> for Model {
        fn from(value: super::Model) -> Self {
            Self {
                created: Ok(value.created),
                id: Ok(value.id),
                object: Ok(value.object),
                owned_by: Ok(value.owned_by),
                shutdown_date: Ok(value.shutdown_date),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModelsApiSchema {
        delete_model_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_model_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_model_params: ::std::result::Result<super::DeleteModelParams, ::std::string::String>,
        delete_model_response:
            ::std::result::Result<super::DeleteModelResponse, ::std::string::String>,
        list_models_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_models_response:
            ::std::result::Result<super::ListModelsResponse, ::std::string::String>,
        retrieve_model_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_model_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_model_params:
            ::std::result::Result<super::RetrieveModelParams, ::std::string::String>,
        retrieve_model_response: ::std::result::Result<super::Model, ::std::string::String>,
    }
    impl ::std::default::Default for ModelsApiSchema {
        fn default() -> Self {
            Self {
                delete_model_error403: Err(
                    "no value supplied for delete_model_error403".to_string()
                ),
                delete_model_error404: Err(
                    "no value supplied for delete_model_error404".to_string()
                ),
                delete_model_params: Err("no value supplied for delete_model_params".to_string()),
                delete_model_response: Err(
                    "no value supplied for delete_model_response".to_string()
                ),
                list_models_error403: Err("no value supplied for list_models_error403".to_string()),
                list_models_response: Err("no value supplied for list_models_response".to_string()),
                retrieve_model_error403: Err(
                    "no value supplied for retrieve_model_error403".to_string()
                ),
                retrieve_model_error404: Err(
                    "no value supplied for retrieve_model_error404".to_string()
                ),
                retrieve_model_params: Err(
                    "no value supplied for retrieve_model_params".to_string()
                ),
                retrieve_model_response: Err(
                    "no value supplied for retrieve_model_response".to_string()
                ),
            }
        }
    }
    impl ModelsApiSchema {
        pub fn delete_model_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_model_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_model_error403: {e}")
            });
            self
        }
        pub fn delete_model_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_model_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_model_error404: {e}")
            });
            self
        }
        pub fn delete_model_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteModelParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_model_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_model_params: {e}")
            });
            self
        }
        pub fn delete_model_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteModelResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_model_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_model_response: {e}")
            });
            self
        }
        pub fn list_models_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_models_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_models_error403: {e}")
            });
            self
        }
        pub fn list_models_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListModelsResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_models_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_models_response: {e}")
            });
            self
        }
        pub fn retrieve_model_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_model_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_model_error403: {e}")
            });
            self
        }
        pub fn retrieve_model_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_model_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_model_error404: {e}")
            });
            self
        }
        pub fn retrieve_model_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveModelParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_model_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_model_params: {e}")
            });
            self
        }
        pub fn retrieve_model_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Model>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_model_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_model_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ModelsApiSchema> for super::ModelsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModelsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                delete_model_error403: value.delete_model_error403?,
                delete_model_error404: value.delete_model_error404?,
                delete_model_params: value.delete_model_params?,
                delete_model_response: value.delete_model_response?,
                list_models_error403: value.list_models_error403?,
                list_models_response: value.list_models_response?,
                retrieve_model_error403: value.retrieve_model_error403?,
                retrieve_model_error404: value.retrieve_model_error404?,
                retrieve_model_params: value.retrieve_model_params?,
                retrieve_model_response: value.retrieve_model_response?,
            })
        }
    }
    impl ::std::convert::From<super::ModelsApiSchema> for ModelsApiSchema {
        fn from(value: super::ModelsApiSchema) -> Self {
            Self {
                delete_model_error403: Ok(value.delete_model_error403),
                delete_model_error404: Ok(value.delete_model_error404),
                delete_model_params: Ok(value.delete_model_params),
                delete_model_response: Ok(value.delete_model_response),
                list_models_error403: Ok(value.list_models_error403),
                list_models_response: Ok(value.list_models_response),
                retrieve_model_error403: Ok(value.retrieve_model_error403),
                retrieve_model_error404: Ok(value.retrieve_model_error404),
                retrieve_model_params: Ok(value.retrieve_model_params),
                retrieve_model_response: Ok(value.retrieve_model_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveModelParams {
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveModelParams {
        fn default() -> Self {
            Self {
                model: Err("no value supplied for model".to_string()),
            }
        }
    }
    impl RetrieveModelParams {
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
    }
    impl ::std::convert::TryFrom<RetrieveModelParams> for super::RetrieveModelParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveModelParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                model: value.model?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveModelParams> for RetrieveModelParams {
        fn from(value: super::RetrieveModelParams) -> Self {
            Self {
                model: Ok(value.model),
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
