use super::files_and_dirs::creators;
use crate::{
    core::{
        contracts::{AddFrom, GetInner, IsValid},
        models::{ProjectAuthors, ProjectFields, ProjectKind, ProjectName, ProjectPath},
        reporting::{messages, KojampReport, ReportType},
    },
    globals::PROGRAM_REPO_URL,
};
use clap::ArgMatches;
use colored::Colorize;
use std::{fs, path::PathBuf};

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
    let name_error = |name: &str| {
        KojampReport::new(
            ReportType::Error,
            INVALID_PROJECT_NAME,
            messages::invalid_project_name(name),
        )
    };
    let kind_error = |kind: &str| {
        KojampReport::new(
            ReportType::Error,
            INVALID_PROJECT_KIND,
            messages::invalid_project_kind(kind),
        )
    };
    let path_error2 = |path: &PathBuf| {
        KojampReport::new(
            ReportType::Error,
            INVALID_PROJECT_PATH,
            messages::invalid_project_path(path),
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

    let (cmd, matching) = (pair.0, &pair.1);

    let name = ProjectName::from(matching);
    let kind = ProjectKind::from(matching);
    let git_repo = !matching.get_flag("no-git");
    let (new_called, path, force) = if cmd == "new" {
        match (ProjectPath::try_from(matching), ProjectPath::try_new(false)) {
            (Ok(x), _) => (true, Ok(x), false),
            (_, Ok(y)) => {
                let mut path = y;
                path.add_from(&name);
                (true, Ok(path), false)
            }
            _ => (true, Err(()), false),
        }
    } else {
        (
            false,
            ProjectPath::try_new(true),
            matching.get_flag("force"),
        )
    };
    let path = path.map_err(|_| vec![path_error1])?;

    let tests_n_errors: Vec<KojampReport> = [
        (name.is_valid(), name_error(name.get_inner())),
        (kind.is_valid(), kind_error(From::from(&kind))),
        (path.is_valid(), path_error2(&path.get_absolute_path())),
    ]
    .into_iter()
    .filter_map(|(cond, er)| if !cond { Some(er) } else { None })
    .collect();

    if !tests_n_errors.is_empty() {
        return Err(tests_n_errors);
    }

    let project_fields: ProjectFields =
        ProjectFields::new(name, kind, ProjectAuthors::try_from(matching).ok());

    let mut mut_path = path.get_inner();

    if new_called {
        creators::create_project_dir(&mut_path).map_err(|_| {
            vec![dir_file_creation_error(
                COULD_NOT_CREATE_PROJECT_DIR,
                &mut_path,
            )]
        })?;
    }

    let (empty_dir, not_allowed_content) = match (
        dir_is_empty(&mut_path),
        dir_contains(&mut_path, NOT_ALLOWED_CONTENT),
    ) {
        (Ok(x), Ok(y)) => (x, y),
        _ => return Err(vec![could_not_read_dir_error()]),
    };

    if !empty_dir && (!force || not_allowed_content) {
        return Err(vec![non_empty_dir_error()]);
    }

    creators::create_src_dir(&mut mut_path)
        .map_err(|x| vec![dir_file_creation_error(COULD_NOT_CREATE_SRC_DIR, &x)])?;

    creators::create_main_source_file(&mut mut_path, &project_fields)
        .map_err(|x| vec![dir_file_creation_error(COULD_NOT_CREATE_MAIN_SRC_FILE, &x)])?;

    creators::create_toml_file(&mut mut_path, &project_fields)
        .map_err(|x| vec![dir_file_creation_error(COULD_NOT_CREATE_TOML_FILE, &x)])?;

    let mut output: Vec<KojampReport> = Vec::new();

    creators::create_readme_file(&mut mut_path, &project_fields)
        .map(|x| output.push(dir_file_creation_warning(COULD_NOT_CREATE_README_FILE, &x)));

    'git_repository: {
        if !git_repo {
            break 'git_repository;
        }

        if let Some(_) = creators::initialize_git(&mut_path) {
            output.push(git_init_warning());
            break 'git_repository;
        }

        creators::create_git_ignore(&mut mut_path)
            .map(|x| output.push(dir_file_creation_warning(COULD_NOT_CREATE_GITIGNORE, &x)));
    }

    output.push(success_report(
        project_fields.get_name().get_inner(),
        new_called,
        &mut_path,
    ));

    Ok(output)
}
