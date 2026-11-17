//! Visualab Studios links (shown in Help ▸, the About dialog, the start screen and the header's
//! Visualab button). The app's name lives here, in one place.

pub const APP_NAME: &str = "VL Effects";
pub const APP_SLUG: &str = "vl-effects";
pub const DISCORD: &str = "https://www.visualabstudios.com";
pub const WEBSITE: &str = "https://www.visualabstudios.com";
pub const APP_PAGE: &str = "https://www.visualabstudios.com/vl-tools/vl-effects";
/// Where Help ▸ Provide Feedback goes.
pub const CONTACT: &str = "https://www.visualabstudios.com/contacto";

/// The other VL Tools by Visualab Studios: (name, slug of its page on visualabstudios.com).
pub const SIBLINGS: &[(&str, &str)] = &[("VL Studios", "vl-studios"), ("VL Vector", "vl-vector"), ("VL Films", "vl-films")];

pub fn page_of(slug: &str) -> String {
    format!("https://www.visualabstudios.com/vl-tools/{slug}")
}
