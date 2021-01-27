use tera::{Tera, Context};
use crate::config::Config;
use std::fs::File;
use std::io::{prelude::*};
use std::fs;
use serde_derive::{Deserialize, Serialize};
use comrak::{markdown_to_html, ComrakOptions};
use crate::errors;
use std::process::exit;

pub struct RenderEngine {
    pub tera: Tera
}

#[derive(Deserialize, Serialize)]
struct BlogMetaData {
    title: String,
    author: String,
    author_link: String,
    date: String,
    draft: bool
}

impl RenderEngine {
    pub fn new(theme_name: &str) -> Self {
        let tera = match Tera::new(
            format!("themes/{}/**/*.html", &theme_name).as_str()
        ) {
            Ok(tera) => tera,
            Err(err) => {
                eprintln!("Parsing error(s): {}", err);
                exit(1);
            }
        };

        RenderEngine { tera }
    }

    pub fn render_index(&self, config: &Config) {
        let rendered_html = self.tera.render("index.html", &Context::from_serialize(&config).unwrap()).unwrap();
        let mut index_file = match File::create("output/index.html") {
          Ok(index_file) => index_file,
          Err(err) => {
              errors::display_io_error(err.kind(), "output/index.html");
              exit(1);
          }
        };
        match index_file.write_all(rendered_html.as_bytes()) {
            Ok(_) => println!("Generated index.html."),
            Err(err) => {
                errors::display_io_error(err.kind(), "output/index.html");
                exit(1);
            }
        };
    }

    pub fn render_blog_posts(&self) {
        let all_drafts = fs::read_dir("./drafts").unwrap();

        for draft in all_drafts {
            let path = draft.unwrap().path();
            let path_str = &path.to_str().unwrap();

            let draft_content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(err) => {
                    errors::display_io_error(err.kind(), path_str);
                    exit(1);
                }
            };

            let split_draft: Vec<&str> = draft_content.split("---").collect();
            let toml_str = split_draft[1];
            let context: BlogMetaData = match toml::from_str(toml_str) {
                Ok(context) => context,
                Err(_) => {
                    eprintln!("Could not process {:?}. Invalid TOML in draft header.", path);
                    exit(1);
                }
            };

            let rendered_html = self.tera.render("partials/blog.html", &Context::from_serialize(&context).unwrap()).unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap().replace(".md", ".html");
            let mut blog_file = match File::create(
                format!("output/posts/{}", &file_name)
            ) {
                Ok(blog_file) => blog_file,
                Err(err) => {
                    errors::display_io_error(err.kind(), path_str);
                    exit(1);
                }
            };
            match blog_file.write_all(rendered_html.as_bytes()) {
                Ok(_) => println!("Rendered {}", file_name),
                Err(err) => {
                    errors::display_io_error(err.kind(), path_str);
                    exit(1);
                }
            }
        }
    }
}

