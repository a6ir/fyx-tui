use crossbeam_channel::{Receiver, Sender};

use crate::preview::worker::{PreviewRequest, PreviewResponse};

pub struct AppContext {
    pub preview_req_tx: Sender<PreviewRequest>,
    pub preview_res_rx: Receiver<PreviewResponse>,
}
