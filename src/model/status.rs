#[derive(Deserialize)]
pub enum Status {
    Released = 0,
    Alpha = 2,
    Beta = 3,
    EarlyAccess = 4,
    Offline = 5,
    Cancelled = 6,
}

impl Default for Status {
    fn default() -> Self {
        Status::Released
    }
}
