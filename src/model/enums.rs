use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum GameCategory {
    MainGame = 0,
    DlcAddon = 1,
    Expansion = 2,
    Bundle = 3,
    StandaloneExpansion = 4,
}

impl Default for GameCategory {
    fn default() -> Self {
        GameCategory::MainGame
    }
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum PlatformCategory {
    Console = 1,
    Arcade = 2,
    Platform = 3,
    OperativeSystem = 4,
    PortableConsole = 5,
    Computer = 6,
}

impl Default for PlatformCategory {
    fn default() -> Self {
        PlatformCategory::Console
    }
}

#[derive(Deserialize_repr, Debug, Clone)]
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

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum Platform {
    Console = 1,
    Arcade = 2,
    Platform = 3,
    OperativeSystem = 4,
    PortableConsole = 5,
    Computer = 6,
}

impl Default for Platform {
    fn default() -> Self {
        Platform::Console
    }
}

#[derive(Deserialize_repr, Debug, Clone)]
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

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum RatingCategory {
    ESRB = 1,
    PEGI = 2,
}

impl Default for RatingCategory {
    fn default() -> Self {
        RatingCategory::ESRB
    }
}

impl Default for Rating {
    fn default() -> Self {
        Rating::Eighteen
    }
}

#[derive(Deserialize_repr, Debug, Clone)]
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

#[derive(Deserialize_repr, Debug, Clone)]
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

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum Region {
    Europe = 1,
    NorthAmerica = 2,
    Australia = 3,
    NewZealand = 4,
    Japan = 5,
    China = 6,
    Asia = 7,
    WorldWide = 8,
}

impl Default for Region {
    fn default() -> Self {
        Region::Europe
    }
}
