# 0.1.0

- Initial Rust crates generation from EFM32G SVD pack v4.1.1

# 0.1.1

- Combined same family MCUs into the single crate

# 0.1.2

- Regenerated PAC crates with `svd2rust` version `0.28.0` with the enabled `atomics` feature
- Added `portable-atomic` dependency to reflect crates with the `atomics` feature generated
- Added `critical-section` dependency to reflect the latest `svd2rust` generation rules
