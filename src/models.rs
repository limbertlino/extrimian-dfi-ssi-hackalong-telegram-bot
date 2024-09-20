use serde::Deserialize;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveName,
    ReceiveCategory {
        name: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub _invitation_id: String,
    pub oob_content_data: String,
}
