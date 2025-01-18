use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct QueryParams {
    #[serde(rename = "$select")]
    pub select: Option<String>,
    #[serde(rename = "$filter")]
    pub filter: Option<String>,
    #[serde(rename = "$orderby")]
    pub orderby: Option<String>,
    #[serde(rename = "$top")]
    pub top: Option<i32>,
    #[serde(rename = "$skip")]
    pub skip: Option<i32>,
    #[serde(rename = "$expand")]
    pub expand: Option<String>,
    #[serde(rename = "$compute")]
    pub compute: Option<String>,
}

