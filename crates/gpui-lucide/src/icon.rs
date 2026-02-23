//! Icon component for rendering SVG icons in GPUI.

use gpui::{
    AnyElement, App, Hsla, IntoElement, Radians, RenderOnce, SharedString, StyleRefinement, Styled,
    Svg, Transformation, Window, prelude::*, svg,
};

/// Trait for types that can provide an icon path.
///
/// Implement this trait to create custom icon sets that work with the `Icon` component.
///
/// # Example
///
/// ```rust,ignore
/// pub enum MyIcons {
///     Logo,
///     CustomIcon,
/// }
///
/// impl IconNamed for MyIcons {
///     fn path(&self) -> &'static str {
///         match self {
///             Self::Logo => "my-icons/logo.svg",
///             Self::CustomIcon => "my-icons/custom.svg",
///         }
///     }
/// }
/// ```
pub trait IconNamed {
    fn path(&self) -> &'static str;
}

// Implement for IconName (generated enum)
impl IconNamed for crate::IconName {
    fn path(&self) -> &'static str {
        crate::IconName::path(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconSize {
    /// Extra small (12px / 0.75rem)
    XSmall,
    /// Small (14px / 0.875rem)
    Small,
    /// Medium (16px / 1rem) - default
    Medium,
    /// Large (24px / 1.5rem)
    Large,
    /// Extra large (32px / 2rem)
    XLarge,
}

impl IconSize {
    fn to_rems(self) -> f32 {
        match self {
            IconSize::XSmall => 0.75,
            IconSize::Small => 0.875,
            IconSize::Medium => 1.0,
            IconSize::Large => 1.5,
            IconSize::XLarge => 2.0,
        }
    }
}

/// An SVG icon component.
///
/// Icons can be created from any type implementing `IconNamed`, including the built-in
/// `IconName` enum which contains all Lucide icons.
///
/// # Examples
///
/// ```rust,ignore
/// use gpui_lucide::{Icon, IconName, IconSize};
///
/// // Basic usage
/// let icon = Icon::new(IconName::Heart);
///
/// // With color
/// let icon = Icon::new(IconName::Star)
///     .color(gpui::rgb(0xffd700));
///
/// // With size
/// let icon = Icon::new(IconName::Search)
///     .with_size(IconSize::Large);
///
/// // With rotation
/// let icon = Icon::new(IconName::ChevronRight)
///     .rotate(std::f32::consts::FRAC_PI_2); // 90 degrees
/// ```
#[derive(IntoElement)]
pub struct Icon {
    base: Svg,
    path: SharedString,
    color: Option<Hsla>,
    size: Option<IconSize>,
    custom_style: StyleRefinement,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            base: svg().flex_none().size_4(),
            path: "".into(),
            color: None,
            size: None,
            custom_style: StyleRefinement::default(),
        }
    }
}

impl Clone for Icon {
    fn clone(&self) -> Self {
        Self {
            base: svg().flex_none().size_4(),
            path: self.path.clone(),
            color: self.color,
            size: self.size,
            custom_style: self.custom_style.clone(),
        }
    }
}

impl Icon {
    /// Creates a new icon from any type implementing `IconNamed`.
    pub fn new(icon: impl IconNamed) -> Self {
        Self::default().path(icon.path())
    }

    /// Creates a new icon from a custom path.
    ///
    /// Use this when you want to specify an SVG path directly.
    pub fn from_path(path: impl Into<SharedString>) -> Self {
        Self::default().path(path)
    }

    /// Sets the icon path.
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = path.into();
        self
    }

    /// Sets the icon color.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the icon size using predefined sizes.
    pub fn with_size(mut self, size: IconSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Rotates the icon by the given angle in radians.
    pub fn rotate(mut self, radians: impl Into<Radians>) -> Self {
        self.base = self
            .base
            .with_transformation(Transformation::rotate(radians));
        self
    }

    /// Applies a custom transformation to the icon.
    pub fn transform(mut self, transformation: Transformation) -> Self {
        self.base = self.base.with_transformation(transformation);
        self
    }
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.custom_style
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = self.color.unwrap_or_else(|| window.text_style().color);
        let text_size = window.text_style().font_size.to_pixels(window.rem_size());

        let has_custom_size =
            self.custom_style.size.width.is_some() || self.custom_style.size.height.is_some();

        let mut base = self.base;
        *base.style() = self.custom_style;

        base.flex_shrink_0()
            .text_color(text_color)
            .when(!has_custom_size && self.size.is_none(), |this| {
                this.size(text_size)
            })
            .when_some(self.size, |this, size| {
                let rems = size.to_rems();
                this.size(gpui::rems(rems))
            })
            .path(self.path)
    }
}

impl From<Icon> for AnyElement {
    fn from(icon: Icon) -> Self {
        icon.into_any_element()
    }
}

impl From<crate::IconName> for Icon {
    fn from(name: crate::IconName) -> Self {
        Icon::new(name)
    }
}
