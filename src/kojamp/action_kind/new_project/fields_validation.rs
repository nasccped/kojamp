use super::{ProjectKind, ProjectName, ProjectPath};
use crate::{
    essentials::report::{
        messages,
        types::{KojampReport, ReportType},
    },
    utils::array::ToText,
};

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

pub fn name_validation(name: &ProjectName) -> Result<(), KojampReport> {
    if !name.is_valid() {
        return Err(KojampReport::new(
            ReportType::Error,
            "Invalid Project Name",
            messages::invalid_project_name(name.get_inner()),
        ));
    }

    Ok(())
}

pub fn kind_validation(kind: &ProjectKind) -> Result<(), KojampReport> {
    if !kind.is_valid() {
        return Err(KojampReport::new(
            ReportType::Error,
            "Invalid Project Kind",
            INVALID_KIND.to_text(),
        ));
    }

    Ok(())
}

pub fn path_validation(path: &ProjectPath, new_called: bool) -> Result<(), KojampReport> {
    let invalid_path_title = "Invalid Project Path";
    match (path.is_valid(!new_called), new_called) {
        (true, _) => Ok(()),
        (_, true) => Err(KojampReport::new(
            ReportType::Error,
            invalid_path_title,
            INVALID_PATH_WHEN_NEW.to_text(),
        )),
        _ => Err(KojampReport::new(
            ReportType::Error,
            invalid_path_title,
            INVALID_PATH_WHEN_INIT.to_text(),
        )),
    }
}
