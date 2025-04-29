mod file_content;
mod init;
mod main;
mod new;
mod project_authors;
mod project_fields;
mod project_kind;
mod project_name;
mod project_path;

use project_authors::ProjectAuthors;
use project_fields::ProjectFields;
use project_kind::ProjectKind;
use project_name::ProjectName;
use project_path::ProjectPath;

pub use main::main as new_project;
