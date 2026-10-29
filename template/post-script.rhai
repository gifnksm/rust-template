let file_map = [
    #{ target: ".github/workflows/ci.yml", bin: ".github/workflows/ci.bin.yml", lib: ".github/workflows/ci.lib.yml" },
    #{ target: ".github/workflows/reusable-build-dist.yml", bin: ".github/workflows/reusable-build-dist.bin.yml", lib: () },
    #{ target: ".github/workflows/release.yml", bin: ".github/workflows/release.bin.yml", lib: ".github/workflows/release.lib.yml" },
    #{ target: "Cargo.toml", bin: "Cargo.bin.toml", lib: "Cargo.lib.toml" },
    #{ target: "mise.toml", bin: "mise.bin.toml", lib: "mise.lib.toml" },
    #{ target: ".mise/tasks/dist.toml", bin: ".mise/tasks/dist.bin.toml", lib: () },
];

let crate_type = variable::get("crate_type");
let delete_type = "";
if crate_type == "bin" {
    delete_type = "lib";
} else if crate_type == "lib" {
    delete_type = "bin";
} else {
    abort("unknown crate_type");
}

for file in file_map {
    let rename = file[crate_type];
    let delete = file[delete_type];
    if rename != () {
        file::rename(rename, file.target);
    }
    if delete != () {
        file::delete(delete);
    }
}

system::command("cargo", ["generate-lockfile"]);
system::command("gh", ["actions-lock"]);
system::command("mise", ["run", "format:markdown"]);
system::command("mise", ["run", "sync:markdown", "--allow-no-vcs"]);
