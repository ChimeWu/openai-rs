#[doc = "`CreateModerationRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateModerationRequest {
    #[doc = "Input (or inputs) to classify. Can be a single string, an array of strings, or\nan array of multi-modal input objects similar to other models.\n"]
    pub input: CreateModerationRequestInput,
    #[doc = "The content moderation model you would like to use. Learn more in\n[the moderation guide](https://developers.openai.com/api/docs/guides/moderation), and learn about\navailable models [here](https://developers.openai.com/api/docs/guides/moderation).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
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
#[doc = "`CreateModerationResponseResultsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateModerationResponseResultsItem {
    pub categories: CreateModerationResponseResultsItemCategories,
    pub category_applied_input_types: CreateModerationResponseResultsItemCategoryAppliedInputTypes,
    pub category_scores: CreateModerationResponseResultsItemCategoryScores,
    #[doc = "Whether any of the below categories are flagged."]
    pub flagged: bool,
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
#[doc = "`ModerationsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModerationsApiSchema {
    #[serde(rename = "CreateModerationRequest")]
    pub create_moderation_request: CreateModerationRequest,
    #[serde(rename = "CreateModerationResponse")]
    pub create_moderation_response: CreateModerationResponse,
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
