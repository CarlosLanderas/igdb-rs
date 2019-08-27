macro_rules! expand_media_download {
    ($i: ident) => {
        impl $i {
            ///Download to the given path the resource specified in the Id
            /// MediaQuality is an enum that specifies different image sizes
            /// # Examples
            /// ```no_run
            /// use igdb_rs::client::IGDBClient;
            /// use igdb_rs::media_quality::MediaQuality;
            /// let client = IGDBClient::new("key");
            /// let screenshot_client = client.screenshots();
            /// screenshot_client.download_by_id(12400, "screen.jpg".to_string(), MediaQuality::ScreenshotHuge,);
            /// ```
            ///
            pub async fn download_by_id<S: Into<String>>(
                &self,
                id: usize,
                path: S,
                media_quality: MediaQuality,
            ) -> Result<(), Error> {
                use crate::media_helpers::download_resource;

                let media = self.get_first_by_id(id).await?;
                download_resource(path.into(), media.url.clone(), media_quality).await
            }
            ///Retrieves content in bytes for the given media resource
            /// MediaQuality is an enum that specifies different image sizes
            /// # Examples
            /// ```no_run
            /// use igdb_rs::client::IGDBClient;
            /// use igdb_rs::media_quality::MediaQuality;
            /// let client = IGDBClient::new("key");
            /// let screenshot_client = client.screenshots();
            /// screenshot_client.get_resource_by_id(12400, MediaQuality::ScreenshotHuge,);
            /// ```
            ///
            pub async fn get_resource_by_id(
                &self,
                id: usize,
                media_quality: MediaQuality,
            ) -> Result<Vec<u8>, Error> {
                use crate::media_helpers::get_resource;

                let media = self.get_first_by_id(id).await?;
                get_resource(media.url.clone(), media_quality).await
            }
        }
    };
}
