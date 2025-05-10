use super::{
    file_content, MainToml, ProjectAuthors, ProjectFields, ProjectKind, ProjectName, ProjectPath,
};
use crate::{
    essentials::report::{
        messages,
        types::{KojampReport, ReportType},
    },
    globals::{
        GIT_COMMAND, GIT_IGNORE_FILE_FULLNAME, GIT_INITIALIZATION_ARG, JAVA_FILE_EXTENSION,
        KOTLIN_FILE_EXTENSION, MARKDOWN_FILE_EXTENSION, PROGRAM_REPO_URL, PROGRAM_TOML_FILE_NAME,
        README_FILE_NAME, SRC_DIR, TOML_FILE_EXTENSION,
    },
};
use clap::ArgMatches;
use colored::Colorize;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

const INVALID_PROJECT_NAME: &str = "Invalid project name";
const INVALID_PROJECT_KIND: &str = "Invalid project kind";
const INVALID_PROJECT_PATH: &str = "Invalid project path";
const COULD_NOT_GET_THE_CURRENT_DIRECTORY: &str = "Couldn't get the current directory";
const COULD_NOT_READ_PROJECT_FOLDER: &str = "Couldn't read project folder";
const NON_EMPTY_DIR: &str = "Non empty dir";
const PROJECT_CREATED: &str = "`$$$` project created";
const COULD_NOT_CREATE_PROJECT_DIR: &str = "Couldn't create the project directory";
const COULD_NOT_CREATE_SRC_DIR: &str = "Couldn't create the `src` directory";
const COULD_NOT_CREATE_MAIN_SRC_FILE: &str = "Couldn't create the main source file";
const COULD_NOT_CREATE_TOML_FILE: &str = "Couldn't create the .toml file";
const COULD_NOT_CREATE_README_FILE: &str = "Couldn't create README file";
const COULD_NOT_INITIALIZE_GIT_REPO: &str = "Couldn't initialize a git repo";
const COULD_NOT_CREATE_GITIGNORE: &str = "Couldn't create .gitignore";

const NOT_ALLOWED_CONTENT: [&str; 3] = ["src", "Kojamp.toml", "out"];

fn create_project_dir(path: &PathBuf) -> Result<(), KojampReport> {
    let optional_path: &str = path
        .file_name()
        .map(|f| f.to_str().unwrap_or(""))
        .unwrap_or("");

    if fs::create_dir(path).is_err() {
        return Err(KojampReport::new(
            ReportType::Error,
            COULD_NOT_CREATE_PROJECT_DIR,
            messages::could_not_create_dir_file(optional_path),
        ));
    }

    Ok(())
}

fn create_src_dir(path: &mut PathBuf) -> Result<(), KojampReport> {
    path.push(SRC_DIR);

    let optional_path: String = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("")
        .to_owned();

    if fs::create_dir(&path).is_err() {
        return Err(KojampReport::new(
            ReportType::Error,
            COULD_NOT_CREATE_SRC_DIR,
            messages::could_not_create_dir_file(&optional_path),
        ));
    }

    path.pop();
    Ok(())
}

fn create_main_source_file(path: &mut PathBuf, fields: &ProjectFields) -> Result<(), KojampReport> {
    let (name, kind) = (fields.get_name().get_inner(), fields.get_kind());
    let (ext, content) = match kind {
        ProjectKind::Java => (JAVA_FILE_EXTENSION, file_content::java(name)),
        _ => (KOTLIN_FILE_EXTENSION, file_content::kotlin(name)),
    };

    path.push(SRC_DIR);
    path.push(name);
    path.set_extension(ext);

    if fs::write(&path, content).is_err() {
        return Err(KojampReport::new(
            ReportType::Error,
            COULD_NOT_CREATE_MAIN_SRC_FILE,
            messages::could_not_create_dir_file(path.to_str().unwrap_or("???")),
        ));
    }

    path.pop();
    path.pop();
    Ok(())
}

fn create_toml_file(path: &mut PathBuf, fields: &ProjectFields) -> Result<(), KojampReport> {
    path.push(PROGRAM_TOML_FILE_NAME);
    path.set_extension(TOML_FILE_EXTENSION);
    let toml_content: String = MainToml::from(fields).into();

    if fs::write(&path, toml_content).is_err() {
        return Err(KojampReport::new(
            ReportType::Error,
            COULD_NOT_CREATE_TOML_FILE,
            messages::could_not_create_dir_file(path.to_str().unwrap_or("???")),
        ));
    }

    path.pop();
    Ok(())
}

fn create_readme_file(path: &mut PathBuf, fields: &ProjectFields) -> Option<KojampReport> {
    path.push(README_FILE_NAME);
    path.set_extension(MARKDOWN_FILE_EXTENSION);

    let abspath = std::env::current_dir().unwrap().join(&path);

    if abspath.exists() {
        path.pop();
        return None;
    }

    let readme_content = file_content::readme(fields);
    let mut output: Option<KojampReport> = None;

    if fs::write(&path, readme_content).is_err() {
        output = Some(KojampReport::new(
            ReportType::Warning,
            COULD_NOT_CREATE_README_FILE,
            messages::could_not_create_dir_file(path.to_str().unwrap_or("")),
        ));
    }
    path.pop();
    output
}

