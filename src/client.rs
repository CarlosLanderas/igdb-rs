use_client_imports!();
//create_client! macro automatically generates clients
//for different endpoints and extends IGDBClient struct

//params: (ClientName, EntityResult, IGDB client impl method)

create_client!(AgeRatingsClient, AgeRating, age_ratings);
create_client!(ArtworksClient, Artwork, artworks);
create_client!(
    CharacterMugshotsClient,
    CharacterMugshot,
    character_mug_shots
);
create_client!(CharactersClient, Character, characters);
create_client!(CompaniesClient, Company, companies);
create_client!(CoversClient, Cover, covers);
create_client!(GamesClient, Game, games);
create_client!(GameModesClient, GameMode, game_modes);
create_client!(GameVideosClient, GameVideo, game_videos);
create_client!(EnginesClient, Engine, game_engines);
create_client!(FranchisesClient, Franchise, franchises);
create_client!(MultiPlayerModesClient, MultiplayerMode, multiplayer_modes);
create_client!(PlatformsClient, Platform, platforms);
create_client!(PlatformLogosClient, PlatformLogo, platform_logos);
create_client!(
    PlayerPerpectivesClient,
    PlayerPerspective,
    player_perspectives
);
create_client!(ReleaseDatesClient, ReleaseDate, release_dates);
create_client!(ScreenshotsClient, Screenshot, screenshots);
create_client!(ThemesClient, Theme, themes);
create_client!(WebsitesClient, Website, websites);

expand_media_download!(ArtworksClient);
expand_media_download!(CoversClient);
expand_media_download!(ScreenshotsClient);
expand_media_download!(PlatformLogosClient);
expand_media_download!(CharacterMugshotsClient);

expand_get_by_game_id!(ArtworksClient, Artwork);
expand_get_by_game_id!(CoversClient, Cover);
expand_get_by_game_id!(GameVideosClient, GameVideo);
expand_get_by_game_id!(MultiPlayerModesClient, MultiplayerMode);
expand_get_by_game_id!(ReleaseDatesClient, ReleaseDate);
expand_get_by_game_id!(ScreenshotsClient, Screenshot);
expand_get_by_game_id!(WebsitesClient, Website);

pub struct IGDBClient {
    client_id: String,
    token: String,
}

// The IGDB client.
impl IGDBClient {
    /// Creates a new instance of the IGDB Client
    /// To create a new instance you need to provide the client_id
    /// and the token your generated using your client_id and secret
    /// provided when you register in the API Website
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    ///
    /// let igdb = IGDBClient::new("client_id", "token");
    /// ```
    pub fn new<S: Into<String>>(client_id: S, token: S) -> IGDBClient {
        IGDBClient {
            client_id: client_id.into(),
            token: token.into(),
        }
    }
    /// Creates a new instance of a Request builder
    /// you can use it's methods to create custom queries
    ///
    /// # Examples
    ///
    /// ```
    /// use igdb_rs::client::IGDBClient;
    /// use igdb_rs::request_builder::Equality;
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .all_fields()
    /// .add_where("id", Equality::Greater, "340")
    /// .limit(10);
    /// ```
    pub fn create_request() -> RequestBuilder {
        RequestBuilder::new()
    }
}
