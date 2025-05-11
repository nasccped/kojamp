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

fn create_project_dir(path: &PathBuf) -> Result<(), ()> {
    fs::create_dir(path).map_err(|_| ())
}

fn create_src_dir(path: &mut PathBuf) -> Result<(), PathBuf> {
    path.push(SRC_DIR);
    fs::create_dir(&path).map_err(|_| path.clone())?;
    path.pop();
    Ok(())
}

fn create_main_source_file(path: &mut PathBuf, fields: &ProjectFields) -> Result<(), PathBuf> {
    let (name, kind) = (fields.get_name().get_inner(), fields.get_kind());
    let (ext, content) = match kind {
        ProjectKind::Java => (JAVA_FILE_EXTENSION, file_content::java(name)),
        _ => (KOTLIN_FILE_EXTENSION, file_content::kotlin(name)),
    };

    path.push(SRC_DIR);
    path.push(name);
    path.set_extension(ext);
    fs::write(&path, content).map_err(|_| path.clone())?;
    path.pop();
    path.pop();
    Ok(())
}

fn create_toml_file(path: &mut PathBuf, fields: &ProjectFields) -> Result<(), PathBuf> {
    path.push(PROGRAM_TOML_FILE_NAME);
    path.set_extension(TOML_FILE_EXTENSION);
    let toml_content: String = MainToml::from(fields).into();
    fs::write(&path, toml_content).map_err(|_| path.clone())?;
    path.pop();
    Ok(())
}

fn create_readme_file(path: &mut PathBuf, fields: &ProjectFields) -> Option<PathBuf> {
    path.push(README_FILE_NAME);
    path.set_extension(MARKDOWN_FILE_EXTENSION);

    let abspath = std::env::current_dir().unwrap().join(&path);

    if abspath.exists() {
        path.pop();
        return None;
    }

    let readme_content = file_content::readme(fields);
    let output = fs::write(&path, readme_content).err().map(|_| path.clone());
    path.pop();
    output
}

fn initialize_git(path: &PathBuf) -> Option<()> {
    let mut git_abspath = std::env::current_dir().unwrap();
    git_abspath.push(".git");
    if git_abspath.exists() {
        return None;
    }
    Command::new(GIT_COMMAND)
        .args([GIT_INITIALIZATION_ARG, path.to_str().unwrap()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .err()
        .map(|_| ())
}

fn create_git_ignore(path: &mut PathBuf) -> Option<PathBuf> {
    path.push(GIT_IGNORE_FILE_FULLNAME);
    let gitignore_content = file_content::gitignore();
    let output = fs::write(&path, gitignore_content)
        .err()
        .map(|_| path.clone());
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
        {}: check the official repo ({}) for more details.",
        actions,
        "note".bright_cyan(),
        PROGRAM_REPO_URL
    )
}

pub fn main(pair: (&str, ArgMatches)) -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    fn pathbuf_to_str(path: &PathBuf) -> &str {
        path.file_name()
            .map(|f| f.to_str().unwrap_or(""))
            .unwrap_or("")
    }

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
    let dir_file_creation_error = |title: &str, path: &PathBuf| {
        let as_str = pathbuf_to_str(path);
        KojampReport::new(
            ReportType::Error,
            title,
            messages::could_not_create_dir_file(as_str),
        )
    };
    let dir_file_creation_warning = |title: &str, path: &PathBuf| {
        let as_str = pathbuf_to_str(path);
        KojampReport::new(
            ReportType::Warning,
            title,
            messages::could_not_create_dir_file(as_str),
        )
    };
    let could_not_read_dir_error = || {
        KojampReport::new(
            ReportType::Error,
            COULD_NOT_READ_PROJECT_FOLDER,
            messages::could_not_read_dir_content(),
        )
    };
    let non_empty_dir_error = || {
        KojampReport::new(
            ReportType::Error,
            NON_EMPTY_DIR,
            messages::non_empty_dir_initializing(),
        )
    };
    let git_init_warning = || {
        KojampReport::new(
            ReportType::Warning,
            COULD_NOT_INITIALIZE_GIT_REPO,
            messages::could_not_initialize_git_repo(),
        )
    };
    let success_report = |project_name: &str, new_called: bool, path: &PathBuf| {
        KojampReport::new(
            ReportType::Success,
            PROJECT_CREATED.replace("$$$", project_name),
            gen_success_message(new_called, path),
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
    .filter_map(|(cond, er)| if !cond { Some(er) } else { None })
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
        create_project_dir(&path)
            .map_err(|_| vec![dir_file_creation_error(COULD_NOT_CREATE_PROJECT_DIR, &path)])?;
    }

    let (empty_dir, not_allowed_content) = match (
        dir_is_empty(&path),
        dir_contains(&path, NOT_ALLOWED_CONTENT),
    ) {
        (Ok(x), Ok(y)) => (x, y),
        _ => return Err(vec![could_not_read_dir_error()]),
    };

    if !empty_dir && (!force || not_allowed_content) {
        return Err(vec![non_empty_dir_error()]);
    }

    create_src_dir(&mut path)
        .map_err(|x| vec![dir_file_creation_error(COULD_NOT_CREATE_SRC_DIR, &x)])?;

    create_main_source_file(&mut path, &project_fields)
        .map_err(|x| vec![dir_file_creation_error(COULD_NOT_CREATE_MAIN_SRC_FILE, &x)])?;

    create_toml_file(&mut path, &project_fields)
        .map_err(|x| vec![dir_file_creation_error(COULD_NOT_CREATE_TOML_FILE, &x)])?;

    let mut output: Vec<KojampReport> = Vec::new();

    create_readme_file(&mut path, &project_fields)
        .map(|x| output.push(dir_file_creation_warning(COULD_NOT_CREATE_README_FILE, &x)));

    'git_repository: {
        if !project_fields.have_repo() {
            break 'git_repository;
        }

        if let Some(_) = initialize_git(&path) {
            output.push(git_init_warning());
            break 'git_repository;
        }

        create_git_ignore(&mut path)
            .map(|x| output.push(dir_file_creation_warning(COULD_NOT_CREATE_GITIGNORE, &x)));
    }

    output.push(success_report(
        project_fields.get_name().get_inner(),
        new_called,
        &path,
    ));

    Ok(output)
}
