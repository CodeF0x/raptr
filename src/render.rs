use tera::{Tera, Context};
use crate::config::Config;
use std::fs::File;
use std::io::{prelude::*};
use std::fs;
use serde_derive::Deserialize;

pub struct RenderEngine {
    pub tera: Tera
}

#[derive(Deserialize)]
struct BlogMetaData {
    title: String,
    author: String,
    author_link: String,
    draft: bool,
    date: String
}

impl RenderEngine {
    pub fn new(theme_name: &str) -> Self {
        let tera = match Tera::new(
            format!("themes/{}/**/*.html", &theme_name).as_str()
        ) {
            Ok(tera) => tera,
            Err(err) => {
                eprintln!("Parsing error(s): {}", err);
                std::process::exit(1);
            }
        };

        RenderEngine { tera: tera }
    }

    pub fn render_index(&self, config: &Config) {
        let rendered_html = self.tera.render("index.html", &Context::from_serialize(&config).unwrap()).unwrap();
        let mut index_file = File::create("output/index.html").expect("Could not create index.html file");
        index_file.write_all(rendered_html.as_bytes()).expect("Could not write to index.html file.");
    }

    pub fn render_blog_posts(&self, config: &Config) {
        let all_drafts = fs::read_dir("./drafts").expect("Could not read drafts directory.");

        for draft in all_drafts {
            let path = draft.unwrap().path();
            let draft_content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(_) => {
                    eprintln!("Could not read draft {:?}", path);
                    std::process::exit(1);
                }
            };

            let meta_data_str = draft_content.split("---").collect();
            let meta_data: BlogMetaData = toml::from_str(meta_data_str).unwrap();
        }
    }
}

