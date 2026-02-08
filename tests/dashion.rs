use std::path::PathBuf;

#[test]
fn test_dashion() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("dashion.sb3");

    assert!(path.exists());
    let mut proj = sbeditor::Project::from_sb3(path).unwrap();
    // let sprite = proj.get_sprite_by_name("Part 1").unwrap();
    // println!(
    //     "{:#?}",
    //     sprite
    //         .blocks
    //         .values()
    //         .filter_map(|b| if b.inputs.is_empty() {
    //             None
    //         } else {
    //             Some(b.inputs.to_owned())
    //         })
    //         .collect::<Vec<_>>()
    // );

    println!("{}", proj.title);
}
