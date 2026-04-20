use std::{path::PathBuf, thread};

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::preview::render;

#[derive(Debug, Clone)]
pub struct PreviewRequest {
    pub token: u64,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PreviewResponse {
    pub token: u64,
    pub content: String,
}

pub fn start_worker() -> (Sender<PreviewRequest>, Receiver<PreviewResponse>) {
    let (tx_req, rx_req) = unbounded::<PreviewRequest>();
    let (tx_res, rx_res) = unbounded::<PreviewResponse>();

    thread::spawn(move || {
        while let Ok(request) = rx_req.recv() {
            let content = render::render_preview(&request.path);
            let _ = tx_res.send(PreviewResponse {
                token: request.token,
                content,
            });
        }
    });

    (tx_req, rx_res)
}
