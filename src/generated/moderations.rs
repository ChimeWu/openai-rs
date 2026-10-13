#[doc = "`CreateModerationRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateModerationRequest {
    #[doc = "Input (or inputs) to classify. Can be a single string, an array of strings, or\nan array of multi-modal input objects similar to other models.\n"]
    pub input: CreateModerationRequestInput,
    #[doc = "The content moderation model you would like to use. Learn more in\n[the moderation guide](https://developers.openai.com/api/docs/guides/moderation), and learn about\navailable models [here](https://developers.openai.com/api/docs/guides/moderation).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
}
impl CreateModerationRequest {
    pub fn builder() -> builder::CreateModerationRequest {
        Default::default()
    }
}
#[doc = "Input (or inputs) to classify. Can be a single string, an array of strings, or\nan array of multi-modal input objects similar to other models.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateModerationRequestInput {
    Variant0(::std::string::String),
    Variant1(::std::vec::Vec<::std::string::String>),
    Variant2(::std::vec::Vec<CreateModerationRequestInputVariant2Item>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for CreateModerationRequestInput {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<CreateModerationRequestInputVariant2Item>>
    for CreateModerationRequestInput
{
    fn from(value: ::std::vec::Vec<CreateModerationRequestInputVariant2Item>) -> Self {
        Self::Variant2(value)
    }
}
#[doc = "`CreateModerationRequestInputVariant2Item`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum CreateModerationRequestInputVariant2Item {
    #[doc = "An object describing an image to classify."]
    #[serde(rename = "image_url")]
    ImageUrl {
        image_url: CreateModerationRequestInputVariant2ItemImageUrl,
    },
    #[doc = "An object describing text to classify."]
    #[serde(rename = "text")]
    Text {
        #[doc = "A string of text to classify."]
        text: ::std::string::String,
    },
}
#[doc = "Contains either an image URL or a data URL for a base64 encoded image."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateModerationRequestInputVariant2ItemImageUrl {
    #[doc = "Either a URL of the image or the base64 encoded image data."]
    pub url: ::std::string::String,
}
impl CreateModerationRequestInputVariant2ItemImageUrl {
    pub fn builder() -> builder::CreateModerationRequestInputVariant2ItemImageUrl {
        Default::default()
    }
}
#[doc = "Represents if a given text input is potentially harmful."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateModerationResponse {
    #[doc = "The unique identifier for the moderation request."]
    pub id: ::std::string::String,
    #[doc = "The model used to generate the moderation results."]
    pub model: ::std::string::String,
    #[doc = "A list of moderation objects."]
    pub results: ::std::vec::Vec<CreateModerationResponseResultsItem>,
}
impl CreateModerationResponse {
    pub fn builder() -> builder::CreateModerationResponse {
        Default::default()
    }
}
#[doc = "`CreateModerationResponseResultsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateModerationResponseResultsItem {
    pub categories: CreateModerationResponseResultsItemCategories,
    pub category_applied_input_types: CreateModerationResponseResultsItemCategoryAppliedInputTypes,
    pub category_scores: CreateModerationResponseResultsItemCategoryScores,
    #[doc = "Whether any of the below categories are flagged."]
    pub flagged: bool,
}
impl CreateModerationResponseResultsItem {
    pub fn builder() -> builder::CreateModerationResponseResultsItem {
        Default::default()
    }
}
#[doc = "A list of the categories, and whether they are flagged or not."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateModerationResponseResultsItemCategories {
    #[doc = "Content that expresses, incites, or promotes harassing language towards any target."]
    pub harassment: bool,
    #[doc = "Harassment content that also includes violence or serious harm towards any target."]
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
    #[doc = "Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste. Hateful content aimed at non-protected groups (e.g., chess players) is harassment."]
    pub hate: bool,
    #[doc = "Hateful content that also includes violence or serious harm towards the targeted group based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste."]
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub illicit: ::std::option::Option<bool>,
    #[serde(
        rename = "illicit/violent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub illicit_violent: ::std::option::Option<bool>,
    #[doc = "Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders."]
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    #[doc = "Content that encourages performing acts of self-harm, such as suicide, cutting, and eating disorders, or that gives instructions or advice on how to commit such acts."]
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,
    #[doc = "Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm, such as suicide, cutting, and eating disorders."]
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,
    #[doc = "Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness)."]
    pub sexual: bool,
    #[doc = "Sexual content that includes an individual who is under 18 years old."]
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    #[doc = "Content that depicts death, violence, or physical injury."]
    pub violence: bool,
    #[doc = "Content that depicts death, violence, or physical injury in graphic detail."]
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}
impl CreateModerationResponseResultsItemCategories {
    pub fn builder() -> builder::CreateModerationResponseResultsItemCategories {
        Default::default()
    }
}
#[doc = "A list of the categories along with the input type(s) that the score applies to."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateModerationResponseResultsItemCategoryAppliedInputTypes {
    #[doc = "The applied input type(s) for the category 'harassment'."]
    pub harassment:
        ::std::vec::Vec<CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentItem>,
    #[doc = "The applied input type(s) for the category 'harassment/threatening'."]
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: ::std::vec::Vec<
        CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentThreateningItem,
    >,
    #[doc = "The applied input type(s) for the category 'hate'."]
    pub hate: ::std::vec::Vec<CreateModerationResponseResultsItemCategoryAppliedInputTypesHateItem>,
    #[doc = "The applied input type(s) for the category 'hate/threatening'."]
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: ::std::vec::Vec<
        CreateModerationResponseResultsItemCategoryAppliedInputTypesHateThreateningItem,
    >,
    #[doc = "The applied input type(s) for the category 'illicit'."]
    pub illicit:
        ::std::vec::Vec<CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitItem>,
    #[doc = "The applied input type(s) for the category 'illicit/violent'."]
    #[serde(rename = "illicit/violent")]
    pub illicit_violent: ::std::vec::Vec<
        CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitViolentItem,
    >,
    #[doc = "The applied input type(s) for the category 'self-harm'."]
    #[serde(rename = "self-harm")]
    pub self_harm:
        ::std::vec::Vec<CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmItem>,
    #[doc = "The applied input type(s) for the category 'self-harm/instructions'."]
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: ::std::vec::Vec<
        CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmInstructionsItem,
    >,
    #[doc = "The applied input type(s) for the category 'self-harm/intent'."]
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: ::std::vec::Vec<
        CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmIntentItem,
    >,
    #[doc = "The applied input type(s) for the category 'sexual'."]
    pub sexual:
        ::std::vec::Vec<CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualItem>,
    #[doc = "The applied input type(s) for the category 'sexual/minors'."]
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: ::std::vec::Vec<
        CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualMinorsItem,
    >,
    #[doc = "The applied input type(s) for the category 'violence'."]
    pub violence:
        ::std::vec::Vec<CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceItem>,
    #[doc = "The applied input type(s) for the category 'violence/graphic'."]
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: ::std::vec::Vec<
        CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceGraphicItem,
    >,
}
impl CreateModerationResponseResultsItemCategoryAppliedInputTypes {
    pub fn builder() -> builder::CreateModerationResponseResultsItemCategoryAppliedInputTypes {
        Default::default()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentItem {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentThreateningItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentThreateningItem {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentThreateningItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentThreateningItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentThreateningItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentThreateningItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesHateItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesHateItem {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for CreateModerationResponseResultsItemCategoryAppliedInputTypesHateItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for CreateModerationResponseResultsItemCategoryAppliedInputTypesHateItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHateItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHateItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesHateThreateningItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesHateThreateningItem {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHateThreateningItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHateThreateningItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHateThreateningItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesHateThreateningItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitItem {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitViolentItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitViolentItem {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitViolentItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitViolentItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitViolentItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitViolentItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmInstructionsItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmInstructionsItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmInstructionsItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmInstructionsItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmInstructionsItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmInstructionsItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmIntentItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmIntentItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmIntentItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmIntentItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmIntentItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmIntentItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualMinorsItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualMinorsItem {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualMinorsItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualMinorsItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualMinorsItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualMinorsItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceGraphicItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceGraphicItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceGraphicItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceGraphicItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceGraphicItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceGraphicItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceItem`"]
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
pub enum CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceItem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceItem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceItem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A list of the categories along with their scores as predicted by model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateModerationResponseResultsItemCategoryScores {
    #[doc = "The score for the category 'harassment'."]
    pub harassment: f64,
    #[doc = "The score for the category 'harassment/threatening'."]
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f64,
    #[doc = "The score for the category 'hate'."]
    pub hate: f64,
    #[doc = "The score for the category 'hate/threatening'."]
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,
    #[doc = "The score for the category 'illicit'."]
    pub illicit: f64,
    #[doc = "The score for the category 'illicit/violent'."]
    #[serde(rename = "illicit/violent")]
    pub illicit_violent: f64,
    #[doc = "The score for the category 'self-harm'."]
    #[serde(rename = "self-harm")]
    pub self_harm: f64,
    #[doc = "The score for the category 'self-harm/instructions'."]
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f64,
    #[doc = "The score for the category 'self-harm/intent'."]
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f64,
    #[doc = "The score for the category 'sexual'."]
    pub sexual: f64,
    #[doc = "The score for the category 'sexual/minors'."]
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,
    #[doc = "The score for the category 'violence'."]
    pub violence: f64,
    #[doc = "The score for the category 'violence/graphic'."]
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
}
impl CreateModerationResponseResultsItemCategoryScores {
    pub fn builder() -> builder::CreateModerationResponseResultsItemCategoryScores {
        Default::default()
    }
}
#[doc = "`ModerationsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModerationsApiSchema {
    #[serde(rename = "CreateModerationRequest")]
    pub create_moderation_request: CreateModerationRequest,
    #[serde(rename = "CreateModerationResponse")]
    pub create_moderation_response: CreateModerationResponse,
}
impl ModerationsApiSchema {
    pub fn builder() -> builder::ModerationsApiSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateModerationRequest {
        input: ::std::result::Result<super::CreateModerationRequestInput, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateModerationRequest {
        fn default() -> Self {
            Self {
                input: Err("no value supplied for input".to_string()),
                model: Ok(Default::default()),
            }
        }
    }
    impl CreateModerationRequest {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateModerationRequestInput>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateModerationRequest> for super::CreateModerationRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateModerationRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                model: value.model?,
            })
        }
    }
    impl ::std::convert::From<super::CreateModerationRequest> for CreateModerationRequest {
        fn from(value: super::CreateModerationRequest) -> Self {
            Self {
                input: Ok(value.input),
                model: Ok(value.model),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateModerationRequestInputVariant2ItemImageUrl {
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateModerationRequestInputVariant2ItemImageUrl {
        fn default() -> Self {
            Self {
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl CreateModerationRequestInputVariant2ItemImageUrl {
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateModerationRequestInputVariant2ItemImageUrl>
        for super::CreateModerationRequestInputVariant2ItemImageUrl
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateModerationRequestInputVariant2ItemImageUrl,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { url: value.url? })
        }
    }
    impl ::std::convert::From<super::CreateModerationRequestInputVariant2ItemImageUrl>
        for CreateModerationRequestInputVariant2ItemImageUrl
    {
        fn from(value: super::CreateModerationRequestInputVariant2ItemImageUrl) -> Self {
            Self { url: Ok(value.url) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateModerationResponse {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::CreateModerationResponseResultsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateModerationResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                model: Err("no value supplied for model".to_string()),
                results: Err("no value supplied for results".to_string()),
            }
        }
    }
    impl CreateModerationResponse {
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
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CreateModerationResponseResultsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateModerationResponse> for super::CreateModerationResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateModerationResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                model: value.model?,
                results: value.results?,
            })
        }
    }
    impl ::std::convert::From<super::CreateModerationResponse> for CreateModerationResponse {
        fn from(value: super::CreateModerationResponse) -> Self {
            Self {
                id: Ok(value.id),
                model: Ok(value.model),
                results: Ok(value.results),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateModerationResponseResultsItem {
        categories: ::std::result::Result<
            super::CreateModerationResponseResultsItemCategories,
            ::std::string::String,
        >,
        category_applied_input_types: ::std::result::Result<
            super::CreateModerationResponseResultsItemCategoryAppliedInputTypes,
            ::std::string::String,
        >,
        category_scores: ::std::result::Result<
            super::CreateModerationResponseResultsItemCategoryScores,
            ::std::string::String,
        >,
        flagged: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for CreateModerationResponseResultsItem {
        fn default() -> Self {
            Self {
                categories: Err("no value supplied for categories".to_string()),
                category_applied_input_types: Err(
                    "no value supplied for category_applied_input_types".to_string(),
                ),
                category_scores: Err("no value supplied for category_scores".to_string()),
                flagged: Err("no value supplied for flagged".to_string()),
            }
        }
    }
    impl CreateModerationResponseResultsItem {
        pub fn categories<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateModerationResponseResultsItemCategories>,
            T::Error: ::std::fmt::Display,
        {
            self.categories = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for categories: {e}"));
            self
        }
        pub fn category_applied_input_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CreateModerationResponseResultsItemCategoryAppliedInputTypes,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.category_applied_input_types = value.try_into().map_err(|e| {
                format!("error converting supplied value for category_applied_input_types: {e}")
            });
            self
        }
        pub fn category_scores<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateModerationResponseResultsItemCategoryScores>,
            T::Error: ::std::fmt::Display,
        {
            self.category_scores = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category_scores: {e}"));
            self
        }
        pub fn flagged<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.flagged = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flagged: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateModerationResponseResultsItem>
        for super::CreateModerationResponseResultsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateModerationResponseResultsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                categories: value.categories?,
                category_applied_input_types: value.category_applied_input_types?,
                category_scores: value.category_scores?,
                flagged: value.flagged?,
            })
        }
    }
    impl ::std::convert::From<super::CreateModerationResponseResultsItem>
        for CreateModerationResponseResultsItem
    {
        fn from(value: super::CreateModerationResponseResultsItem) -> Self {
            Self {
                categories: Ok(value.categories),
                category_applied_input_types: Ok(value.category_applied_input_types),
                category_scores: Ok(value.category_scores),
                flagged: Ok(value.flagged),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateModerationResponseResultsItemCategories {
        harassment: ::std::result::Result<bool, ::std::string::String>,
        harassment_threatening: ::std::result::Result<bool, ::std::string::String>,
        hate: ::std::result::Result<bool, ::std::string::String>,
        hate_threatening: ::std::result::Result<bool, ::std::string::String>,
        illicit: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        illicit_violent: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        self_harm: ::std::result::Result<bool, ::std::string::String>,
        self_harm_instructions: ::std::result::Result<bool, ::std::string::String>,
        self_harm_intent: ::std::result::Result<bool, ::std::string::String>,
        sexual: ::std::result::Result<bool, ::std::string::String>,
        sexual_minors: ::std::result::Result<bool, ::std::string::String>,
        violence: ::std::result::Result<bool, ::std::string::String>,
        violence_graphic: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for CreateModerationResponseResultsItemCategories {
        fn default() -> Self {
            Self {
                harassment: Err("no value supplied for harassment".to_string()),
                harassment_threatening: Err(
                    "no value supplied for harassment_threatening".to_string()
                ),
                hate: Err("no value supplied for hate".to_string()),
                hate_threatening: Err("no value supplied for hate_threatening".to_string()),
                illicit: Err("no value supplied for illicit".to_string()),
                illicit_violent: Err("no value supplied for illicit_violent".to_string()),
                self_harm: Err("no value supplied for self_harm".to_string()),
                self_harm_instructions: Err(
                    "no value supplied for self_harm_instructions".to_string()
                ),
                self_harm_intent: Err("no value supplied for self_harm_intent".to_string()),
                sexual: Err("no value supplied for sexual".to_string()),
                sexual_minors: Err("no value supplied for sexual_minors".to_string()),
                violence: Err("no value supplied for violence".to_string()),
                violence_graphic: Err("no value supplied for violence_graphic".to_string()),
            }
        }
    }
    impl CreateModerationResponseResultsItemCategories {
        pub fn harassment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.harassment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for harassment: {e}"));
            self
        }
        pub fn harassment_threatening<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.harassment_threatening = value.try_into().map_err(|e| {
                format!("error converting supplied value for harassment_threatening: {e}")
            });
            self
        }
        pub fn hate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.hate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hate: {e}"));
            self
        }
        pub fn hate_threatening<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.hate_threatening = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hate_threatening: {e}"));
            self
        }
        pub fn illicit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.illicit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for illicit: {e}"));
            self
        }
        pub fn illicit_violent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.illicit_violent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for illicit_violent: {e}"));
            self
        }
        pub fn self_harm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.self_harm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for self_harm: {e}"));
            self
        }
        pub fn self_harm_instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.self_harm_instructions = value.try_into().map_err(|e| {
                format!("error converting supplied value for self_harm_instructions: {e}")
            });
            self
        }
        pub fn self_harm_intent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.self_harm_intent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for self_harm_intent: {e}"));
            self
        }
        pub fn sexual<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.sexual = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sexual: {e}"));
            self
        }
        pub fn sexual_minors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.sexual_minors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sexual_minors: {e}"));
            self
        }
        pub fn violence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.violence = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for violence: {e}"));
            self
        }
        pub fn violence_graphic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.violence_graphic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for violence_graphic: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateModerationResponseResultsItemCategories>
        for super::CreateModerationResponseResultsItemCategories
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateModerationResponseResultsItemCategories,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                harassment: value.harassment?,
                harassment_threatening: value.harassment_threatening?,
                hate: value.hate?,
                hate_threatening: value.hate_threatening?,
                illicit: value.illicit?,
                illicit_violent: value.illicit_violent?,
                self_harm: value.self_harm?,
                self_harm_instructions: value.self_harm_instructions?,
                self_harm_intent: value.self_harm_intent?,
                sexual: value.sexual?,
                sexual_minors: value.sexual_minors?,
                violence: value.violence?,
                violence_graphic: value.violence_graphic?,
            })
        }
    }
    impl ::std::convert::From<super::CreateModerationResponseResultsItemCategories>
        for CreateModerationResponseResultsItemCategories
    {
        fn from(value: super::CreateModerationResponseResultsItemCategories) -> Self {
            Self {
                harassment: Ok(value.harassment),
                harassment_threatening: Ok(value.harassment_threatening),
                hate: Ok(value.hate),
                hate_threatening: Ok(value.hate_threatening),
                illicit: Ok(value.illicit),
                illicit_violent: Ok(value.illicit_violent),
                self_harm: Ok(value.self_harm),
                self_harm_instructions: Ok(value.self_harm_instructions),
                self_harm_intent: Ok(value.self_harm_intent),
                sexual: Ok(value.sexual),
                sexual_minors: Ok(value.sexual_minors),
                violence: Ok(value.violence),
                violence_graphic: Ok(value.violence_graphic),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateModerationResponseResultsItemCategoryAppliedInputTypes { harassment : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentItem > , :: std :: string :: String > , harassment_threatening : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentThreateningItem > , :: std :: string :: String > , hate : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesHateItem > , :: std :: string :: String > , hate_threatening : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesHateThreateningItem > , :: std :: string :: String > , illicit : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitItem > , :: std :: string :: String > , illicit_violent : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitViolentItem > , :: std :: string :: String > , self_harm : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmItem > , :: std :: string :: String > , self_harm_instructions : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmInstructionsItem > , :: std :: string :: String > , self_harm_intent : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmIntentItem > , :: std :: string :: String > , sexual : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualItem > , :: std :: string :: String > , sexual_minors : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualMinorsItem > , :: std :: string :: String > , violence : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceItem > , :: std :: string :: String > , violence_graphic : :: std :: result :: Result < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceGraphicItem > , :: std :: string :: String > , }
    impl ::std::default::Default for CreateModerationResponseResultsItemCategoryAppliedInputTypes {
        fn default() -> Self {
            Self {
                harassment: Err("no value supplied for harassment".to_string()),
                harassment_threatening: Err(
                    "no value supplied for harassment_threatening".to_string()
                ),
                hate: Err("no value supplied for hate".to_string()),
                hate_threatening: Err("no value supplied for hate_threatening".to_string()),
                illicit: Err("no value supplied for illicit".to_string()),
                illicit_violent: Err("no value supplied for illicit_violent".to_string()),
                self_harm: Err("no value supplied for self_harm".to_string()),
                self_harm_instructions: Err(
                    "no value supplied for self_harm_instructions".to_string()
                ),
                self_harm_intent: Err("no value supplied for self_harm_intent".to_string()),
                sexual: Err("no value supplied for sexual".to_string()),
                sexual_minors: Err("no value supplied for sexual_minors".to_string()),
                violence: Err("no value supplied for violence".to_string()),
                violence_graphic: Err("no value supplied for violence_graphic".to_string()),
            }
        }
    }
    impl CreateModerationResponseResultsItemCategoryAppliedInputTypes {
        pub fn harassment < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentItem > > , T :: Error : :: std :: fmt :: Display ,{
            self.harassment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for harassment: {e}"));
            self
        }        pub fn harassment_threatening < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesHarassmentThreateningItem > > , T :: Error : :: std :: fmt :: Display ,{
            self.harassment_threatening = value.try_into().map_err(|e| {
                format!("error converting supplied value for harassment_threatening: {e}")
            });
            self
        }
        pub fn hate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<
                    super::CreateModerationResponseResultsItemCategoryAppliedInputTypesHateItem,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.hate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hate: {e}"));
            self
        }        pub fn hate_threatening < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesHateThreateningItem > > , T :: Error : :: std :: fmt :: Display ,{
            self.hate_threatening = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hate_threatening: {e}"));
            self
        }
        pub fn illicit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<
                    super::CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitItem,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.illicit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for illicit: {e}"));
            self
        }        pub fn illicit_violent < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesIllicitViolentItem > > , T :: Error : :: std :: fmt :: Display ,{
            self.illicit_violent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for illicit_violent: {e}"));
            self
        }
        pub fn self_harm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<
                    super::CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmItem,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.self_harm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for self_harm: {e}"));
            self
        }        pub fn self_harm_instructions < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmInstructionsItem > > , T :: Error : :: std :: fmt :: Display ,{
            self.self_harm_instructions = value.try_into().map_err(|e| {
                format!("error converting supplied value for self_harm_instructions: {e}")
            });
            self
        }        pub fn self_harm_intent < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesSelfHarmIntentItem > > , T :: Error : :: std :: fmt :: Display ,{
            self.self_harm_intent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for self_harm_intent: {e}"));
            self
        }
        pub fn sexual<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<
                    super::CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualItem,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sexual = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sexual: {e}"));
            self
        }        pub fn sexual_minors < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesSexualMinorsItem > > , T :: Error : :: std :: fmt :: Display ,{
            self.sexual_minors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sexual_minors: {e}"));
            self
        }
        pub fn violence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<
                    super::CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceItem,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.violence = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for violence: {e}"));
            self
        }        pub fn violence_graphic < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: vec :: Vec < super :: CreateModerationResponseResultsItemCategoryAppliedInputTypesViolenceGraphicItem > > , T :: Error : :: std :: fmt :: Display ,{
            self.violence_graphic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for violence_graphic: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateModerationResponseResultsItemCategoryAppliedInputTypes>
        for super::CreateModerationResponseResultsItemCategoryAppliedInputTypes
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateModerationResponseResultsItemCategoryAppliedInputTypes,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                harassment: value.harassment?,
                harassment_threatening: value.harassment_threatening?,
                hate: value.hate?,
                hate_threatening: value.hate_threatening?,
                illicit: value.illicit?,
                illicit_violent: value.illicit_violent?,
                self_harm: value.self_harm?,
                self_harm_instructions: value.self_harm_instructions?,
                self_harm_intent: value.self_harm_intent?,
                sexual: value.sexual?,
                sexual_minors: value.sexual_minors?,
                violence: value.violence?,
                violence_graphic: value.violence_graphic?,
            })
        }
    }
    impl ::std::convert::From<super::CreateModerationResponseResultsItemCategoryAppliedInputTypes>
        for CreateModerationResponseResultsItemCategoryAppliedInputTypes
    {
        fn from(
            value: super::CreateModerationResponseResultsItemCategoryAppliedInputTypes,
        ) -> Self {
            Self {
                harassment: Ok(value.harassment),
                harassment_threatening: Ok(value.harassment_threatening),
                hate: Ok(value.hate),
                hate_threatening: Ok(value.hate_threatening),
                illicit: Ok(value.illicit),
                illicit_violent: Ok(value.illicit_violent),
                self_harm: Ok(value.self_harm),
                self_harm_instructions: Ok(value.self_harm_instructions),
                self_harm_intent: Ok(value.self_harm_intent),
                sexual: Ok(value.sexual),
                sexual_minors: Ok(value.sexual_minors),
                violence: Ok(value.violence),
                violence_graphic: Ok(value.violence_graphic),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateModerationResponseResultsItemCategoryScores {
        harassment: ::std::result::Result<f64, ::std::string::String>,
        harassment_threatening: ::std::result::Result<f64, ::std::string::String>,
        hate: ::std::result::Result<f64, ::std::string::String>,
        hate_threatening: ::std::result::Result<f64, ::std::string::String>,
        illicit: ::std::result::Result<f64, ::std::string::String>,
        illicit_violent: ::std::result::Result<f64, ::std::string::String>,
        self_harm: ::std::result::Result<f64, ::std::string::String>,
        self_harm_instructions: ::std::result::Result<f64, ::std::string::String>,
        self_harm_intent: ::std::result::Result<f64, ::std::string::String>,
        sexual: ::std::result::Result<f64, ::std::string::String>,
        sexual_minors: ::std::result::Result<f64, ::std::string::String>,
        violence: ::std::result::Result<f64, ::std::string::String>,
        violence_graphic: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for CreateModerationResponseResultsItemCategoryScores {
        fn default() -> Self {
            Self {
                harassment: Err("no value supplied for harassment".to_string()),
                harassment_threatening: Err(
                    "no value supplied for harassment_threatening".to_string()
                ),
                hate: Err("no value supplied for hate".to_string()),
                hate_threatening: Err("no value supplied for hate_threatening".to_string()),
                illicit: Err("no value supplied for illicit".to_string()),
                illicit_violent: Err("no value supplied for illicit_violent".to_string()),
                self_harm: Err("no value supplied for self_harm".to_string()),
                self_harm_instructions: Err(
                    "no value supplied for self_harm_instructions".to_string()
                ),
                self_harm_intent: Err("no value supplied for self_harm_intent".to_string()),
                sexual: Err("no value supplied for sexual".to_string()),
                sexual_minors: Err("no value supplied for sexual_minors".to_string()),
                violence: Err("no value supplied for violence".to_string()),
                violence_graphic: Err("no value supplied for violence_graphic".to_string()),
            }
        }
    }
    impl CreateModerationResponseResultsItemCategoryScores {
        pub fn harassment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.harassment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for harassment: {e}"));
            self
        }
        pub fn harassment_threatening<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.harassment_threatening = value.try_into().map_err(|e| {
                format!("error converting supplied value for harassment_threatening: {e}")
            });
            self
        }
        pub fn hate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.hate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hate: {e}"));
            self
        }
        pub fn hate_threatening<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.hate_threatening = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hate_threatening: {e}"));
            self
        }
        pub fn illicit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.illicit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for illicit: {e}"));
            self
        }
        pub fn illicit_violent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.illicit_violent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for illicit_violent: {e}"));
            self
        }
        pub fn self_harm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.self_harm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for self_harm: {e}"));
            self
        }
        pub fn self_harm_instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.self_harm_instructions = value.try_into().map_err(|e| {
                format!("error converting supplied value for self_harm_instructions: {e}")
            });
            self
        }
        pub fn self_harm_intent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.self_harm_intent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for self_harm_intent: {e}"));
            self
        }
        pub fn sexual<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.sexual = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sexual: {e}"));
            self
        }
        pub fn sexual_minors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.sexual_minors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sexual_minors: {e}"));
            self
        }
        pub fn violence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.violence = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for violence: {e}"));
            self
        }
        pub fn violence_graphic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.violence_graphic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for violence_graphic: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateModerationResponseResultsItemCategoryScores>
        for super::CreateModerationResponseResultsItemCategoryScores
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateModerationResponseResultsItemCategoryScores,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                harassment: value.harassment?,
                harassment_threatening: value.harassment_threatening?,
                hate: value.hate?,
                hate_threatening: value.hate_threatening?,
                illicit: value.illicit?,
                illicit_violent: value.illicit_violent?,
                self_harm: value.self_harm?,
                self_harm_instructions: value.self_harm_instructions?,
                self_harm_intent: value.self_harm_intent?,
                sexual: value.sexual?,
                sexual_minors: value.sexual_minors?,
                violence: value.violence?,
                violence_graphic: value.violence_graphic?,
            })
        }
    }
    impl ::std::convert::From<super::CreateModerationResponseResultsItemCategoryScores>
        for CreateModerationResponseResultsItemCategoryScores
    {
        fn from(value: super::CreateModerationResponseResultsItemCategoryScores) -> Self {
            Self {
                harassment: Ok(value.harassment),
                harassment_threatening: Ok(value.harassment_threatening),
                hate: Ok(value.hate),
                hate_threatening: Ok(value.hate_threatening),
                illicit: Ok(value.illicit),
                illicit_violent: Ok(value.illicit_violent),
                self_harm: Ok(value.self_harm),
                self_harm_instructions: Ok(value.self_harm_instructions),
                self_harm_intent: Ok(value.self_harm_intent),
                sexual: Ok(value.sexual),
                sexual_minors: Ok(value.sexual_minors),
                violence: Ok(value.violence),
                violence_graphic: Ok(value.violence_graphic),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModerationsApiSchema {
        create_moderation_request:
            ::std::result::Result<super::CreateModerationRequest, ::std::string::String>,
        create_moderation_response:
            ::std::result::Result<super::CreateModerationResponse, ::std::string::String>,
    }
    impl ::std::default::Default for ModerationsApiSchema {
        fn default() -> Self {
            Self {
                create_moderation_request: Err(
                    "no value supplied for create_moderation_request".to_string()
                ),
                create_moderation_response: Err(
                    "no value supplied for create_moderation_response".to_string()
                ),
            }
        }
    }
    impl ModerationsApiSchema {
        pub fn create_moderation_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateModerationRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_moderation_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_moderation_request: {e}")
            });
            self
        }
        pub fn create_moderation_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateModerationResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_moderation_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_moderation_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ModerationsApiSchema> for super::ModerationsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModerationsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_moderation_request: value.create_moderation_request?,
                create_moderation_response: value.create_moderation_response?,
            })
        }
    }
    impl ::std::convert::From<super::ModerationsApiSchema> for ModerationsApiSchema {
        fn from(value: super::ModerationsApiSchema) -> Self {
            Self {
                create_moderation_request: Ok(value.create_moderation_request),
                create_moderation_response: Ok(value.create_moderation_response),
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
