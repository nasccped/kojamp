use super::{helper, project::ProjectComposition, report};
use crate::{
    utils::io::{IOReporting, ReportStatus},
    vec_dispbox,
};
use clap::ArgMatches;
use std::fmt;

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

    let mut fail_reporting = IOReporting::new::<_, &str>(
        ReportStatus::Err,
        Some("REPORTING FAILS - `kojamp new` OPERATIONS"),
        None,
        vec_dispbox!["Returned a fail for the following validation tests: "],
    );

    if helper::only_prompt_called(matches) {
        project = ProjectComposition::new_from_prompt_mode(matches);
    } else if helper::prompt_called(matches) {
        report::prompt_not_allowed();
        return 1;
    } else {
        project = ProjectComposition::new_from_matches(matches);
        let (name, _, p_type, authors, _) = project.destructure();

        let mut is_invalid = false;
        let checkers: [(bool, fn(), &str); 3] = [
            (!name.is_valid(), report::invalid_name, "Name"),
            (!p_type.is_valid(), report::invalid_project_type, "Type"),
            (!authors.is_valid(), report::invalid_authors, "Authors"),
        ];

        checkers
            .iter()
            .for_each(|(cond, func, message)| match (*cond, is_verbose) {
                (true, x) => {
                    if fail_reporting.get_how_many_rows() == 1 {
                        fail_reporting.append_message_line("");
                    }
                    is_invalid = true;
                    if x {
                        func();
                    } else {
                        fail_reporting.append_message_line(format!(
                            "  {}- {}{}",
                            "\x1b[96m", message, "\x1b[0m"
                        ));
                    }
                }
                _ => {}
            });

        if is_invalid {
            fail_reporting.append_message_line("");
            fail_reporting.append_message_line(format!(
                "You can try using the same inputs with the {}`--verbose`{} flag.",
                "\x1b[92m", "\x1b[0m",
            ));
        }

        match (is_invalid, is_verbose) {
            (true, true) => {
                return 1;
            }
            (true, false) => {
                fail_reporting.print_content();
                return 1;
            }
            _ => {}
        }
    }

    println!("Let's build the project");

    0
}
