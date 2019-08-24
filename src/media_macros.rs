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
            ) -> async_std::io::Result<()> {
                let mut request = RequestBuilder::new();
                request
                    .all_fields()
                    .add_where("id", Equality::Equal, id.to_string());

                //TODO -> Implement std::ops::Try trait in macro clients
                let covers_response = self.get(request).await.unwrap();
                let cover = covers_response.first().unwrap();

                download_resource(path.into(), cover.url.clone(), media_quality).await
            }
        }
    };
}
