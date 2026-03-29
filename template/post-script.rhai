let crate_type = variable::get("crate_type");

if crate_type == "bin" {
  file::rename(".github/workflows/bin-cd.yml", ".github/workflows/cd.yml");
  file::delete(".github/workflows/lib-cd.yml");
} else if crate_type == "lib" {
  file::rename(".github/workflows/lib-cd.yml", ".github/workflows/cd.yml");
  file::delete(".github/workflows/bin-cd.yml");
} else {
  abort("unknown crate_type");
}

system::command("just", ["sync-rdme-all", "--allow-dirty", "--allow-no-vcs"])

