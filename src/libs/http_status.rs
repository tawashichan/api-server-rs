#[derive(Debug)]
pub enum HTTPStatus {
    BadRquest,
    Unauthorized,
    NotFound,
    InternalServerError,
}

impl HTTPStatus {
    pub fn status_code(&self) -> u64 {
        use HTTPStatus::*;
        match self {
            BadRquest => 400,
            Unauthorized => 401,
            NotFound => 404,
            InternalServerError => 500,
        }
    }
}
