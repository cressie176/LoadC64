use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaType {
    BoxFront2D,
    BoxFront2DThumbnail,
    ScreenshotLoading,
    ScreenshotTitle,
    ScreenshotGameplay,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Media {
    media_type: MediaType,
    path: PathBuf,
}

impl Media {
    pub const fn new(media_type: MediaType, path: PathBuf) -> Self {
        Self { media_type, path }
    }

    pub const fn media_type(&self) -> &MediaType {
        &self.media_type
    }

    pub const fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaSet {
    box_front_2d: Option<Media>,
    box_front_2d_thumbnail: Media,
    screenshot_loading: Option<Media>,
    screenshot_title: Option<Media>,
    screenshot_gameplay: Option<Media>,
}

impl MediaSet {
    pub const fn new(
        box_front_2d: Option<Media>,
        box_front_2d_thumbnail: Media,
        screenshot_loading: Option<Media>,
        screenshot_title: Option<Media>,
        screenshot_gameplay: Option<Media>,
    ) -> Self {
        Self { box_front_2d, box_front_2d_thumbnail, screenshot_loading, screenshot_title, screenshot_gameplay }
    }

    pub const fn box_front_2d(&self) -> Option<&Media> {
        self.box_front_2d.as_ref()
    }

    pub const fn box_front_2d_thumbnail(&self) -> &Media {
        &self.box_front_2d_thumbnail
    }

    pub const fn screenshot_loading(&self) -> Option<&Media> {
        self.screenshot_loading.as_ref()
    }

    pub const fn screenshot_title(&self) -> Option<&Media> {
        self.screenshot_title.as_ref()
    }

    pub const fn screenshot_gameplay(&self) -> Option<&Media> {
        self.screenshot_gameplay.as_ref()
    }
}

#[cfg(test)]
impl Default for MediaSet {
    fn default() -> Self {
        Self {
            box_front_2d: None,
            box_front_2d_thumbnail: Media::new(MediaType::BoxFront2DThumbnail, PathBuf::from("test-default.png")),
            screenshot_loading: None,
            screenshot_title: None,
            screenshot_gameplay: None,
        }
    }
}
