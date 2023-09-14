use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct MsgRequest {
    pub(crate) tid: String,
    pub(crate) msg_type: MsgType,
    pub(crate) payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum MsgType {
    Subcribe = 1,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct MsgStream {
    pub(crate) channel: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct MsgAck {
    pub(crate) tid: String,
    pub(crate) error: String,
}
