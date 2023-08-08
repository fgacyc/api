use poem_openapi::payload::PlainText;

impl super::Routes {
    pub async fn _health(&self) -> PlainText<String> {
        PlainText("Healthy".to_string())
    }
}
