pub struct Request<'a> {
    method: &'a str,
    pub path: &'a str,
    proto: &'a str,
}

impl<'a> Request<'a> {
    pub fn from(str: &'a str) -> Option<Self> {
        let elements: Vec<&str> = str.split_whitespace().collect();
        if elements.len() != 3 {
            return None;
        }
        let method = elements[0];
        let path = elements[1];
        let proto = elements[2];

        Some(Self {
            method,
            path,
            proto,
        })
    }

    pub fn is_correct_proto(&self) -> bool {
        self.proto == crate::HTTP_VERSION
    }

    pub fn is_get(&self) -> bool {
        self.method == "GET"
    }

    pub fn is_head(&self) -> bool {
        self.method == "HEAD"
    }
}
