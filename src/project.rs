use std::fs;
use std::io::Error;
use std::path::Path;
use std::fs::File;
use fs_extra::dir::{copy, CopyOptions};

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

pub fn prepare_output_dir(theme_name: &str) {
    if let Ok(mut entries) = fs::read_dir("templates") {
        if entries.next().is_none() {
            eprintln!("You don't have any themes installed. Please add a theme to the themes directory in your project root.");
            std::process::exit(1);
        }
    }

    let mut options = CopyOptions::new();
    options.overwrite = true;
    let _ = copy(
        format!("themes/{}/assets", theme_name), "output", &options
    ).unwrap();
}

pub fn create_new_draft(theme_name: &str, draft_name: &str) {
    let draft_path = Path::new("drafts").join(
        format!("{}.md", draft_name)
    );

    if draft_path.exists() {
        eprintln!("A draft with that name already exists.");
        std::process::exit(1);
    }

    let _draft_file = File::create(&draft_path).expect("Could not create new draft.");
    fs::copy(
        format!("themes/{}/archetypes/post.md", &theme_name),
        &draft_path
    ).expect("Could not copy archetype to new draft file");
    println!("Created a new draft {}", draft_name);
}