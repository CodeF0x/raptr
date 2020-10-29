use std::path::Path;
use std::io::Error;
use std::fs;

pub fn create_new_project(project_name: &str) -> Result<(), Error> {
    let new_project_dir = Path::new("./").join(project_name);
    fs::create_dir(&new_project_dir)?;

    let draft_dir = Path::new(&new_project_dir).join("drafts");
    fs::create_dir(draft_dir)?;

    let output_dir = Path::new(&new_project_dir).join("target");
    fs::create_dir(output_dir)?;
    Ok(())
}