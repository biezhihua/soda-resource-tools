use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FanartTV {
    pub name: Option<String>,
    pub thetvdb_id: Option<String>,
    pub tvbanner: Option<Vec<FanartImg>>,
    pub hdclearart: Option<Vec<FanartImg>>,
    pub characterart: Option<Vec<FanartImg>>,
    pub hdtvlogo: Option<Vec<FanartImg>>,
    pub tvthumb: Option<Vec<FanartImg>>,
    pub showbackground: Option<Vec<FanartImg>>,
    pub seasonposter: Option<Vec<FanartImg>>,
    pub tvposter: Option<Vec<FanartImg>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FanartImg {
    pub id: Option<String>,
    pub url: Option<String>,
    pub lang: Option<String>,
    pub likes: Option<String>,
    pub season: Option<String>,
}
impl FanartImg {
    pub(crate) fn url(&self) -> &str {
        match &self.url {
            Some(url) => url.as_str(),
            None => "",
        }
    }
}
