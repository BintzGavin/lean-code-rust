# Release Packaging

This directory contains documentation regarding the release packaging framework.

## Building a Release

To build the release artifacts, run the following command from the repository root:

```bash
bash packaging/build_release.sh
```

This script will:
1. Build the workspace in release mode (`cargo build --release --workspace`).
2. Copy the resulting binaries into the `dist/` directory at the repository root.

## Artifacts

The compiled release artifacts will be placed in the `dist/` folder. This folder is ignored by git to keep the repository clean.
