# Changelog

All notable changes to agape will be documented in this file.

## 0.3.0 - 2025-09-25

### Features

- Added support for emojis.
- Added support for images ([#102](https://github.com/snubwoody/agape-rs/pull/102)).
    - New `Image` widget.
- Added support for svgs ([#104](https://github.com/snubwoody/agape-rs/pull/104)).
    - New `Svg` widget.
- Added corner radius ([#126](https://github.com/snubwoody/agape-rs/pull/126)).
- Added asset manager ([#157](https://github.com/snubwoody/agape-rs/pull/157)).
- Added support text input ([#170](https://github.com/snubwoody/agape-rs/pull/170)).
    - New `TextField`: Receive text input.
- `VStack` widget now have scrolling ([#172](https://github.com/snubwoody/agape-rs/pull/172)).
- Added `#[derive(Widget)]` macro ([#161](https://github.com/snubwoody/agape-rs/pull/161)).
- Added `Container` widget.

### Changed

- Removed the `LayoutSolver` struct and renamed its single method to
  `solve_layout` ([#98](https://github.com/snubwoody/agape-rs/pull/98)).
- Switch to cosmic text for rendering ([#113](https://github.com/snubwoody/agape-rs/pull/113))

### Bug fixes

- Fixed incorrect text size being reported by renderer.

### Removed

- Removed views, rendering is now done through the `Renderer`.
- Removed `hstack`, `vstack`, `input` examples.

## 0.2.0 - 2025-07-17

### Features

- Added `Resources` struct to share global resources
- Repeat syntax, `hstack![widget;10]`, to `hstack!` and `vstack!` macros
- Added `on_click` and `on_hover` gestures
- Added `TextField` widget and text input
- Added borders
- `BoxStyle`, which contains common styling for all widgets
- Added event systems, i.e. systems that run when specific events are emitted

### Changed

- Systems now have a `&mut Resources` instead of the previous `&mut Context`
- Most of the functionality, like layout and state, is now handled in systems

### Removed

- Removed `Context` object, use `Resources` instead
- (core) Removed deprecated `colors` module

### Performance

- Use global font variable, instead of creating one each frame ([#77])(https://github.com/snubwoody/agape-rs/pull/77)
