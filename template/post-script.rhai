let crate_type = variable::get("crate_type");

if crate_type == "bin" {
    file::rename(".github/workflows/bin-ci.yml", ".github/workflows/ci.yml");
    file::delete(".github/workflows/lib-ci.yml");
    file::rename(".github/workflows/bin-cd.yml", ".github/workflows/cd.yml");
    file::delete(".github/workflows/lib-cd.yml");
    file::rename("Cargo.bin.toml", "Cargo.toml");
    file::delete("Cargo.lib.toml");
    file::rename("mise.bin.toml", "mise.toml");
    file::delete("mise.lib.toml");
    file::rename(".mise/tasks/ci.bin.toml", ".mise/tasks/ci.toml");
    file::delete(".mise/tasks/ci.lib.toml");
    file::rename(".mise/tasks/cd.bin.toml", ".mise/tasks/cd.toml");
    file::rename(".mise/tasks/rust.bin.toml", ".mise/tasks/rust.toml");
    file::delete(".mise/tasks/rust.lib.toml");
} else if crate_type == "lib" {
    file::rename(".github/workflows/lib-ci.yml", ".github/workflows/ci.yml");
    file::delete(".github/workflows/bin-ci.yml");
    file::rename(".github/workflows/lib-cd.yml", ".github/workflows/cd.yml");
    file::delete(".github/workflows/bin-cd.yml");
    file::rename("Cargo.lib.toml", "Cargo.toml");
    file::delete("Cargo.bin.toml");
    file::rename("mise.lib.toml", "mise.toml");
    file::delete("mise.bin.toml");
    file::rename(".mise/tasks/ci.lib.toml", ".mise/tasks/ci.toml");
    file::delete(".mise/tasks/ci.bin.toml");
    file::delete(".mise/tasks/cd.bin.toml");
    file::rename(".mise/tasks/rust.lib.toml", ".mise/tasks/rust.toml");
    file::delete(".mise/tasks/rust.bin.toml");
} else {
    abort("unknown crate_type");
}

system::command("mise", ["run", "sync:markdown", "--allow-no-vcs"]);
