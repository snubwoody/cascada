#!/usr/bin/env bash

cargo publish -p agape_core --dry-run
cargo publish -p agape_macros --dry-run
cargo publish -p agape_layout --dry-run
cargo publish -p agape --dry-run

echo "✅ Published all crate in workspace"