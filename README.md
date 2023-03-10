# MobileCoin: Serialization Wrappers

[![Project Chat][chat-image]][chat-link]<!--
-->![License][license-image]<!--
-->![Architecture: any][arch-image]<!--
-->[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->[![CodeCov Status][codecov-image]][codecov-link]<!--
-->[![GitHub Workflow Status][gha-image]][gha-link]<!--
-->[![Contributor Covenant][conduct-image]][conduct-link]

Basic wrappers for serialization.

The goal of this crate is to provide a single common interface to whatever
third-party serialization library we choose, so that we can easily change it
later. Please call into this crate rather than talking to bincode etc. directly, for
data that is being passed to / from the enclave.

[chat-image]: https://img.shields.io/discord/844353360348971068?style=flat-square
[chat-link]: https://discord.gg/mobilecoin
[license-image]: https://img.shields.io/crates/l/mc-serial?style=flat-square
[arch-image]: https://img.shields.io/badge/arch-any-brightgreen?style=flat-square
[crate-image]: https://img.shields.io/crates/v/mc-serial.svg?style=flat-square
[crate-link]: https://crates.io/crates/mc-serial
[docs-image]: https://img.shields.io/docsrs/mc-serial?style=flat-square
[docs-link]: https://docs.rs/crate/mc-serial
[deps-image]: https://deps.rs/repo/github/mobilecoinfoundation/serial/status.svg?style=flat-square
[deps-link]: https://deps.rs/repo/github/mobilecoinfoundation/serial
[codecov-image]: https://img.shields.io/codecov/c/github/mobilecoinfoundation/serial/develop?style=flat-square
[codecov-link]: https://codecov.io/gh/mobilecoinfoundation/serial
[gha-image]: https://img.shields.io/github/actions/workflow/status/mobilecoinfoundation/serial/ci.yaml?branch=main&style=flat-square
[gha-link]: https://github.com/mobilecoinfoundation/serial/actions/workflows/ci.yaml?query=branch%3Amain
[conduct-link]: CODE_OF_CONDUCT.md
[conduct-image]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=flat-square
