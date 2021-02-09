use tera::{Tera, Context};
use crate::config::Config;
use std::fs::File;
use std::io::{prelude::*};
use std::fs;
use serde_derive::{Deserialize, Serialize};
use comrak::{markdown_to_html, ComrakOptions};
use crate::errors;
use std::process::exit;
use std::path::{Path, PathBuf};
use chrono::prelude::*;

pub struct RenderEngine {
    pub tera: Tera
}

#[derive(Deserialize, Serialize)]
struct BlogMetaData {
    title: String,
    author: String,
    author_link: String,
    date: String,
    keywords: Vec<String>,
    description: String,
    draft: bool
}

#[derive(Deserialize, Serialize)]
pub struct BlogPost {
    age_in_seconds: i64,
    title: String,
    url: String
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

    pub fn render_index(&self, config: &Config, output_dir: &str, links: Vec<BlogPost>, verbose: bool) {
        let mut out_path = PathBuf::from(output_dir);
        if !out_path.exists() {
            match fs::create_dir_all(output_dir) {
                Ok(_) => println!("Created new directory {}", output_dir),
                Err(err) => {
                    errors::display_io_error(err, output_dir, verbose);
                    exit(1);
                }
            }
        }

        let mut tera_context = Context::from_serialize(&config).unwrap();
        tera_context.insert("blog_posts", &links);
        let rendered_html = self.tera.render("index.html", &tera_context).unwrap();

        out_path.push("index.html");
        let out_path_str = out_path.to_str().unwrap();
        let mut index_file = match File::create(&out_path) {
          Ok(index_file) => index_file,
          Err(err) => {
              errors::display_io_error(err, out_path_str, verbose);
              exit(1);
          }
        };
        match index_file.write_all(rendered_html.as_bytes()) {
            Ok(_) => println!("Rendered index.html"),
            Err(err) => {
                errors::display_io_error(err, out_path_str, verbose);
                exit(1);
            }
        };
    }

    pub fn render_blog_posts(&self, output_dir: &str, verbose: bool) -> Vec<BlogPost> {
        let mut rendered_posts: Vec<BlogPost> = vec![];

        let out_path = Path::new(output_dir).join("posts");
        if !out_path.exists() {
            match fs::create_dir_all(&out_path) {
                Ok(_) => {},
                Err(err) => {
                    let out_path_str = &out_path.to_str().unwrap();
                    errors::display_io_error(err, out_path_str, verbose);
                    exit(1);
                }
            }
        }

        let all_drafts = fs::read_dir("./drafts").unwrap();

        for draft in all_drafts {
            let path = draft.unwrap().path();
            let path_str = &path.to_str().unwrap();

            if !path_str.ends_with(".md") {
                continue;
            }

            let draft_content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(err) => {
                    errors::display_io_error(err, path_str, verbose);
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

            if context.draft {
                continue;
            }

            let html_str = split_draft[2];

            let mut options = ComrakOptions::default();
            options.extension.strikethrough = true;
            options.extension.table = true;
            options.extension.tasklist = true;
            options.extension.superscript = true;
            options.extension.footnotes = true;

            let blog_html = markdown_to_html(html_str, &options);
            let mut tera_context = Context::from_serialize(&context).unwrap();
            tera_context.insert("html", &blog_html);

            let rendered_html = self.tera.render("partials/blog.html", &tera_context).unwrap();

            let file_name = path.file_name().unwrap().to_str().unwrap().replace(".md", ".html");
            let mut blog_file = match File::create(
                format!("{}/posts/{}", &output_dir, &file_name)
            ) {
                Ok(blog_file) => blog_file,
                Err(err) => {
                    errors::display_io_error(err, path_str, verbose);
                    exit(1);
                }
            };
            match blog_file.write_all(rendered_html.as_bytes()) {
                Ok(_) => println!("Rendered {}", file_name),
                Err(err) => {
                    errors::display_io_error(err, path_str, verbose);
                    exit(1);
                }
            }

            let date_time = DateTime::parse_from_str(&context.date, "%Y-%m-%d %H:%M:%S%.f %z").unwrap();

            rendered_posts.push(BlogPost {
                age_in_seconds: date_time.timestamp(),
                title: context.title,
                url: file_name
            });
            rendered_posts.sort_by(|a, b| b.age_in_seconds.cmp(&a.age_in_seconds))
        }
        rendered_posts
    }
}

