use super::{helper, project::ProjectComposition, report};
use clap::ArgMatches;

pub fn exec(matches: &ArgMatches) -> i32 {
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

    let project: ProjectComposition;
    let is_verbose = matches.get_flag("verbose");

    if helper::only_prompt_called(matches) {
        project = ProjectComposition::new_from_prompt_mode(matches);
    } else if helper::prompt_called(matches) {
        report::prompt_not_allowed();
        return 1;
    } else {
        project = ProjectComposition::new_from_matches(matches);
        let (name, _, p_type, authors, _) = project.destructure();

        let mut invalid_count = 0;
        let checkers: [(bool, fn(bool)); 3] = [
            (!name.is_valid(), report::invalid_name),
            (!p_type.is_valid(), report::invalid_project_type),
            (!authors.is_valid(), report::invalid_authors),
        ];

        checkers.iter().for_each(|(cond, func)| {
            if *cond {
                func(is_verbose);
                invalid_count += 1;
            }
        });

        if invalid_count > 0 {
            return 1;
        }
    }

    println!("Let's build the project");

    0
}
