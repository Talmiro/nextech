//! Windows executable metadata (icon + VersionInfo) via `winres`.
//!
//! No-op on non-Windows hosts: `cargo build` on Linux/macOS skips the
//! resource compiler entirely. On Windows (`x86_64-pc-windows-msvc/gnu`)
//! attaches `assets/icon.ico` (when present) plus publisher/version
//! metadata consumed by Explorer and Inno Setup.

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    // 0x0416 = Português (Brasil).
    res.set_language(0x0416);
    res.set("FileDescription", "NexTech PDV — Controle Financeiro");
    res.set("ProductName", "NexTech PDV");
    res.set("CompanyName", "NX NEXTECH");
    res.set("LegalCopyright", "Copyright (C) 2026 NX NEXTECH");
    res.set("OriginalFilename", "nextech.exe");
    res.set("FileVersion", env!("CARGO_PKG_VERSION"));
    res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
    // Ícone opcional: o build NÃO falha sem ele (basta adicionar
    // `assets/icon.ico` multi-tamanho 16/32/48/256 antes do release).
    if std::path::Path::new("assets/icon.ico").exists() {
        res.set_icon("assets/icon.ico");
    }
    if let Err(e) = res.compile() {
        eprintln!("winres failed: {e}");
        std::process::exit(1);
    }
}

#[cfg(not(windows))]
fn main() {}
