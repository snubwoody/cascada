# Changelog

All notable changes to `cascada` will be documented in this file.

## 0.3.0 - 04-12-2025

### BREAKING CHANGES!

- The inner type of `BoxSizing::Flex` has changed to `u32` from `u8` ([#32](https://github.com/snubwoody/cascada/pull/32)).

### New features

- Added `debug_tree` method to `Layout` trait ([#25](https://github.com/snubwoody/cascada/pull/25)).
- Added support for setting custom layout constraints.
  - Added max width constraint ([#29](https://github.com/snubwoody/cascada/pull/29)).
  - Added max height constraint ([#43](https://github.com/snubwoody/cascada/pull/43)).
  - Added min width constraint ([#31](https://github.com/snubwoody/cascada/pull/31)).
  - Added min height constraint ([#41](https://github.com/snubwoody/cascada/pull/41)).
- Added `IntoIterator` constructors to `HorizontalLayout` and `VerticalLayout` ([#30](https://github.com/snubwoody/cascada/pull/30)).
- Added `BlockLayout::from_boxed` constructor.

### Bug fixes

- `VerticalLayout` no longer panics when aligning (center alignment) with no children ([#44](https://github.com/snubwoody/cascada/pull/44)).

## 0.2.0 - 27-10-2025

- All layout fields are now private ([#8](https://github.com/snubwoody/cascada/pull/8)).
- Added `GlobalId` ([#18](https://github.com/snubwoody/cascada/pull/18)).
- `BlockLayout::new` now takes in a `Layout` instead of a `Box<dyn Layout>` ([#17](https://github.com/snubwoody/cascada/pull/17));
