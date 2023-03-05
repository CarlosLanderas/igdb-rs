use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum GameCategory {
    MainGame = 0,
    DlcAddon = 1,
    Expansion = 2,
    Bundle = 3,
    StandaloneExpansion = 4,
    Mod = 5,
    Episode = 6,
    Season = 7,
    Remake = 8,
    Remaster = 9,
    ExpandedGame = 10,
    Port = 11,
    Fork = 12,
    Pack = 13,
    Update = 14,
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
    Male = 0,
    Female = 1,
    Other = 2,
    Unknown = 3,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Unknown
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
    CeroA = 13,
    CeroB = 14,
    CeroC = 15,
    CeroD = 16,
    CeroZ = 17,
    Usk0 = 18,
    Usk6 = 19,
    Usk12 = 20,
    Usk18 = 21,
    GracAll = 22,
    GracTwelve = 23,
    GracFifteen = 24,
    GracEighteen = 25,
    GracTesting = 26,
    ClassIndL = 27,
    ClassIndTen = 28,
    ClassIndTwelve = 29,
    ClassIndFourteen = 30,
    ClassIndSixteen = 31,
    ClassIndEighteen = 32,
    AcbG = 33,
    AcbPg = 34,
    AcbM = 35,
    AcbMa15 = 36,
    AcbR18 = 37,
    AcbRc = 38,
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum RatingCategory {
    ESRB = 1,
    PEGI = 2,
    CERO = 3,
    USK = 4,
    GRAC = 5,
    ClassInd = 6,
    ACB = 7,
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
    Rumored = 7,
    Delisted = 8,
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
    Korea = 9,
    Brazil = 10,
}

impl Default for Region {
    fn default() -> Self {
        Region::Europe
    }
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum ExternalGameCategory {
    Steam = 1,
    Gog = 5,
    Youtube = 10,
    Microsoft = 11,
    Apple = 13,
    Twitch = 14,
    Android = 15,
    AmazonAsin = 20,
    AmazonLuna = 22,
    AmazonAdg = 23,
    EpicGameStore = 26,
    Oculus = 28,
    Utomik = 29,
    ItchIo = 30,
    XboxMarketplace = 31,
    Kartridge = 32,
    PlaystationStoreUs = 36,
    FocusEntertainment = 37,
    XboxGamePassUltimateCloud = 54,
    Gamejolt = 55,
}

impl Default for ExternalGameCategory {
    fn default() -> Self {
        ExternalGameCategory::Steam
    }
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum Media {
    Digital = 1,
    Physical = 5,
}

impl Default for Media {
    fn default() -> Self {
        Media::Digital
    }
}
