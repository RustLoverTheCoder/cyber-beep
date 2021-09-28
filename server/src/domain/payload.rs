use axum::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload<T> {
    pub code: u16,
    pub data: Option<T>,
    pub message: String,
}

impl<T> Payload<T> {
    pub fn ok() -> Json<Payload<T>> {
        Payload {
            code: 2000,
            data: None,
            message: "success".to_string(),
        }
        .into()
    }

    pub fn success(t: T) -> Json<Payload<T>> {
        Payload {
            data: Some(t),
            ..Payload::ok().0
        }
        .into()
    }

    pub fn failure(code: u16, message: String) -> Json<Payload<T>> {
        Payload {
            code,
            message,
            data: None,
        }
        .into()
    }
}
