use std::collections::HashMap;

pub type Body = Vec<u8>;

pub struct Response {
    status: u16,
    headers: HashMap<&'static str, String>,
    pub send_body: bool,
    pub body: Body,
}

impl Response {
    pub fn new(status: u16, body: Body, send_body: bool, mime: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Length", body.len().to_string());
        headers.insert("Content-Type", mime.to_string());

        Self {
            status,
            headers,
            send_body,
            body,
        }
    }

    pub fn start_line(&self) -> String {
        let status = crate::status_with_name(self.status);
        format!("{} {}\n", crate::HTTP_VERSION, status)
    }

    pub fn header_lines(&self) -> String {
        let lines: Vec<String> = self
            .headers
            .iter()
            .map(|(name, value)| format!("{}: {}\n", name, value))
            .collect();
        lines.join("")
    }
}
