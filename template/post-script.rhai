let file_map = [
    #{ target: ".github/workflows/ci.yml", bin: ".github/workflows/ci.bin.yml", lib: ".github/workflows/ci.lib.yml" },
    #{ target: ".github/workflows/cd.yml", bin: ".github/workflows/cd.bin.yml", lib: ".github/workflows/cd.lib.yml" },
    #{ target: "Cargo.toml", bin: "Cargo.bin.toml", lib: "Cargo.lib.toml" },
    #{ target: "mise.toml", bin: "mise.bin.toml", lib: "mise.lib.toml" },
    #{ target: ".mise/tasks/ci.toml", bin: ".mise/tasks/ci.bin.toml", lib: ".mise/tasks/ci.lib.toml" },
    #{ target: ".mise/tasks/cd.toml", bin: ".mise/tasks/cd.bin.toml", lib: () },
    #{ target: ".mise/tasks/rust.toml", bin: ".mise/tasks/rust.bin.toml", lib: ".mise/tasks/rust.lib.toml" },
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

system::command("mise", ["run", "sync:markdown", "--allow-no-vcs"]);
