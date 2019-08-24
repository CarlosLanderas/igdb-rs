pub enum MediaQuality {
    CoverSmall,
    ScreenshotMedium,
    CoverBig,
    ScreenshotBig,
    ScreenshotHuge,
    Thumb,
    Micro,
    HD,
    FullHD,
}
///Represents the quality of the resource
impl MediaQuality {
    /// Returns the MediaQuality String representation to download resources from internet
    /// # Examples
    /// ```
    /// use igdb_rs::media_quality::MediaQuality;
    /// assert_eq!(MediaQuality::ScreenshotHuge.get_value(), "screenshot_huge");
    /// assert_eq!(MediaQuality::FullHD.get_value(), "1080p");
    /// ```
    pub fn get_value(&self) -> &str {
        match self {
            MediaQuality::CoverSmall => "cover_small",
            MediaQuality::ScreenshotMedium => "screenshot_med",
            MediaQuality::CoverBig => "cover_big",
            MediaQuality::ScreenshotBig => "screenshot_big",
            MediaQuality::ScreenshotHuge => "screenshot_huge",
            MediaQuality::Thumb => "thumb",
            MediaQuality::Micro => "micro",
            MediaQuality::HD => "720p",
            MediaQuality::FullHD => "1080p",
        }
    }
}
