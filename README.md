# EFM32LG (Leopard Gecko) support for Rust

[![PACs](https://github.com/efm32-rs/efm32lg-pacs/actions/workflows/pacs.yml/badge.svg)](https://github.com/efm32-rs/efm32lg-pacs/actions/workflows/pacs.yml)

This repository contains Peripheral Access Crates (PACs) for Silabs' EFM32 series of Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Refer to the [CHANGELOG](CHANGELOG.md) to see what changed in the last releases.

## Crates

Every EFM32LG chip has its own PAC, listed below:

| Crate            | Docs                                                                                   | crates.io                                                                                                   | Target               |
|------------------|----------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|----------------------|
| `efm32lg230-pac` | [![docs.rs](https://docs.rs/efm32lg230-pac/badge.svg)](https://docs.rs/efm32lg230-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg230-pac.svg)](https://crates.io/crates/efm32lg230-pac) | `thumbv7m-none-eabi` |
 | `efm32lg232-pac` | [![docs.rs](https://docs.rs/efm32lg232-pac/badge.svg)](https://docs.rs/efm32lg232-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg232-pac.svg)](https://crates.io/crates/efm32lg232-pac) | `thumbv7m-none-eabi` |
 | `efm32lg280-pac` | [![docs.rs](https://docs.rs/efm32lg280-pac/badge.svg)](https://docs.rs/efm32lg280-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg280-pac.svg)](https://crates.io/crates/efm32lg280-pac) | `thumbv7m-none-eabi` |
 | `efm32lg290-pac` | [![docs.rs](https://docs.rs/efm32lg290-pac/badge.svg)](https://docs.rs/efm32lg290-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg290-pac.svg)](https://crates.io/crates/efm32lg290-pac) | `thumbv7m-none-eabi` |
 | `efm32lg295-pac` | [![docs.rs](https://docs.rs/efm32lg295-pac/badge.svg)](https://docs.rs/efm32lg295-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg295-pac.svg)](https://crates.io/crates/efm32lg295-pac) | `thumbv7m-none-eabi` |
 | `efm32lg330-pac` | [![docs.rs](https://docs.rs/efm32lg330-pac/badge.svg)](https://docs.rs/efm32lg330-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg330-pac.svg)](https://crates.io/crates/efm32lg330-pac) | `thumbv7m-none-eabi` |
 | `efm32lg332-pac` | [![docs.rs](https://docs.rs/efm32lg332-pac/badge.svg)](https://docs.rs/efm32lg332-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg332-pac.svg)](https://crates.io/crates/efm32lg332-pac) | `thumbv7m-none-eabi` |
 | `efm32lg360-pac` | [![docs.rs](https://docs.rs/efm32lg360-pac/badge.svg)](https://docs.rs/efm32lg360-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg360-pac.svg)](https://crates.io/crates/efm32lg360-pac) | `thumbv7m-none-eabi` |
 | `efm32lg380-pac` | [![docs.rs](https://docs.rs/efm32lg380-pac/badge.svg)](https://docs.rs/efm32lg380-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg380-pac.svg)](https://crates.io/crates/efm32lg380-pac) | `thumbv7m-none-eabi` |
 | `efm32lg390-pac` | [![docs.rs](https://docs.rs/efm32lg390-pac/badge.svg)](https://docs.rs/efm32lg390-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg390-pac.svg)](https://crates.io/crates/efm32lg390-pac) | `thumbv7m-none-eabi` |
 | `efm32lg395-pac` | [![docs.rs](https://docs.rs/efm32lg395-pac/badge.svg)](https://docs.rs/efm32lg395-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg395-pac.svg)](https://crates.io/crates/efm32lg395-pac) | `thumbv7m-none-eabi` |
 | `efm32lg840-pac` | [![docs.rs](https://docs.rs/efm32lg840-pac/badge.svg)](https://docs.rs/efm32lg840-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg840-pac.svg)](https://crates.io/crates/efm32lg840-pac) | `thumbv7m-none-eabi` |
 | `efm32lg842-pac` | [![docs.rs](https://docs.rs/efm32lg842-pac/badge.svg)](https://docs.rs/efm32lg842-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg842-pac.svg)](https://crates.io/crates/efm32lg842-pac) | `thumbv7m-none-eabi` |
 | `efm32lg880-pac` | [![docs.rs](https://docs.rs/efm32lg880-pac/badge.svg)](https://docs.rs/efm32lg880-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg880-pac.svg)](https://crates.io/crates/efm32lg880-pac) | `thumbv7m-none-eabi` |
 | `efm32lg890-pac` | [![docs.rs](https://docs.rs/efm32lg890-pac/badge.svg)](https://docs.rs/efm32lg890-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg890-pac.svg)](https://crates.io/crates/efm32lg890-pac) | `thumbv7m-none-eabi` |
 | `efm32lg895-pac` | [![docs.rs](https://docs.rs/efm32lg895-pac/badge.svg)](https://docs.rs/efm32lg895-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg895-pac.svg)](https://crates.io/crates/efm32lg895-pac) | `thumbv7m-none-eabi` |
 | `efm32lg900-pac` | [![docs.rs](https://docs.rs/efm32lg900-pac/badge.svg)](https://docs.rs/efm32lg900-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg900-pac.svg)](https://crates.io/crates/efm32lg900-pac) | `thumbv7m-none-eabi` |
 | `efm32lg940-pac` | [![docs.rs](https://docs.rs/efm32lg940-pac/badge.svg)](https://docs.rs/efm32lg940-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg940-pac.svg)](https://crates.io/crates/efm32lg940-pac) | `thumbv7m-none-eabi` |
 | `efm32lg942-pac` | [![docs.rs](https://docs.rs/efm32lg942-pac/badge.svg)](https://docs.rs/efm32lg942-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg942-pac.svg)](https://crates.io/crates/efm32lg942-pac) | `thumbv7m-none-eabi` |
 | `efm32lg980-pac` | [![docs.rs](https://docs.rs/efm32lg980-pac/badge.svg)](https://docs.rs/efm32lg980-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg980-pac.svg)](https://crates.io/crates/efm32lg980-pac) | `thumbv7m-none-eabi` |
 | `efm32lg990-pac` | [![docs.rs](https://docs.rs/efm32lg990-pac/badge.svg)](https://docs.rs/efm32lg990-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg990-pac.svg)](https://crates.io/crates/efm32lg990-pac) | `thumbv7m-none-eabi` |
 | `efm32lg995-pac` | [![docs.rs](https://docs.rs/efm32lg995-pac/badge.svg)](https://docs.rs/efm32lg995-pac) | [![crates.io](https://img.shields.io/crates/d/efm32lg995-pac.svg)](https://crates.io/crates/efm32lg995-pac) | `thumbv7m-none-eabi` |

## Device Reference Manuals from Silabs

**WIP**

## License

The included SVD files are sourced from https://www.silabs.com/documents/public/cmsis-packs and
are licensed under the Zlib (see [LICENSE-3RD-PARTY](LICENSE-3RD-PARTY-Zlib)).

The remainder of the code is under:

- 3-Clause BSD license ([LICENSE-3BSD](LICENSE-3BSD) or https://opensource.org/licenses/BSD-3-Clause)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the BSD-3-Clause license without any additional terms or conditions.