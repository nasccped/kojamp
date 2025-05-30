use super::{files_and_dirs::*, reports::*, utils::*};
use crate::core::{
    contracts::{AddFrom, GetInner, IsValid},
    models::{ProjectAuthors, ProjectFields, ProjectKind, ProjectName, ProjectPath},
    reporting::KojampReport,
};
use clap::ArgMatches;

const NOT_ALLOWED_CONTENT: [&str; 3] = ["src", "Kojamp.toml", "out"];

pub fn main(pair: (&str, ArgMatches)) -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    let (cmd, matching) = (pair.0, &pair.1);
    let name = ProjectName::from(matching);
    let kind = ProjectKind::from(matching);
    let git_repo = matching.get_flag("no-git");
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
    let path = path.map_err(|_| vec![path_error1()])?;

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
        create_project_dir(&mut_path).map_err(|_| {
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

    create_src_dir(&mut mut_path)
        .map_err(|x| vec![dir_file_creation_error(COULD_NOT_CREATE_SRC_DIR, &x)])?;

    create_main_source_file(&mut mut_path, &project_fields)
        .map_err(|x| vec![dir_file_creation_error(COULD_NOT_CREATE_MAIN_SRC_FILE, &x)])?;

    create_toml_file(&mut mut_path, &project_fields)
        .map_err(|x| vec![dir_file_creation_error(COULD_NOT_CREATE_TOML_FILE, &x)])?;

    let mut output: Vec<KojampReport> = Vec::new();

    if let Some(x) = create_readme_file(&mut mut_path, &project_fields) {
        output.push(dir_file_creation_warning(COULD_NOT_CREATE_README_FILE, &x));
    }

    'git_repository: {
        if !git_repo {
            break 'git_repository;
        }

        if initialize_git(&mut_path).is_some() {
            output.push(git_init_warning());
            break 'git_repository;
        }

        if let Some(x) = create_git_ignore(&mut mut_path) {
            output.push(dir_file_creation_warning(COULD_NOT_CREATE_GITIGNORE, &x));
        }
    }

    output.push(success_report(
        project_fields.get_name().get_inner(),
        new_called,
        &mut_path,
    ));

    Ok(output)
}
