use std::path::PathBuf;

#[test]
fn test_open() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("new.sb3");

    assert!(path.exists());
    let proj = sbeditor::Project::from_sb3(path).unwrap();
    println!("{}", proj.title)
}
