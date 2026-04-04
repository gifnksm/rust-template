let crate_type = variable::get("crate_type");

if crate_type == "bin" {
  file::rename(".github/workflows/bin-ci.yml", ".github/workflows/ci.yml");
  file::delete(".github/workflows/lib-ci.yml");
  file::rename(".github/workflows/bin-cd.yml", ".github/workflows/cd.yml");
  file::delete(".github/workflows/lib-cd.yml");
  file::rename("justfile.bin", "justfile");
  file::delete("justfile.lib");
} else if crate_type == "lib" {
  file::rename(".github/workflows/lib-ci.yml", ".github/workflows/ci.yml");
  file::delete(".github/workflows/bin-ci.yml");
  file::rename(".github/workflows/lib-cd.yml", ".github/workflows/cd.yml");
  file::delete(".github/workflows/bin-cd.yml");
  file::rename("justfile.lib", "justfile");
  file::delete("justfile.bin");
} else {
  abort("unknown crate_type");
}

system::command("just", ["sync-rdme-all", "--allow-dirty", "--allow-no-vcs"])
