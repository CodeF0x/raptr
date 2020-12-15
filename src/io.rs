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
pub fn render_index(config: &Config) -> Result<(), String> {
    #[derive(TemplateOnce)]
    #[template(path = "default/index.stpl")]
    struct Index<'a> {
        data: &'a Config
    }

    let ctx = Index {
        data: config
    };

    let error_message = String::from("Could not render index.html: ");

    let rendered_output = ctx.render_once().unwrap();
    let mut index_file = match File::create("output/index.html") {
        Ok(index_file) => index_file,
        Err(err) => return Err(format!("{}{}", error_message, err))
    };
    match index_file.write_all(rendered_output.as_bytes()) {
        Ok(_) => {},
        Err(err) => return Err(format!("{}{}", error_message, err))
    };

    println!("Generated index.html.");
    Ok(())
}

/// Renders every blog post.
pub fn render_blog() -> Result<(), String> {
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
        // todo: This is ugly, change!!
        let mut drafts_avaiable = false;

        println!("Rendering blog posts...");
        for path in drafts {
            drafts_avaiable = true;

            if let Ok(path) = path {
                let file_name = path.file_name().into_string().unwrap();
                let error_message = format!("Error: Could not render file {}: ", path.file_name().into_string().unwrap());

                let markdown = match fs::read_to_string(path.path()) {
                    Ok(markdown) => markdown,
                    Err(err) => return Err(format!("{}{:?}", error_message, err)),
                };
                let html = markdown_to_html(&markdown, &parse_options);
        
                let ctx = BlogPage {
                    text: html
                };
        
                let result = ctx.render_once().unwrap();
                let mut blog_file = match File::create(
                    format!("output/posts/{}.html", file_name)
                ) {
                    Ok(blog_file) => blog_file,
                    Err(err) => return Err(format!("{}{:?}", error_message, err))
                };
                match blog_file.write_all(result.as_bytes()) {
                    Ok(_) => println!("Generated file {}", file_name),
                    Err(err) => return Err(format!("{}{:?}", error_message, err))
                };
            }
        }
        
        if !drafts_avaiable {
            println!("Warning: You don't have any drafts, therefore no blog posts have been generated.");
            return Ok(());
        }

    } else {
        return Err(String::from("Error: Cant't find drafts directory. Aborting."));
    }
    
    println!("Generated blog posts.");
    Ok(())
}