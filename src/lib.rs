mod lua;

use std::convert::TryFrom;

use lua::process_config;
use serde::{Deserialize, Serialize};
use tinytemplate::{format_unescaped, TinyTemplate};

type TemplateError = tinytemplate::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error while generating template")]
    Template(#[from] TemplateError),
    #[error("lua error {0}")]
    Lua(#[from] mlua::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub phony: bool,
    pub name: String,
    pub prerequisites: Vec<String>,
    pub rules: Vec<String>,
}

impl Target {
    pub fn new(name: &str) -> Self {
        Self {
            phony: false,
            name: name.to_string(),
            prerequisites: vec![],
            rules: vec![],
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Configuration {
    pub targets: Vec<Target>,
}

impl TryFrom<&str> for Configuration {
    type Error = Error;

    fn try_from(src: &str) -> Result<Self> {
        Ok(process_config(Self::new(), src)?)
    }
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            targets: vec![
                Target {
                    name: String::from("all"),
                    phony: true,
                    prerequisites: vec![String::from("$(NAME)")],
                    rules: vec![],
                },
                Target {
                    name: String::from("clean"),
                    phony: true,
                    prerequisites: vec![String::new()],
                    rules: vec![],
                },
                Target {
                    name: String::from("fclean"),
                    phony: true,
                    prerequisites: vec![String::from("clean")],
                    rules: vec![],
                },
                Target {
                    name: String::from("re"),
                    phony: true,
                    prerequisites: vec![String::from("fclean all")],
                    rules: vec![],
                },
            ],
        }
    }

    pub fn generate(&self) -> Result<String> {
        let mut tt = TinyTemplate::new();

        tt.set_default_formatter(&format_unescaped);
        tt.add_template("target", include_str!("./templates/target.mk"))?;
        tt.add_template("body", include_str!("./templates/body.mk"))?;

        Ok(tt.render("body", self)?.trim().to_string())
    }
}
