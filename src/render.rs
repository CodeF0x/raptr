//! # render.rs
//!
//! Contains all functions regarding templating and
//! generating HTML files from drafts.

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
    /// Creates a new instance of the Tera render engine
    pub fn new(theme_name: &str) -> Self {
        let tera = match Tera::new(
            format!("themes/{}/**/*.html", &theme_name).as_str()
        ) {
            Ok(tera) => tera,
            Err(err) => {
                eprintln!("Could not parse theme: {}", theme_name);
                eprintln!("{}", err);
                exit(1);
            }
        };

        RenderEngine { tera }
    }

    /// Renders the index.html file
    pub fn render_index(&self, config: &Config, user_output_directory: &str, links: Vec<BlogPost>, verbose: bool) {
        let output_directory = PathBuf::from(user_output_directory);

        let mut tera_context = Context::from_serialize(&config).unwrap();
        tera_context.insert("blog_posts", &links);
        let rendered_html = self.tera.render("index.html", &tera_context).unwrap();

        let file_path = output_directory.join("index.html");
        let mut index_file = match File::create(&file_path) {
          Ok(index_file) => index_file,
          Err(err) => {
              errors::display_io_error(err, file_path.to_str().unwrap_or("index.html"), verbose);
              exit(1);
          }
        };
        match index_file.write_all(rendered_html.as_bytes()) {
            Ok(_) => println!("Rendered index.html"),
            Err(err) => {
                errors::display_io_error(err, file_path.to_str().unwrap_or("index.html"), verbose);
                exit(1);
            }
        };
    }

    /// Renders all draft files that are not ignored via header
    pub fn render_blog_posts(&self, user_output_directory: &str, verbose: bool) -> Vec<BlogPost> {
        let mut rendered_posts: Vec<BlogPost> = vec![];

        let output_directory = Path::new(user_output_directory).join("posts");
        if !output_directory.exists() {
            match fs::create_dir(&output_directory) {
                Ok(_) => {},
                Err(err) => {
                    let out_path_str = &output_directory.to_str().unwrap();
                    errors::display_io_error(err, out_path_str, verbose);
                    exit(1);
                }
            }
        }

        let all_drafts = fs::read_dir("./drafts").unwrap();

        for draft in all_drafts {
            let draft_path = draft.unwrap().path();
            let path_str = &draft_path.to_str().unwrap();

            if !path_str.ends_with(".md") {
                continue;
            }

            let draft_content = match fs::read_to_string(&draft_path) {
                Ok(content) => content,
                Err(err) => {
                    errors::display_io_error(err, path_str, verbose);
                    exit(1);
                }
            };

            let split_draft: Vec<&str> = draft_content.split("---").collect();
            let draft_header = split_draft[1];
            let blog_meta_data: BlogMetaData = match toml::from_str(draft_header) {
                Ok(context) => context,
                Err(_) => {
                    eprintln!("Could not process {:?}. Invalid TOML in draft header.", draft_path);
                    exit(1);
                }
            };

            if blog_meta_data.draft {
                continue;
            }

            let html_str = split_draft[2];

            let mut options = ComrakOptions::default();
            options.extension.strikethrough = true;
            options.extension.table = true;
            options.extension.tasklist = true;
            options.extension.superscript = true;
            options.extension.footnotes = true;

            let parsed_post_html = markdown_to_html(html_str, &options);
            let mut tera_context = Context::from_serialize(&blog_meta_data).unwrap();
            tera_context.insert("html", &parsed_post_html);
            let human_friendly_date: &str = blog_meta_data.date.split(" ").collect::<Vec<&str>>()[0];
            tera_context.insert("human_friendly_date", human_friendly_date);

            let rendered_file_html = self.tera.render("partials/blog.html", &tera_context).unwrap();

            let file_name = draft_path.file_name().unwrap().to_str().unwrap().replace(".md", ".html");
            let mut blog_file = match File::create(
                format!("{}/posts/{}", &user_output_directory, &file_name)
            ) {
                Ok(blog_file) => blog_file,
                Err(err) => {
                    errors::display_io_error(err, &path_str, verbose);
                    exit(1);
                }
            };
            match blog_file.write_all(rendered_file_html.as_bytes()) {
                Ok(_) => println!("Rendered {}", file_name),
                Err(err) => {
                    errors::display_io_error(err, &path_str, verbose);
                    exit(1);
                }
            }

            let date_time = DateTime::parse_from_str(&blog_meta_data.date, "%Y-%m-%d %H:%M:%S%.f %z").unwrap();

            rendered_posts.push(BlogPost {
                age_in_seconds: date_time.timestamp(),
                title: blog_meta_data.title,
                url: file_name
            });
            // newest post should be first in array
            rendered_posts.sort_by(|a, b| b.age_in_seconds.cmp(&a.age_in_seconds));
        }
        rendered_posts
    }
}

