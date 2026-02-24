//! Playground app for gpui-lucide icons
//!
//! A visual demo to browse, search, and customize Lucide icons.

use gpui::{
  App, AppContext, Application, AssetSource, Bounds, Context, Entity, FocusHandle, Focusable, Hsla,
  InteractiveElement, IntoElement, KeyBinding, MouseButton, Render, SharedString,
  StatefulInteractiveElement, Styled, Subscription, Window, WindowBounds, WindowOptions, actions,
  div, prelude::*, px, radians, rgb, uniform_list,
};
use gpui_lucide::{Icon, IconName, IconSize};
use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

mod search_input;
use search_input::SearchInput;

actions!(
  playground,
  [
    Quit,
    ClearSearch,
    Backspace,
    Delete,
    Left,
    Right,
    SelectLeft,
    SelectRight,
    SelectAll,
    Home,
    End,
    MoveWordLeft,
    MoveWordRight,
    SelectWordLeft,
    SelectWordRight,
    SelectToBeginningOfLine,
    SelectToEndOfLine,
    DeleteWordBackward,
    DeleteToBeginningOfLine,
    DeleteToEndOfLine,
    ShowCharacterPalette,
    Paste,
    Cut,
    Copy
  ]
);

struct Assets {
  base: PathBuf,
}

impl AssetSource for Assets {
  fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
    let full_path = self.base.join(path);
    match fs::read(&full_path) {
      Ok(data) => Ok(Some(Cow::Owned(data))),
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
    let full_path = self.base.join(path);
    match fs::read_dir(&full_path) {
      Ok(entries) => Ok(
        entries
          .filter_map(|e| e.ok())
          .filter_map(|e| e.file_name().into_string().ok())
          .map(SharedString::from)
          .collect(),
      ),
      Err(_) => Ok(vec![]),
    }
  }
}

mod theme {
  use gpui::{Hsla, rgb};

  pub fn bg(is_dark: bool) -> Hsla {
    if is_dark {
      rgb(0x000000).into()
    } else {
      rgb(0xffffff).into()
    }
  }

  pub fn bg_secondary(is_dark: bool) -> Hsla {
    if is_dark {
      rgb(0x0b0b0b).into()
    } else {
      rgb(0xf5f5f5).into()
    }
  }

  pub fn bg_hover(is_dark: bool) -> Hsla {
    if is_dark {
      rgb(0x171717).into()
    } else {
      rgb(0xe9e9e9).into()
    }
  }

  pub fn border(is_dark: bool) -> Hsla {
    if is_dark {
      rgb(0x202020).into()
    } else {
      rgb(0xd9d9d9).into()
    }
  }

  pub fn text(is_dark: bool) -> Hsla {
    if is_dark {
      rgb(0xe8e8e8).into()
    } else {
      rgb(0x111111).into()
    }
  }

  pub fn text_muted(is_dark: bool) -> Hsla {
    if is_dark {
      rgb(0x8b8b8b).into()
    } else {
      rgb(0x666666).into()
    }
  }

  pub fn accent(_is_dark: bool) -> Hsla {
    rgb(0xe94560).into()
  }
}

const COLOR_PRESETS: &[(u32, &str)] = &[
  (0xffffff, "White"),
  (0xe94560, "Red"),
  (0xff6b6b, "Coral"),
  (0xfeca57, "Yellow"),
  (0x48dbfb, "Cyan"),
  (0x1dd1a1, "Green"),
  (0x5f27cd, "Purple"),
  (0x54a0ff, "Blue"),
];

struct Playground {
  focus_handle: FocusHandle,
  search_input: Entity<SearchInput>,
  _search_subscription: Subscription,
  is_dark: bool,
  selected_color: u32,
  selected_size: IconSize,
  rotation_degrees: f32,
  filtered_icons: Vec<IconName>,
  hovered_icon: Option<IconName>,
}

fn filter_icons(query: &str) -> Vec<IconName> {
  let query = query.to_lowercase();
  if query.is_empty() {
    IconName::all().collect()
  } else {
    IconName::all()
      .filter(|icon| icon.name().contains(&query))
      .collect()
  }
}

