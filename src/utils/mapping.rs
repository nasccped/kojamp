use std::collections::HashMap;

type StStr = &'static str;

pub fn turn_env_into_hashmap(source: StStr) -> HashMap<StStr, StStr> {
    source
        .lines()
        .map(|row| row.trim())
        .filter(|row| !row.is_empty())
        .filter(|row| !row.starts_with("#"))
        .map(|row| {
            let mut splited = row.split("=");
            let key = splited.next();
            let value = splited.next();
            (key, value)
        })
        .enumerate()
        .fold(
            HashMap::new(),
            |mut map: HashMap<&str, &str>,
             (i, (k, v)): (usize, (Option<StStr>, Option<&'static str>))| {
                map.insert(
                    k.unwrap_or_else(|| panic!("UNDEFINED KEY AT ITER: {i}")),
                    v.unwrap_or_else(|| panic!("UNDEFINED VALUE AT ITER: {i}")),
                );
                map
            },
        )
}
