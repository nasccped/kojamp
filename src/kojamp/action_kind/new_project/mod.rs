mod file_content;
mod main;
mod project_authors;
mod project_fields;
mod project_kind;
mod project_name;
mod project_path;
mod project_toml;

use project_authors::ProjectAuthors;
use project_fields::ProjectFields;
use project_kind::ProjectKind;
use project_name::ProjectName;
use project_path::ProjectPath;
use project_toml::MainToml;

pub use main::main as new_project;
