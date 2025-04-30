use super::{init, new, ProjectAuthors, ProjectFields, ProjectKind, ProjectName, ProjectPath};
use crate::utils::array::ToText;
use clap::ArgMatches;
use std::io::{Error, ErrorKind};

const INVALID_NAME: [&str; 15] = [
    "Couldn't create a new project due to invalid name!",
    "",
    "Accordingly to Java/Kotlin rules, a file/class name should",
    "be &&",
    "\x1b[3;96mCamelCased\x1b[0m &&",
    "and only alpha numeric. So, your project name",
    "should start with an uppercase letter and have no",
    "special chars. &&",
    "\x1b[3;90m(accents, whitespaces, ... you get it)\x1b[0m",
    "",
    "Still, you can specify the path name by using `&&",
    "\x1b[92m--path\x1b[0m \x1b[93m<PATH_HERE>\x1b[0m` &&",
    "flag.",
    "\x1b[3;96mNOTE:\x1b[0m &&",
    "\x1b[3;90m(`--path` flag is only available for the `new` subcommand)\x1b[0m",
];

const INVALID_KIND: [&str; 15] = [
    "Couldn't create a new project due to invalid project kind!",
    "",
    "This is a Java/Kotlin project manager, so these",
    "kinds are expected, but an invalid one was found!",
    "",
    "You can specify a project kind by using",
    "the `\x1b[92m--kind \x1b[93m<PROJECT_KIND>\x1b[0m` &&",
    "flag.",
    "",
    "\x1b[3;96mNOTE:\x1b[0m &&",
    "\x1b[3;93m<PROJECT_KIND>\x1b[0m\x1b[3m &&",
    "can only be &&",
    "`\x1b[3;94mJava\x1b[0m`\x1b[3m or &&",
    "`\x1b[3;94mKotlin\x1b[0m\x1b[3m` &&",
    "\x1b[90m(No case sensitive)\x1b[0m",
];

const INVALID_CUR_DIR: [&str; 7] = [
    "\x1b[91mstd\x1b[97m::\x1b[91menv\x1b[97m::\x1b[91mcurrent_dir()\x1b[0m &&",
    "returned an error!",
    "",
    "The reasons may be:",
    "  \x1b[96ma)\x1b[0m The current path doesn't exists",
    "  \x1b[96mb)\x1b[0m You don't have enough permissions &&",
    "\x1b[3;90m(no sudo or Admin)\x1b[0m",
];

const INVALID_PATH_WHEN_NEW: [&str; 23] = [
    "Couldn't create a project due to invalid path!",
    "The path returned fail when doing validation tests.",
    "",
    "This can occur due to some reasons:",
    "  \x1b[96ma)\x1b[0m You've set a compound path &&",
    "\x1b[3;90m(with '/' chars)\x1b[0m",
    "  \x1b[96mb)\x1b[0m You've used dots when seting the path",
    "  \x1b[96mc)\x1b[0m The path already exists",
    "  \x1b[96md)\x1b[0m You're near to the root path",
    "",
    "Also, this can occur if you don't specify a path name!",
    "The program build a kebab-case path based on your",
    "project name, so, if you create a new project called &&",
    "`\x1b[92mCoolJava\x1b[0m`",
    "and there's a directory called `&&",
    "\x1b[93mcool-java\x1b[0m`, the action will fail!",
    "",
    "You can use `&&",
    "\x1b[92mkojamp new &&",
    "\x1b[93m<PROJECT_NAME> &&",
    "\x1b[90m--path &&",
    "\x1b[93m<PATH_NAME>\x1b[0m",
    "in this case.",
];

const INVALID_PATH_WHEN_INIT: [&str; 7] = [
    "Couldn't create a project due to invalid path!",
    "The path returned fail when doing validation tests.",
    "",
    "If you're seeing this message, you're probably",
    "near to the root path.",
    "Avoid creating projects here. It can conflict with",
    "the program's path validation tests!",
];

pub fn main(pair: (&str, ArgMatches)) -> Result<(), Error> {
    let (cmd, matching) = (pair.0, &pair.1);
    let name = ProjectName::from(matching);
    let kind = ProjectKind::from(matching);
    let path = ProjectPath::try_new();

    if !name.is_valid() {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_NAME.to_text()));
    }

    if !kind.is_valid() {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_KIND.to_text()));
    }

    let is_new_subcommand: bool;

    let path = match (cmd, path) {
        (_, Err(_)) => return Err(Error::new(ErrorKind::Other, INVALID_CUR_DIR.to_text())),
        ("new", Ok(mut x)) => {
            is_new_subcommand = true;
            if x.add_from_matching(matching).is_none() {
                x.add_from_project_name(&name);
            }
            x
        }
        (_, Ok(x)) => {
            is_new_subcommand = false;
            x
        }
    };

    if !path.is_valid(!is_new_subcommand) {
        return Err(Error::new(
            ErrorKind::Other,
            if is_new_subcommand {
                INVALID_PATH_WHEN_NEW.to_text()
            } else {
                INVALID_PATH_WHEN_INIT.to_text()
            },
        ));
    }

    let project_fields: ProjectFields = ProjectFields::new()
        .set_name(name)
        .set_kind(kind)
        .set_path(path)
        .set_authors(ProjectAuthors::from(matching))
        .set_repo(matching.get_flag("no-git"))
        .build();

    let output = if is_new_subcommand {
        new::new(project_fields)
    } else {
        init::init(project_fields)
    };

    if output.is_ok() {
        // TODO:
        println!("TODO")
    }

    output
}
