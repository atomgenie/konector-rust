#[derive(Debug)]
pub struct KonectorError {
    detail: String,
}

impl KonectorError {
    pub fn new(detail: &str) -> Box<KonectorError> {
        Box::new(KonectorError {
            detail: detail.to_string(),
        })
    }
}

impl std::fmt::Display for KonectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.detail)
    }
}

impl std::error::Error for KonectorError {}
