use std::fs;
use std::path::{Path, PathBuf};

// TODO: possibly pybevy could have this copying if some feature flag is used?
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .expect("failed to run cargo metadata");

    let pybevy_pkg = metadata
        .packages
        .iter()
        .find(|p| p.name == "pybevy")
        .expect("pybevy not found in dependencies — is it in [dependencies]?");

    let pybevy_root = pybevy_pkg
        .manifest_path
        .parent()
        .expect("pybevy manifest has no parent dir");

    let pybevy_python = pybevy_root.join("pybevy");

    if !pybevy_python.exists() {
        panic!(
            "Could not find pybevy's Python source at {}. \
             If pybevy's layout has changed, check where __init__.py lives.",
            pybevy_python
        );
    }

    let dest = out_dir.join("pybevy");
    if dest.exists() {
        fs::remove_dir_all(&dest).ok();
    }

    copy_dir_all(pybevy_python.as_std_path(), &dest)
        .expect("failed to copy pybevy Python sources to OUT_DIR");

    // TODO: this is temporary - patch pybevy absolute imports to relative imports
    let init_py = dest.join("__init__.py");
    let original = fs::read_to_string(&init_py).expect("failed to read pybevy __init__.py");
    let patched = format!(
        "import sys as _sys; _sys.modules.setdefault(\"pybevy\", _sys.modules[__name__])  # whl-build alias\n\
         {original}"
    );
    fs::write(&init_py, patched).expect("failed to write patched __init__.py");

    println!(
        "cargo:warning=pybevy-whl-build: copied Python sources from {} to {}",
        pybevy_python,
        dest.display()
    );
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), &dest_path)?;
        }
    }
    Ok(())
}
