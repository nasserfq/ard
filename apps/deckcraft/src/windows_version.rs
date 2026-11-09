//! Shared by `apps/deckcraft/build.rs` and `apps/deckcraft-cli/build.rs`: the Windows file details
//! (Explorer > Properties > Details) for عرض's executables. They name عرض and its maker, credit
//! DeckCraft, and carry the release version (`ARD_VERSION`, set by the release workflow), not
//! the DeckCraft crate version.

/// Product name shown in the Details tab, the taskbar and Task Manager.
pub const PRODUCT_NAME: &str = "عرض (Ard)";
pub const COMPANY_NAME: &str = "Nasser Alqahtani";
pub const COPYRIGHT: &str = "Copyright (c) Nasser Alqahtani. Based on DeckCraft, copyright (c) the DeckCraft authors. MIT OR Apache-2.0.";

/// The version the executables report: `ARD_VERSION` when set, else the crate version.
pub fn version(package: &str) -> String {
    std::env::var("ARD_VERSION").ok().filter(|v| !v.trim().is_empty()).unwrap_or_else(|| package.to_string())
}

/// `MAJOR << 48 | MINOR << 32 | PATCH << 16`, the numeric form of `version`.
pub fn version_number(version: &str) -> u64 {
    let mut parts = version.split(['.', '-', '+']).map(|p| p.parse::<u16>().unwrap_or(0));
    let mut number = 0u64;
    for shift in [48, 32, 16] {
        number |= u64::from(parts.next().unwrap_or(0)) << shift;
    }
    number
}
