use_client_imports!();
use surf::Exception;

//create_client! macro automatically generates clients
//for different endpoints and extends IGDBClient struct

//params: (ClientName, EntityResult, IGDB client impl method)

create_client!(ArtworksClient, Artwork, artworks);
create_client!(CharacterClient, Character, characters);
create_client!(CompanyClient, Company, companies);
create_client!(CoversClient, Cover, covers);
create_client!(GameClient, Game, games);
create_client!(GameModesClient, GameMode, game_modes);
create_client!(MultiPlayerModeClient, MultiplayerMode, multiplayer_modes);
create_client!(ReleaseDateClient, ReleaseDate, release_dates);
create_client!(ScreenshotsClient, Screenshot, screenshots);
create_client!(WebsitesClient, Website, websites);


pub struct IGDBClient {
    api_key: String,
}

impl IGDBClient {
    pub fn new<S: Into<String>>(api_key: S) -> IGDBClient {
        IGDBClient {
            api_key: api_key.into(),
        }
    }
}
