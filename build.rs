use std::{env, fs, path::PathBuf};

fn collect(spec: &str, dir: &str, entries: &mut Vec<String>) {
    let base = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join(dir);
    let mut files = fs::read_dir(&base).unwrap().flatten().collect::<Vec<_>>();
    files.sort_by_key(|entry| entry.file_name());
    for entry in files {
        let path = entry.path();
        if !matches!(path.extension().and_then(|e| e.to_str()), Some("yaml") | Some("yml")) {
            continue;
        }
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        entries.push(format!(
            "    (\"{spec}\", \"{name}\", include_str!({:?})),",
            path
        ));
    }
}

fn main() {
    println!("cargo:rerun-if-changed=base16");
    println!("cargo:rerun-if-changed=base24");
    println!("cargo:rerun-if-changed=tinted8");

    let mut entries = Vec::new();
    collect("base16", "base16", &mut entries);
    collect("base24", "base24", &mut entries);
    collect("tinted8", "tinted8", &mut entries);
    entries.sort();

    let text = format!(
        "/// (spec, name, yaml text), sorted by name.\n\
         pub const SCHEMES: &[(&str, &str, &str)] = &[\n{}\n];\n",
        entries.join("\n")
    );
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("schemes_table.rs");
    fs::write(out, text).unwrap();
}
