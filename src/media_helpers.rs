use crate::media_quality::MediaQuality;
use crate::Error;

#[allow(dead_code)]
///This function receives a path, an IGDB provided url and it normalices the path and downloads
/// the resource to the specified path file using the specified MediaQuality"
pub(crate) async fn download_resource(
    path: String,
    url: String,
    quality: MediaQuality,
) -> Result<(), Error> {
    use async_std::fs::File;
    use async_std::io::Write;

    let parsed_url = parse_url(url, quality);
    log::debug!("Downloading resource: {}", parsed_url);

    let content = surf::get(parsed_url).recv_bytes().await?;

    let mut file = File::create(path).await?;
    file.write(&content[..]).await?;

    Ok(())
}

///This function receives a path, an IGDB provided url and it normalices the path and
///returns the resource content in bytes
pub(crate) async fn get_resource<S: Into<String>>(
    url: S,
    quality: MediaQuality,
) -> Result<Vec<u8>, Error> {
    let parsed_url = parse_url(url, quality);
    let contents = surf::get(parsed_url).recv_bytes().await?;
    Ok(contents)
}

pub(crate) fn parse_url<S: Into<String>>(url: S, quality: MediaQuality) -> String {
    let parsed_url = match url.into() {
        ref u if !u.starts_with("http") => format!("{}{}", "http:", u),
        u => u,
    };

    parsed_url.replace("thumb", &quality.get_value())
}
