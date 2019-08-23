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
create_client!(GameVideosClient, GameVideo, game_videos);
create_client!(EnginesClient, Engine, game_engines);
create_client!(MultiPlayerModeClient, MultiplayerMode, multiplayer_modes);
create_client!(ReleaseDateClient, ReleaseDate, release_dates);
create_client!(ScreenshotsClient, Screenshot, screenshots);
create_client!(WebsitesClient, Website, websites);
create_client!(PlatformsClient, Platform, platforms);

expand_media_download!(ArtworksClient);
expand_media_download!(CoversClient);
expand_media_download!(ScreenshotsClient);

expand_get_by_game_id!(ArtworksClient, Artwork);
expand_get_by_game_id!(CoversClient, Cover);
expand_get_by_game_id!(GameVideosClient, GameVideo);
expand_get_by_game_id!(MultiPlayerModeClient, MultiplayerMode);
expand_get_by_game_id!(ReleaseDateClient, ReleaseDate);
expand_get_by_game_id!(ScreenshotsClient, Screenshot);
expand_get_by_game_id!(WebsitesClient, Website);

pub struct IGDBClient {
    api_key: String,
}

// The IGDB client.
impl IGDBClient {
    /// Creates a new instance of the IGDB Client
    /// To create a new instance you need to provide the user-key your are provided with
    /// when you register in the API Website
    /// # Examples
    /// ```
    /// use igdb_client::client::IGDBClient;
    ///
    /// let igdb = IGDBClient::new("user-key");
    /// ```
    pub fn new<S: Into<String>>(api_key: S) -> IGDBClient {
        IGDBClient {
            api_key: api_key.into(),
        }
    }
    /// Creates a new instance of a Request builder
    /// you can use it's methods to create custom queries
    ///
    /// # Examples
    ///
    /// ```
    /// use igdb_client::client::IGDBClient;
    /// use igdb_client::request_builder::Equality;
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

#[allow(dead_code)]
///This function receives a path, an IGDB provided url and it normalices the path and downloads
/// the resource to the specified path file using the specified MediaQuality"
async fn download_resource(
    path: String,
    url: String,
    quality: MediaQuality,
) -> async_std::io::Result<()> {
    use async_std::fs::File;
    use async_std::io::Write;

    let mut parsed_url = match url {
        _ if !url.starts_with("http") => format!("{}{}", "http:", url),
        _ => url.to_owned(),
    };

    parsed_url = parsed_url.replace("thumb", &quality.get_value());

    println!("Downloading resource: {}", parsed_url);

    let content = surf::get(parsed_url).recv_bytes().await.unwrap();
    let mut file = File::create(path).await?;
    file.write(&content[..]).await.unwrap();

    Ok(())
}