impl Playground {
  fn icon_render_color(&self) -> gpui::Rgba {
    if self.is_dark {
      rgb(self.selected_color)
    } else {
      rgb(0x000000)
    }
  }

  fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
    let focus_handle = cx.focus_handle();
    let search_input = cx.new(SearchInput::new);
    let search_focus = search_input.read(cx).focus_handle(cx);
    window.focus(&search_focus, cx);

    let search_subscription = cx.observe(&search_input, |this, search_input, cx| {
      this.filtered_icons = filter_icons(search_input.read(cx).text());
      if let Some(hovered) = this.hovered_icon
        && !this.filtered_icons.contains(&hovered)
      {
        this.hovered_icon = None;
      }
      cx.notify();
    });

    let mut app = Self {
      focus_handle,
      search_input,
      _search_subscription: search_subscription,
      is_dark: true,
      selected_color: 0xffffff,
      selected_size: IconSize::Large,
      rotation_degrees: 0.0,
      filtered_icons: vec![],
      hovered_icon: None,
    };
    app.filtered_icons = filter_icons("");
    app
  }

  fn set_color(&mut self, color: u32, cx: &mut Context<Self>) {
    self.selected_color = color;
    cx.notify();
  }

  fn set_size(&mut self, size: IconSize, cx: &mut Context<Self>) {
    self.selected_size = size;
    cx.notify();
  }

  fn set_rotation(&mut self, degrees: f32, cx: &mut Context<Self>) {
    self.rotation_degrees = degrees;
    cx.notify();
  }

  fn set_hovered(&mut self, icon: Option<IconName>, cx: &mut Context<Self>) {
    self.hovered_icon = icon;
    cx.notify();
  }

  fn toggle_theme(&mut self, cx: &mut Context<Self>) {
    self.is_dark = !self.is_dark;
    cx.notify();
  }
}

impl Focusable for Playground {
  fn focus_handle(&self, _cx: &App) -> FocusHandle {
    self.focus_handle.clone()
  }
}

impl Render for Playground {
  fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let icon_color = self.icon_render_color();
    let rotation_rad = self.rotation_degrees.to_radians();
    let is_dark = self.is_dark;

    let viewport_width: f32 = window.viewport_size().width.into();
    let sidebar_width = 300.0;
    let grid_width = (viewport_width - sidebar_width).max(200.0);

    div()
      .id("playground")
      .key_context("Playground")
      .track_focus(&self.focus_handle)
      .size_full()
      .flex()
      .bg(theme::bg(is_dark))
      .text_color(theme::text(is_dark))
      .font_family("Inter, system-ui, sans-serif")
      .child(
        div()
          .size_full()
          .flex()
          .overflow_hidden()
          .child(self.render_sidebar(cx))
          .child(self.render_icon_grid(icon_color, rotation_rad, grid_width, cx)),
      )
  }
}

impl Playground {
  fn render_sidebar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    let is_dark = self.is_dark;

