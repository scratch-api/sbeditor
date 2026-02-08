use std::path::PathBuf;

#[test]
fn test_bima() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("NEW BEST! _ Part 1 for BIMA.sb3");

    assert!(path.exists());
    let mut proj = sbeditor::Project::from_sb3(path).unwrap();
    let sprite = proj.get_sprite_by_name("Part 1").unwrap();
    println!(
        "{:#?}",
        sprite
            .blocks
            .values()
            .filter_map(|b| b.mutation.to_owned())
            .collect::<Vec<_>>()
    );
}
