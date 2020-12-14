use sailfish::TemplateOnce;
use std::path::Path;
use std::io::{prelude::*, Error};
use std::fs;
use crate::config::{Config};
use std::fs::File;
use comrak::{markdown_to_html, ComrakOptions};
use fs_extra::dir::{copy, CopyOptions};

/// Creates directories and files for a new project.
pub fn create_new_project(project_name: &str) -> Result<(), Error> {
    let new_project_dir = Path::new("./").join(project_name);
    fs::create_dir(&new_project_dir)?;

    let sub_dirs = vec!["drafts", "output", "templates", "output/posts"];

    for dir in sub_dirs {
        let dir_to_create = new_project_dir.join(dir);
        fs::create_dir(dir_to_create)?;
    }

    Ok(())
}

/// Copies assets etc. to output path
pub fn copy_theme_files() {
    let mut options = CopyOptions::new();
    options.overwrite = true;
    let _ = copy("templates/default/assets", "output", &options).unwrap();
}

/// Renders the index page.
pub fn render_index(config: &Config) -> Result<(), Error> {
    #[derive(TemplateOnce)]
    #[template(path = "default/index.stpl")]
    struct Index<'a> {
        data: &'a Config
    }

    let ctx = Index {
        data: config
    };

    let rendered_output = ctx.render_once().unwrap();
    let mut index_file = File::create("output/index.html")?;
    index_file.write_all(rendered_output.as_bytes())?;

    Ok(())
}

/// Renders every blog post.
pub fn render_blog(config: &Config) -> Result<(), Error> {
    let mut parse_options = ComrakOptions::default();
    parse_options.extension.table = true;
    parse_options.extension.tasklist = true;
    parse_options.extension.footnotes = true;
    parse_options.extension.description_lists = true;
    parse_options.extension.strikethrough = true;

    #[derive(TemplateOnce)]
    #[template(path = "default/partials/blog.stpl")]
    struct BlogPage {
        text: String
    };

    if let Ok(drafts) = fs::read_dir("drafts") {
        for path in drafts {
            if let Ok(path) = path {
                let markdown = fs::read_to_string(path.path())?;
                let html = markdown_to_html(&markdown, &parse_options);
        
                let ctx = BlogPage {
                    text: html
                };
        
                let result = ctx.render_once().unwrap();
                let mut blog_file = File::create(
                    format!("output/posts/{}.html", path.file_name().into_string().unwrap())
                )?;
                blog_file.write_all(result.as_bytes())?;
            }
        }
    }


    
    Ok(())
}