    div()
      .w(px(300.0))
      .h_full()
      .flex_shrink_0()
      .flex()
      .flex_col()
      .bg(theme::bg_secondary(is_dark))
      .border_r_1()
      .border_color(theme::border(is_dark))
      .p_4()
      .gap_6()
      .child(self.render_theme_toggle(cx))
      // Color picker
      .child(
        div()
          .flex()
          .flex_col()
          .gap_2()
          .child(
            div()
              .text_sm()
              .font_weight(gpui::FontWeight::MEDIUM)
              .child("Color"),
          )
          .child(self.render_color_picker(cx)),
      )
      // Size picker
      .child(
        div()
          .flex()
          .flex_col()
          .gap_2()
          .child(
            div()
              .text_sm()
              .font_weight(gpui::FontWeight::MEDIUM)
              .child("Size"),
          )
          .child(self.render_size_picker(cx)),
      )
      // Rotation
      .child(
        div()
          .flex()
          .flex_col()
          .gap_2()
          .child(
            div()
              .text_sm()
              .font_weight(gpui::FontWeight::MEDIUM)
              .child(format!("Rotation: {}°", self.rotation_degrees as i32)),
          )
          .child(self.render_rotation_picker(cx)),
      )
      // Preview
      .child(
        div()
          .flex_1()
          .flex()
          .flex_col()
          .gap_2()
          .child(
            div()
              .text_sm()
              .font_weight(gpui::FontWeight::MEDIUM)
              .child("Preview"),
          )
          .child(self.render_preview(cx)),
      )
  }

  fn render_theme_toggle(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    let is_dark = self.is_dark;

    div()
      .size_9()
      .flex()
      .rounded_full()
      .cursor_pointer()
      .items_center()
      .justify_center()
      .border_1()
      .border_color(theme::border(is_dark))
      .bg(theme::bg(is_dark))
      .hover(move |s| s.bg(theme::bg_hover(is_dark)))
      .on_mouse_up(
        MouseButton::Left,
        cx.listener(|this, _, _, cx| {
          this.toggle_theme(cx);
        }),
      )
      .child(
        Icon::new(if is_dark {
          IconName::Sun
        } else {
          IconName::Moon
        })
        .color(theme::text(is_dark))
        .with_size(IconSize::Medium),
      )
  }

  fn render_search_input(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    let is_dark = self.is_dark;

    div()
      .flex()
      .items_center()
      .gap_2()
      .px_3()
      .py_2()
      .rounded_md()
      .bg(theme::bg_secondary(is_dark))
      .border_1()
      .border_color(theme::border(is_dark))
      .on_mouse_down(
        MouseButton::Left,
        cx.listener(|this, _, window, cx| {
          let focus_handle = this.search_input.read(cx).focus_handle(cx);
          window.focus(&focus_handle, cx);
        }),
      )
      .child(
        Icon::new(IconName::Search)
          .color(theme::text_muted(is_dark))
          .with_size(IconSize::Small),
      )
      .child(div().flex_1().child(self.search_input.clone()))
  }

  fn render_color_picker(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    let selected = self.selected_color;
    let is_dark = self.is_dark;

    div()
      .flex()
      .flex_wrap()
      .gap_2()
      .children(COLOR_PRESETS.iter().map(|(color, name)| {
        let is_selected = *color == selected;
        let color_val = *color;

        div()
          .id(SharedString::from(*name))
          .size_8()
          .rounded_md()
          .cursor_pointer()
          .bg(rgb(color_val))
          .border_2()
          .border_color(if is_selected {
            theme::text(is_dark)
          } else {
            Hsla::transparent_black()
          })
          .hover(|s| s.opacity(0.8))
          .on_click(cx.listener(move |this, _, _, cx| {
            this.set_color(color_val, cx);
          }))
      }))
  }

  fn render_size_picker(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    let sizes = [
      (IconSize::XSmall, "XS"),
      (IconSize::Small, "S"),
      (IconSize::Medium, "M"),
      (IconSize::Large, "L"),
      (IconSize::XLarge, "XL"),
    ];
    let selected = self.selected_size;
    let is_dark = self.is_dark;

    div()
      .flex()
      .gap_2()
      .children(sizes.iter().map(|(size, label)| {
        let is_selected = *size == selected;
        let size_val = *size;

        div()
          .id(SharedString::from(*label))
          .px_3()
          .py_1()
          .rounded_md()
          .cursor_pointer()
          .bg(if is_selected {
            theme::accent(is_dark)
          } else {
            theme::bg(is_dark)
          })
          .text_sm()
          .hover(|s| {
            s.bg(if is_selected {
              theme::accent(is_dark)
            } else {
              theme::bg_hover(is_dark)
            })
          })
          .on_click(cx.listener(move |this, _, _, cx| {
            this.set_size(size_val, cx);
          }))
          .child(*label)
      }))
  }

  fn render_rotation_picker(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    let rotations = [0.0, 45.0, 90.0, 180.0, 270.0];
    let selected = self.rotation_degrees;
    let is_dark = self.is_dark;

    div().flex().gap_2().children(rotations.iter().map(|deg| {
      let is_selected = (*deg - selected).abs() < 0.1;
      let deg_val = *deg;

      div()
        .id(SharedString::from(format!("rot-{}", deg)))
        .px_3()
        .py_1()
        .rounded_md()
        .cursor_pointer()
        .bg(if is_selected {
          theme::accent(is_dark)
        } else {
          theme::bg(is_dark)
        })
        .text_sm()
        .hover(|s| {
          s.bg(if is_selected {
            theme::accent(is_dark)
          } else {
            theme::bg_hover(is_dark)
          })
        })
        .on_click(cx.listener(move |this, _, _, cx| {
          this.set_rotation(deg_val, cx);
        }))
        .child(format!("{}°", deg))
    }))
  }

  fn render_preview(&self, _cx: &mut Context<Self>) -> impl IntoElement {
    let icon = self.hovered_icon.unwrap_or(IconName::Heart);
    let color = self.icon_render_color();
    let rotation = radians(self.rotation_degrees.to_radians());
    let is_dark = self.is_dark;

    div()
      .flex_1()
      .flex()
      .flex_col()
      .items_center()
      .justify_center()
      .gap_4()
      .p_4()
      .rounded_lg()
      .bg(theme::bg(is_dark))
      .child(
        Icon::new(icon)
          .color(color)
          .with_size(IconSize::XLarge)
          .rotate(rotation),
      )
      .child(
        div()
          .text_sm()
          .text_color(theme::text_muted(is_dark))
          .child(icon.name()),
      )
  }

  fn render_icon_grid(
    &mut self,
    color: gpui::Rgba,
    rotation: f32,
    grid_width: f32,
    cx: &mut Context<Self>,
  ) -> impl IntoElement {
    let count = self.filtered_icons.len();
    let selected_size = self.selected_size;
    let color_hsla: Hsla = color.into();
    let is_dark = self.is_dark;

    const CARD_SIZE: f32 = 72.0;
    const GAP: f32 = 8.0;
    const PADDING: f32 = 16.0; // px_4 = 16px on each side

    let available_width = grid_width - (PADDING * 2.0);
    let items_per_row = ((available_width + GAP) / (CARD_SIZE + GAP)).floor() as usize;
    let items_per_row = items_per_row.max(1);

    const ROW_HEIGHT: f32 = CARD_SIZE + GAP;
    let num_rows = count.div_ceil(items_per_row);

    div()
      .flex_1()
      .flex()
      .flex_col()
      .overflow_hidden()
      .child(
        div()
          .w_full()
          .px_4()
          .py_3()
          .flex()
          .items_center()
          .gap_3()
          .border_b_1()
          .border_color(theme::border(is_dark))
          .child(div().flex_1().child(self.render_search_input(cx)))
          .child(
            div()
              .text_sm()
              .text_color(theme::text_muted(is_dark))
              .child(format!("{} icons", count)),
          ),
      )
      .child(
        uniform_list(
          "icon-grid",
          num_rows,
          cx.processor(
            move |this, visible_range: std::ops::Range<usize>, _window, cx| {
              visible_range
                .map(|row_idx| {
                  let start_idx = row_idx * items_per_row;
                  let end_idx = (start_idx + items_per_row).min(this.filtered_icons.len());

                  div()
                    .id(row_idx)
                    .h(px(ROW_HEIGHT))
                    .px_4()
                    .flex()
                    .gap_2()
                    .children((start_idx..end_idx).map(|idx| {
                      let icon: IconName = this.filtered_icons[idx];
                      let name = icon.name();
                      let truncated_name = if name.len() > 10 {
                        format!("{}...", &name[..8])
                      } else {
                        name.to_string()
                      };

                      div()
                        .id(SharedString::from(name))
                        .w(px(CARD_SIZE))
                        .h(px(CARD_SIZE))
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap_1()
                        .rounded_lg()
                        .cursor_pointer()
                        .bg(theme::bg_secondary(is_dark))
                        .hover(|s| s.bg(theme::bg_hover(is_dark)))
                        .on_hover(cx.listener(move |this, is_hovered, _, cx| {
                          if *is_hovered {
                            this.set_hovered(Some(icon), cx);
                          } else {
                            this.set_hovered(None, cx);
                          }
                        }))
                        .child(
                          Icon::new(icon)
                            .color(color_hsla)
                            .with_size(selected_size)
                            .rotate(radians(rotation)),
                        )
                        .child(
                          div()
                            .text_xs()
                            .text_color(theme::text_muted(is_dark))
                            .overflow_hidden()
                            .max_w_full()
                            .truncate()
                            .child(truncated_name),
                        )
                    }))
                })
                .collect()
            },
          ),
        )
        .flex_1()
        .pt_4(),
      )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_filter_icons_matches_query() {
    let icons = filter_icons("heart");
    assert!(icons.contains(&IconName::Heart));
  }

  #[test]
  fn test_filter_icons_is_case_insensitive() {
    let lower = filter_icons("heart");
    let upper = filter_icons("HEART");
    assert_eq!(lower, upper);
  }
}

