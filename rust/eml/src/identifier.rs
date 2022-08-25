use uuid::Uuid;

pub fn newid() -> String {
    Uuid::new_v4().to_string()
}
