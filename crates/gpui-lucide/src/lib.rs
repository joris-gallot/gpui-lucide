//! # gpui-lucide
//!
//! Lucide icons for GPUI applications.
//!
//! This crate provides a complete set of [Lucide](https://lucide.dev) icons for use in GPUI
//! applications, with support for custom colors, sizes, and rotation.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use gpui::prelude::*;
//! use gpui_lucide::{Icon, IconName};
//!
//! // Use an icon directly
//! fn my_component() -> impl IntoElement {
//!     Icon::new(IconName::Heart)
//!         .size_6()
//!         .text_color(gpui::rgb(0xff0000))
//! }
//! ```
//!
//! ## Custom Icons
//!
//! You can also define your own icons by implementing the `IconNamed` trait:
//!
//! ```rust,ignore
//! use gpui_lucide::{IconNamed, Icon};
//!
//! pub enum MyCustomIcon {
//!     Logo,
//!     CustomSymbol,
//! }
//!
//! impl IconNamed for MyCustomIcon {
//!     fn path(&self) -> &'static str {
//!         match self {
//!             Self::Logo => "custom-icons/logo.svg",
//!             Self::CustomSymbol => "custom-icons/symbol.svg",
//!         }
//!     }
//! }
//!
//! // Use it the same way
//! let icon = Icon::new(MyCustomIcon::Logo);
//! ```

mod icon;

pub use icon::*;

// Include the generated icon names
include!(concat!(env!("OUT_DIR"), "/icons_generated.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::ffi::OsStr;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_icon_name_path() {
        let path = IconName::Heart.path();
        assert!(path.starts_with("icons/"));
        assert!(path.ends_with(".svg"));
    }

    #[test]
    fn test_icon_name_display() {
        let name = IconName::Heart;
        assert_eq!(name.to_string(), "heart");
    }

    #[test]
    fn test_icon_count() {
        assert!(IconName::count() > 1000);
    }

    #[test]
    fn test_all_icons_iterator() {
        let count = IconName::all().count();
        assert_eq!(count, IconName::count());
    }

    #[test]
    fn test_icon_count_matches_icons_directory() {
        let icons_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../icons");
        let svg_count = fs::read_dir(&icons_dir)
            .expect("icons directory should be readable")
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension() == Some(OsStr::new("svg")))
            .count();

        assert_eq!(IconName::count(), svg_count);
    }

    #[test]
    fn test_all_icon_name_path_mappings() {
        for icon in IconName::all() {
            let expected = format!("icons/{}.svg", icon.name());
            assert_eq!(icon.path(), expected.as_str());
        }
    }

    #[test]
    fn test_names_and_paths_are_unique() {
        let mut names = HashSet::new();
        let mut paths = HashSet::new();

        for icon in IconName::all() {
            assert!(
                names.insert(icon.name()),
                "duplicate icon name: {}",
                icon.name()
            );
            assert!(
                paths.insert(icon.path()),
                "duplicate icon path: {}",
                icon.path()
            );
        }
    }
}
