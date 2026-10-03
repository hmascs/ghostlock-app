//! Offline symbol lookup helper for profile authoring and verification.
//!
//! Prints the image-relative offset of each requested kernel symbol, recovered
//! from the boot image's embedded kallsyms (no root, no device).
//!
//! usage: cargo run --release --example lookup_symbols -- <boot.img> <sym> [<sym>...]

use std::path::Path;

use ghostlock_extract::boot::BootImage;
use ghostlock_extract::derive::relative_symbols;
use ghostlock_extract::kallsyms;
use ghostlock_extract::kallsyms_finder;

fn main() {
    let mut args = std::env::args().skip(1);
    let boot_path = args.next().unwrap_or_default();
    let names: Vec<String> = args.collect();
    if boot_path.is_empty() || names.is_empty() {
        eprintln!("usage: lookup_symbols <boot.img> <sym> [<sym>...]");
        std::process::exit(2);
    }

    let boot = BootImage::load(Path::new(&boot_path)).expect("load boot image");
    let btf_at = boot.embedded_btf_at();
    let pair = btf_at.as_ref().map(|(o, b)| (*o, b.len()));
    let ks = kallsyms_finder::recover(&boot.kernel, pair).expect("recover kallsyms");
    let base = kallsyms::unique(&ks.symbols, "_text")
        .or_else(|| kallsyms::unique(&ks.symbols, "_head"))
        .expect("_text/_head");
    let (rel, _) = relative_symbols(&ks.symbols, base);

    println!("# base(_text) = 0x{base:x}   symbols = {}", ks.symbols.len());
    for name in &names {
        match rel.get(name.as_str()) {
            Some(offs) if !offs.is_empty() => {
                let mut v: Vec<u64> = offs.iter().copied().collect();
                v.sort_unstable();
                for off in v {
                    println!("{name} = {off}  # 0x{off:x}");
                }
            }
            _ => println!("{name} = <not found>"),
        }
    }
}
