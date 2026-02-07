You can open a project using the `Project.from_sb3` method.

!!! Warning

    The code below is untested!
    <!-- TODO: test it -->

```rs
use std::path::PathBuf;
use sbeditor::Project;

fn main() {
  let path = PathBuf("new.sb3");
  let proj = sbeditor::Project::from_sb3(path).unwrap();
  println!("{}", proj.title);
}

```
