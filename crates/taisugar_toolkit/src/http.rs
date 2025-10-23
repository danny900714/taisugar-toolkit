use gpui::Global;
use ureq::Agent;

pub struct HttpClient(pub Agent);

impl Global for HttpClient {}
