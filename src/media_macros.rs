macro_rules! expand_media_download {
    ($i: ident) => {
        impl $i {
            pub async fn download_by_id<S: Into<String>>(
                &self,
                id: String,
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
