use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

pub struct Assets;

macro_rules! bundled {
    ($($path:literal),* $(,)?) => {
        &[$(($path, include_bytes!(concat!("../assets/", $path)).as_slice())),*]
    };
}

const FILES: &[(&str, &[u8])] = bundled![
    "icons/close_small.svg",
    "icons/eye.svg",
    "icons/eye_off.svg",
    "icons/lock.svg",
    "icons/more_horiz.svg",
    "icons/play_arrow.svg",
    "icons/plus.svg",
    "icons/unlock.svg",
    "images/hue_ring.png",
    "images/sprite.png",
];

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(FILES
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(FILES
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}
