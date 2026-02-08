use std::path::PathBuf;

#[test]
fn test_open() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("new.sb3");

    assert!(path.exists());
    let mut proj = sbeditor::Project::from_sb3(path).unwrap();
    let sprite = proj.get_sprite_by_name("Part 1").unwrap();
    println!(
        "{:#?}",
        sprite
            .blocks
            .values()
            .skip(15)
            .take(15)
            .map(|b| b.opcode.to_owned())
            .collect::<Vec<_>>()
    );
    println!("{:#?}", proj.title)
}
