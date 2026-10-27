//! The crate's integration tests: one executable, one module per topic (CONVENTIONS.md, Tests).

mod companion;
mod shapes;

/// A file in tests/it that main.rs does not declare would never run.
#[test]
fn every_module_is_declared() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/it");
    let main = include_str!("main.rs");
    for entry in std::fs::read_dir(&dir).expect("tests/it is readable") {
        let name = entry.expect("a directory entry").file_name();
        let name = name.to_string_lossy();
        let Some(module) = name.strip_suffix(".rs") else {
            continue;
        };
        if module != "main" {
            assert!(
                main.contains(&format!("mod {module};")),
                "tests/it/{name} is not declared in main.rs"
            );
        }
    }
}
