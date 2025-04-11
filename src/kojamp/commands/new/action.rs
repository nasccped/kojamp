use super::project::Project;
use clap::ArgMatches;

pub fn action(mtc: &ArgMatches) -> i32 {
    let project = Project::from_match(mtc);

    if !project.prompt_allowed() && project.prompt_called() {
        println!("error");
        return 1;
    } else if project.prompt_called() {
        return 0;
    }

    println!("{:?}", project);

    0
}
