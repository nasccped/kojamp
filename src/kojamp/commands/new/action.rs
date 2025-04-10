use super::project::Project;
use clap::ArgMatches;

pub fn action(mtc: &ArgMatches) -> i32 {
    let mut project = Project::from_match(mtc);

    if !project.prompt_allowed() && project.prompt_called() {
        println!("error");
        return 1;
    } else if project.prompt_called() {
        project.prompt_mode();
        return 0;
    }

    println!("{:?}", project);

    0
}
