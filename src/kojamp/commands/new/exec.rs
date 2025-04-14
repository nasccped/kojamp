use super::project::ProjectFields;
use clap::ArgMatches;

pub fn exec(mtc: &ArgMatches) -> i32 {
    /* TODO:
     *   1. build project structu by using &ArgMatches
     *   2. test if prompt was called and if it's allowed
     *       - if so, start prompt mode
     *       - if not allowed, return an error
     *       - if not called, continue
     *   3. test if name is valid. If not, report error
     *   4. test if project type is valid. If not, report error
     *   5. build the project (done - returning success)
     *
     * */
    let project = ProjectFields::from_match(mtc);

    if !project.prompt_allowed() && project.prompt_called() {
        println!("error");
        return 1;
    } else if project.prompt_called() {
        return 0;
    }

    println!("{:?}", project);

    0
}
