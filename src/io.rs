use sailfish::TemplateOnce;
use std::path::Path;
use std::io::Error;
use std::fs;
use crate::config::{Config};

/// Creates directories and files for a new project.
pub fn create_new_project(project_name: &str) -> Result<(), Error> {
    let new_project_dir = Path::new("./").join(project_name);
    fs::create_dir(&new_project_dir)?;

    let sub_dirs = vec!["drafts", "target", "themes"];

    for dir in sub_dirs {
        let dir_to_create = new_project_dir.join(dir);
        fs::create_dir(dir_to_create)?;
    }

    Ok(())
}

pub fn register_theme(config: &Config) -> Result<(), Error> {
    #[derive(TemplateOnce)]
    #[template(path = "default/index.stpl")]
    struct IndexPage<'a> {
        data: &'a Config
    };

    let ctx = IndexPage {
        data: config
    };

    println!("{}", ctx.render_once().unwrap());

    Ok(())
}