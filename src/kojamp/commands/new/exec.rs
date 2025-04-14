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
        todo!(); // prompt was called but it isn't allowed
        return 1;
    } else if project.prompt_called() {
        todo!(); // start prompt mode
        return 0;
    }

    if !project.name_is_valid() {
        todo!(); // invalid name alert
        return 1;
    }

    if !project.type_is_valid() {
        todo!(); // invalid type alert
        return 1;
    }

    println!("{:?}", project);

    0
}
