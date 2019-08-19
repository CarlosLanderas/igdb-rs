pub mod games;
pub mod rating;

pub enum Category {
    ESRB = 1,
    PEGI = 2,
}

pub enum Status {
    Released = 0,
    Alpha = 2,
    Beta = 3,
    EarlyAccess = 4,
    Offline = 5,
    Cancelled = 6,
}

pub struct Keyword {
    created_at: u64,
    name: String,
    slug: String,
    updated_at: u64,
    url: String,
}