fn main() {
  Application::with_platform(gpui_platform::current_platform(false))
    .with_assets(Assets {
      base: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf(),
    })
    .run(|cx: &mut App| {
      cx.bind_keys([
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("escape", ClearSearch, Some("SearchInput")),
        KeyBinding::new("backspace", Backspace, Some("SearchInput")),
        KeyBinding::new("alt-backspace", DeleteWordBackward, Some("SearchInput")),
        KeyBinding::new(
          "cmd-backspace",
          DeleteToBeginningOfLine,
          Some("SearchInput"),
        ),
        KeyBinding::new("delete", Delete, Some("SearchInput")),
        KeyBinding::new("cmd-delete", DeleteToEndOfLine, Some("SearchInput")),
        KeyBinding::new("left", Left, Some("SearchInput")),
        KeyBinding::new("right", Right, Some("SearchInput")),
        KeyBinding::new("alt-left", MoveWordLeft, Some("SearchInput")),
        KeyBinding::new("alt-right", MoveWordRight, Some("SearchInput")),
        KeyBinding::new("cmd-left", Home, Some("SearchInput")),
        KeyBinding::new("cmd-right", End, Some("SearchInput")),
        KeyBinding::new("shift-left", SelectLeft, Some("SearchInput")),
        KeyBinding::new("shift-right", SelectRight, Some("SearchInput")),
        KeyBinding::new("shift-alt-left", SelectWordLeft, Some("SearchInput")),
        KeyBinding::new("shift-alt-right", SelectWordRight, Some("SearchInput")),
        KeyBinding::new(
          "shift-cmd-left",
          SelectToBeginningOfLine,
          Some("SearchInput"),
        ),
        KeyBinding::new("shift-cmd-right", SelectToEndOfLine, Some("SearchInput")),
        KeyBinding::new("cmd-a", SelectAll, Some("SearchInput")),
        KeyBinding::new("cmd-v", Paste, Some("SearchInput")),
        KeyBinding::new("cmd-c", Copy, Some("SearchInput")),
        KeyBinding::new("cmd-x", Cut, Some("SearchInput")),
        KeyBinding::new("home", Home, Some("SearchInput")),
        KeyBinding::new("end", End, Some("SearchInput")),
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, Some("SearchInput")),
      ]);

      cx.on_action(|_: &Quit, cx| cx.quit());

      let bounds = Bounds::centered(None, gpui::size(px(1200.0), px(800.0)), cx);
      cx.open_window(
        WindowOptions {
          window_bounds: Some(WindowBounds::Windowed(bounds)),
          ..Default::default()
        },
        |window, cx| cx.new(|cx| Playground::new(window, cx)),
      )
      .unwrap();

      cx.activate(true);
    });
}
