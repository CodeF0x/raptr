use std::fs;
use std::io::Error;
use std::path::Path;

pub fn create_project(project_name: &str) -> Result<(), Error> {
    let root_dir = Path::new("./").join(&project_name);
    fs::create_dir(&root_dir)?;

    let sub_dirs = vec!["drafts", "output", "themes", "output/posts"];

    for dir in sub_dirs {
        let sub_dir = root_dir.join(dir);
        fs::create_dir(sub_dir)?;
    }

    Ok(())
}