fn initialize_git_and_create_gitignore(path: &mut PathBuf) -> Option<KojampReport> {
    let mut output: Option<KojampReport> = None;

    let mut git_abspath = std::env::current_dir().unwrap();
    git_abspath.push(".git");

    if git_abspath.exists() {
        return None;
    }

    if Command::new(GIT_COMMAND)
        .args([GIT_INITIALIZATION_ARG, path.to_str().unwrap()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        output = Some(KojampReport::new(
            ReportType::Warning,
            COULD_NOT_INITIALIZE_GIT_REPO,
            messages::could_not_initialize_git_repo(),
        ));
        return output;
    }

    path.push(GIT_IGNORE_FILE_FULLNAME);
    let gitignore_content = file_content::gitignore();

    if fs::write(&path, gitignore_content).is_err() {
        output = Some(KojampReport::new(
            ReportType::Warning,
            COULD_NOT_CREATE_GITIGNORE,
            messages::could_not_create_dir_file(path.to_str().unwrap_or("???")),
        ));
    }
    path.pop();
    output
}

fn dir_is_empty(path: &PathBuf) -> Result<bool, ()> {
    Ok(fs::read_dir(path).map_err(|_| ())?.next().is_none())
}

fn dir_contains<const N: usize>(path: &PathBuf, items: [&str; N]) -> Result<bool, ()> {
    Ok(fs::read_dir(path)
        .map_err(|_| ())?
        .filter_map(|entry| entry.ok())
        .any(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|name| items.contains(&name))
                .unwrap_or(false)
        }))
}

fn gen_success_message(new_called: bool, path: &PathBuf) -> String {
    let mut may_goto = if new_called {
        vec![format!(
            "cd to your project ({})",
            path.to_str().unwrap_or("???").bright_green()
        )]
    } else {
        vec![]
    };

    may_goto.push(format!("use `{}`", "kojamp run".bright_green()));

    let actions = may_goto
        .into_iter()
        .map(|row| format!("... {} ", "*".bright_green()) + row.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "\
        You can now:\n\
        {}
        \n\
        {}: check the official repo ({})\n\
        for more details.",
        actions,
        "note".bright_cyan(),
        PROGRAM_REPO_URL
    )
}

pub fn main(pair: (&str, ArgMatches)) -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    let path_error1 = KojampReport::new(
        ReportType::Error,
        COULD_NOT_GET_THE_CURRENT_DIRECTORY,
        messages::invalid_cur_dir(),
    );
    let name_error = |name: &ProjectName| {
        KojampReport::new(
            ReportType::Error,
            INVALID_PROJECT_NAME,
            messages::invalid_project_name(name.get_inner()),
        )
    };
    let kind_error = |kind: &ProjectKind| {
        KojampReport::new(
            ReportType::Error,
            INVALID_PROJECT_KIND,
            messages::invalid_project_kind(From::from(kind)),
        )
    };
    let path_error2 = |path: &ProjectPath| {
        KojampReport::new(
            ReportType::Error,
            INVALID_PROJECT_PATH,
            messages::invalid_project_path(&path.get_absolute_path()),
        )
    };

    let mut path = ProjectPath::try_new().map_err(|_| vec![path_error1])?;
    let (cmd, matching) = (pair.0, &pair.1);
    let name = ProjectName::from(matching);
    let kind = ProjectKind::from(matching);
    let (new_called, force) = if cmd == "new" {
        if path.add_from_matching(matching).is_none() {
            path.add_from_project_name(&name);
        }
        (true, false)
    } else {
        (false, matching.get_flag("force"))
    };

    let tests_n_errors: Vec<KojampReport> = [
        (name.is_valid(), name_error(&name)),
        (kind.is_valid(), kind_error(&kind)),
        (path.is_valid(!new_called), path_error2(&path)),
    ]
    .into_iter()
    .filter(|(cond, _)| *cond)
    .map(|(_, e)| e)
    .collect();

    if !tests_n_errors.is_empty() {
        return Err(tests_n_errors);
    }

    let project_fields: ProjectFields = ProjectFields::new()
        .set_name(name)
        .set_kind(kind)
        .set_path(path)
        .set_authors(ProjectAuthors::from(matching))
        .set_repo(matching.get_flag("no-git"))
        .build();

    let mut path = project_fields.get_path().get_inner();

    if new_called {
        create_project_dir(&path).map_err(|e| vec![e])?;
    }

    let (empty_dir, not_allowed_content) = if let (Ok(c1), Ok(c2)) = (
        dir_is_empty(&path),
        dir_contains(&path, NOT_ALLOWED_CONTENT),
    ) {
        (c1, c2)
    } else {
        return Err(vec![KojampReport::new(
            ReportType::Error,
            COULD_NOT_READ_PROJECT_FOLDER,
            messages::could_not_read_dir_content(),
        )]);
    };

    if !empty_dir && !force {
        return Err(vec![KojampReport::new(
            ReportType::Error,
            NON_EMPTY_DIR,
            messages::non_empty_dir_initializing(),
        )]);
    }

    if !empty_dir && not_allowed_content {
        return Err(vec![KojampReport::new(
            ReportType::Error,
            NON_EMPTY_DIR,
            messages::non_empty_dir_initializing(),
        )]);
    }

    create_src_dir(&mut path).map_err(|e| vec![e])?;

    create_main_source_file(&mut path, &project_fields).map_err(|e| vec![e])?;

    create_toml_file(&mut path, &project_fields).map_err(|e| vec![e])?;

    let mut output: Vec<KojampReport> = Vec::new();

    create_readme_file(&mut path, &project_fields).map(|r| output.push(r));

    if project_fields.have_repo() {
        initialize_git_and_create_gitignore(&mut path).map(|r| output.push(r));
    }

    output.push(KojampReport::new(
        ReportType::Success,
        PROJECT_CREATED.replace("$$$", project_fields.get_name().get_inner()),
        gen_success_message(new_called, &path),
    ));

    Ok(output)
}
