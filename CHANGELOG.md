# Changelog

All notable changes to agape will be documented in this file.

## 0.3.0 (unreleased)
- Added `debug_tree` method.
- Added max width constraint.
- Added max height constraint.
- Added min width constraint.
- Added min height constraint.
- Added `BlockLayout::from_boxed` constructor.
- Make `layout` module public.
- `VerticalLayout` no longer panics when aligning (center alignment) with no children.

## 0.2.0 - 27-10-2025

- All layout fields are now private ([#8](https://github.com/snubwoody/cascada/pull/8)).
- Added `GlobalId` ([#18](https://github.com/snubwoody/cascada/pull/18)).
- `BlockLayout::new` now takes in a `Layout` instead of a `Box<dyn Layout>` ([#17](https://github.com/snubwoody/cascada/pull/17));
