use super::{
    fields_validation, file_content, MainToml, ProjectAuthors, ProjectFields, ProjectKind,
    ProjectName, ProjectPath,
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

const COULD_NOT_GET_THE_CURRENT_DIRECTORY: &str = "Couldn't get the current directory";
const COULD_NOT_READ_PROJECT_FOLDER: &str = "Couldn't read project folder";
const NON_EMPTY_DIR: &str = "Non empty dir";
const PROJECT_CREATED: &str = "`$$$` project created";
const COULD_NOT_CREATE_PROJECT_DIR: &str = "Couldn't create the project directory";
const COULD_NOT_CREATE_SRC_DIR: &str = "Couldn't create the `src` directory";
const COULD_NOT_CREATE_MAIN_SRC_FILE: &str = "Couldn't create the main source file";
const COULD_NOT_CREATE_TOML_FILE: &str = "Couldn't create the .toml file";

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
    let readme_content = file_content::readme(fields);
    let mut output: Option<KojampReport> = None;

    if fs::write(&path, readme_content).is_err() {
        output = Some(KojampReport::new(
            ReportType::Warning,
            "Couldn't Create README File",
            "This is probablye due to memory issue
Even so, the project was created!",
        ));
    }
    path.pop();
    output
}

fn initialize_git_and_create_gitignore(path: &mut PathBuf) -> Option<KojampReport> {
    let mut output: Option<KojampReport> = None;

    if Command::new(GIT_COMMAND)
        .args([GIT_INITIALIZATION_ARG, path.to_str().unwrap()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        output = Some(KojampReport::new(
            ReportType::Warning,
            "Couldn't Initialize A Git Repo",
            "The git program probably doesn't exists on your machine
Even so, the project was created!",
        ));
        return output;
    }

    path.push(GIT_IGNORE_FILE_FULLNAME);
    let gitignore_content = file_content::gitignore();

    if fs::write(&path, gitignore_content).is_err() {
        output = Some(KojampReport::new(
            ReportType::Warning,
            "Couldn't Create The `.gitignore` File",
            "This is probably due to memory issue
Even so, the project was created!",
        ));
    }
    path.pop();
    output
}

fn dir_is_empty(path: &PathBuf) -> Result<bool, ()> {
    Ok(fs::read_dir(path).map_err(|_| ())?.next().is_none())
}

fn print_success(new_was_called: bool, fields: ProjectFields) {
    println!("{}!", "Success".bright_green());
    println!();
    println!(
        "The `{}` project was successfully created at the {} directory!",
        fields.get_name().get_inner().bright_green(),
        if new_was_called {
            format!(
                "`{}`",
                fields
                    .get_path()
                    .get_inner()
                    .to_str()
                    .unwrap()
                    .bright_yellow()
            )
        } else {
            String::from("current")
        }
    );
    println!();
    println!("You can get help about kojamp at");
    println!(
        "it's official repository: {}",
        PROGRAM_REPO_URL.bright_cyan()
    );
}

pub fn main(pair: (&str, ArgMatches)) -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    let path_error = KojampReport::new(
        ReportType::Error,
        COULD_NOT_GET_THE_CURRENT_DIRECTORY,
        messages::invalid_cur_dir(),
    );
    let mut path = ProjectPath::try_new().map_err(|_| vec![path_error])?;
    let (cmd, matching) = (pair.0, &pair.1);
    let name = ProjectName::from(matching);
    let kind = ProjectKind::from(matching);
    let new_called = if cmd == "new" {
        if path.add_from_matching(matching).is_none() {
            path.add_from_project_name(&name);
        }
        true
    } else {
        false
    };

    let tests: Vec<KojampReport> = [
        fields_validation::name_validation(&name),
        fields_validation::kind_validation(&kind),
        fields_validation::path_validation(&path, new_called),
    ]
    .into_iter()
    .filter(|cond| cond.is_err())
    .map(|e| e.unwrap_err())
    .collect();

    if !tests.is_empty() {
        return Err(tests);
    }

    let project_fields: ProjectFields = ProjectFields::new()
        .set_name(name)
        .set_kind(kind)
        .set_path(path)
        .set_authors(ProjectAuthors::from(matching))
        .set_repo(matching.get_flag("no-git"))
        .set_force_mode(if new_called {
            None
        } else {
            Some(matching.get_flag("force"))
        })
        .build();

    let mut path = project_fields.get_path().get_inner();

    if new_called {
        create_project_dir(&path).map_err(|e| vec![e])?;
    }

    match (dir_is_empty(&path), project_fields.is_forced()) {
        (Err(_), _) => {
            return Err(vec![KojampReport::new(
                ReportType::Error,
                COULD_NOT_READ_PROJECT_FOLDER,
                messages::could_not_read_dir_content(),
            )])
        }
        (Ok(false), false) => {
            return Err(vec![KojampReport::new(
                ReportType::Error,
                NON_EMPTY_DIR,
                messages::non_empty_dir_initializing(),
            )])
        }
        _ => {}
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
        "",
    ));
    print_success(new_called, project_fields);

    Ok(output)
}
