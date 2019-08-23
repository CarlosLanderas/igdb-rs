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

impl MediaQuality {
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
