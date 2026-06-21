let crate_type = variable::get("crate_type");

if crate_type == "bin" {
    file::rename(".github/workflows/bin-ci.yml", ".github/workflows/ci.yml");
    file::delete(".github/workflows/lib-ci.yml");
    file::rename(".github/workflows/bin-cd.yml", ".github/workflows/cd.yml");
    file::delete(".github/workflows/lib-cd.yml");
    file::rename("Cargo.bin.toml", "Cargo.toml");
    file::delete("Cargo.lib.toml");
    file::rename("justfile.bin", "justfile");
    file::delete("justfile.lib");
    file::rename("release.bin.toml", "release.toml");
    file::delete("release.lib.toml");
} else if crate_type == "lib" {
    file::rename(".github/workflows/lib-ci.yml", ".github/workflows/ci.yml");
    file::delete(".github/workflows/bin-ci.yml");
    file::rename(".github/workflows/lib-cd.yml", ".github/workflows/cd.yml");
    file::delete(".github/workflows/bin-cd.yml");
    file::rename("Cargo.lib.toml", "Cargo.toml");
    file::delete("Cargo.bin.toml");
    file::rename("justfile.lib", "justfile");
    file::delete("justfile.bin");
    file::rename("release.lib.toml", "release.toml");
    file::delete("release.bin.toml");
} else {
    abort("unknown crate_type");
}

system::command("just", ["sync-rdme-all", "--allow-dirty", "--allow-no-vcs"])
