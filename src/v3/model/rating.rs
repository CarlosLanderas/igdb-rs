use crate::v3::model::Category;

pub enum Rating {
    Three = 1,
    Seven = 2,
    Twelve = 3,
    Sixteen = 4,
    Eighteen = 5,
    RP = 6,
    EC = 7,
    E = 8,
    E10 = 9,
    T = 10,
    M = 11,
    AO = 12,
}

pub struct AgeRating {
    category: Category,
    content_descriptions: usize,
    rating: Rating,
    rating_cover_url: String,
    synopsis: String,
}
