//! Peripheral access API for EFM32LG microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.24.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32lg-pacs)
//!
//! This crate supports all EFM32LG devices; for the complete list please see:
//! [efm32lg](https://github.com/efm32-rs/efm32lg-pacs/pacs/efm32lg)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32lg230")]
pub mod efm32lg230;

#[cfg(feature = "efm32lg232")]
pub mod efm32lg232;

#[cfg(feature = "efm32lg280")]
pub mod efm32lg280;

#[cfg(feature = "efm32lg290")]
pub mod efm32lg290;

#[cfg(feature = "efm32lg295")]
pub mod efm32lg295;

#[cfg(feature = "efm32lg330")]
pub mod efm32lg330;

#[cfg(feature = "efm32lg332")]
pub mod efm32lg332;

#[cfg(feature = "efm32lg360")]
pub mod efm32lg360;

#[cfg(feature = "efm32lg380")]
pub mod efm32lg380;

#[cfg(feature = "efm32lg390")]
pub mod efm32lg390;

#[cfg(feature = "efm32lg395")]
pub mod efm32lg395;

#[cfg(feature = "efm32lg840")]
pub mod efm32lg840;

#[cfg(feature = "efm32lg842")]
pub mod efm32lg842;

#[cfg(feature = "efm32lg880")]
pub mod efm32lg880;

#[cfg(feature = "efm32lg890")]
pub mod efm32lg890;

#[cfg(feature = "efm32lg895")]
pub mod efm32lg895;

#[cfg(feature = "efm32lg900")]
pub mod efm32lg900;

#[cfg(feature = "efm32lg940")]
pub mod efm32lg940;

#[cfg(feature = "efm32lg942")]
pub mod efm32lg942;

#[cfg(feature = "efm32lg980")]
pub mod efm32lg980;

#[cfg(feature = "efm32lg990")]
pub mod efm32lg990;

#[cfg(feature = "efm32lg995")]
pub mod efm32lg995;
