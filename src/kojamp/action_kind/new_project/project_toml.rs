use crate::globals::PROGRAM_REPO_URL;

use super::ProjectFields;
use serde::Serialize;
use std::rc::Rc;
use toml;

#[derive(Serialize)]
struct ProjectToml<'a> {
    name: &'a str,
    kind: &'static str,
    authors: Option<Vec<Rc<str>>>,
}

#[derive(Serialize)]
pub struct MainToml<'a> {
    #[serde(rename = "project")]
    values: ProjectToml<'a>,
}

impl<'a> From<&'a ProjectFields> for MainToml<'a> {
    fn from(value: &'a ProjectFields) -> Self {
        Self {
            values: ProjectToml {
                name: value.get_name().get_inner(),
                kind: value.get_kind().as_str(),
                authors: value.get_authors().get_inner(),
            },
        }
    }
}

impl<'a> Into<String> for MainToml<'a> {
    fn into(self) -> String {
        let result = [
            "# This file was created using the kojamp-CLI app.",
            "# Manual changes aren't encouraged!",
            "# If you found any error or have trouble using it, consider",
            "# opening the official repository:",
            ("#      ".to_string() + PROGRAM_REPO_URL).as_ref(),
            "\n",
        ]
        .join("\n");

        result + toml::to_string(&self).unwrap().as_ref()
    }
}
