# Reference checkpoint

Approved interaction reference: installed branded Visual Studio Code 1.133.0, commit a5b500951314efd502d07465bd138dfbd714a960, macOS 26.3 arm64. This is not the Code - OSS performance baseline. Prepared separate synthetic workspaces, user-data and empty extension profiles remain outside this repository.

The prior attempt met a locked screen. On the next supported UI inventory the Mac was unlocked. Selecting Terminal to launch the reference with isolated profile arguments was denied by the computer-use tool: “Computer Use is not allowed to use the app 'com.apple.Terminal' for safety reasons.” No launch occurred. No alternate terminal was used to bypass the denial, no existing app windows or profiles were altered, and no outcomes were observed.

The imported spec remains 0/64 directly observed. Initial Rust tests validate only the explicitly requested new slice. Startup, process-tree memory, network, desktop launch, VSIX behavior and distributable artifacts remain unmeasured/unverified. A supported isolated GUI-launch path is required to resume the reference interaction cases.
