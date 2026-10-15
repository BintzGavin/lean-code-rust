# Lean Code Rust

An independent Rust editor exploration with repository-owned Black Hole Architecture. The desired product is in [VISION.md](VISION.md); ownership and cadence in [.sys/black-hole/role-map.md](.sys/black-hole/role-map.md). Existing VS Code extension compatibility is a hard requirement, not yet implemented.

The initial work order implemented a small document editing core with eight passing integration tests and a runnable synthetic roundtrip example. This repository is not yet a desktop editor. The [independent behavior package](spec/README.md) contains 64 unobserved scenarios; it is not a parity oracle. See [historical BHA choices](docs/decisions/historical-bha.md), [validation evidence](docs/decisions/bootstrap-validation.md) and [reference limitation](docs/decisions/reference-checkpoint.md).

Run repository validation checks and tests:

```sh
bash tooling/validate_repo.sh
```

Public repository publication does not install a scheduler or establish a GUI, extension host or release artifact. Existing VS Code extension compatibility remains required and unimplemented.

No project license has been designated. The Rust crate remains `publish = false`; this bootstrap does not assign an open-source license to the repository or include third-party extension packages. See [publication review and provenance](docs/decisions/publication-review.md).
