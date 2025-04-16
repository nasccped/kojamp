use clap::ArgMatches;

pub fn prompt_called(matches: &ArgMatches) -> bool {
    matches.get_flag("prompt")
}

pub fn only_prompt_called(matches: &ArgMatches) -> bool {
    [
        prompt_called(matches),
        matches.get_one::<String>("name").is_none(),
        matches.get_one::<String>("path").is_none(),
        matches.get_one::<String>("type").is_none(),
        matches
            .get_many("authors")
            .map(|vector| vector.cloned().collect::<Vec<String>>())
            .is_none(),
        !matches.get_flag("no-git"),
    ]
    .iter()
    .all(|cond| *cond)
}
