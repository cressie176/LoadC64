use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaType {
    BoxFront2D,
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MediaSet {
    box_front_2d: Option<Media>,
    screenshot_loading: Option<Media>,
    screenshot_title: Option<Media>,
    screenshot_gameplay: Option<Media>,
}

impl MediaSet {
    pub const fn new(box_front_2d: Option<Media>, screenshot_loading: Option<Media>, screenshot_title: Option<Media>, screenshot_gameplay: Option<Media>) -> Self {
        Self {
            box_front_2d,
            screenshot_loading,
            screenshot_title,
            screenshot_gameplay,
        }
    }

    pub const fn box_front_2d(&self) -> Option<&Media> {
        self.box_front_2d.as_ref()
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
