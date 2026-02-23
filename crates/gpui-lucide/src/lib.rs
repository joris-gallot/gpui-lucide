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

    #[test]
    fn test_icon_name_path() {
        // Test that paths are generated correctly
        let path = IconName::Heart.path();
        assert!(path.starts_with("icons/"));
        assert!(path.ends_with(".svg"));
    }

    #[test]
    fn test_icon_name_display() {
        let name = IconName::Heart;
        assert_eq!(name.name(), "heart");
    }

    #[test]
    fn test_icon_count() {
        // We should have many icons
        assert!(IconName::count() > 1000);
    }

    #[test]
    fn test_all_icons_iterator() {
        let count = IconName::all().count();
        assert_eq!(count, IconName::count());
    }
}
