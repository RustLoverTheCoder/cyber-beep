use axum::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload<T> {
    pub code: u16,
    pub data: Option<T>,
    pub message: String,
}

impl<T> Payload<T> {
    pub fn success(t: Option<T>) -> Json<Payload<T>> {
        Payload {
            data: t,
            code: 2000,
            message: "success".to_string(),
        }
        .into()
    }
}
