use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Category {
    MainGame = 0,
    DlcAddon = 1,
    Expansion = 2,
    Bundle = 3,
    StandaloneExpansion = 4,
}

impl Default for Category {
    fn default() -> Self {
        Category::MainGame
    }
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Gender {
    Male = 1,
    Female = 2,
    Other = 3,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
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

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Species {
    Human = 1,
    Alien = 2,
    Animal = 3,
    Android = 4,
    Unknown = 5,
}

impl Default for Species {
    fn default() -> Self {
        Species::Human
    }
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
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