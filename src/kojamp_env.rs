use std::collections::HashMap;

const KOJAMP_ENV_VARS: &str = include_str!("../kojamp.env");

type StStr = &'static str;

pub struct KojampEnvStruct {
    project_name: StStr,
    project_about: StStr,
    project_author: StStr,
    project_current_version: StStr,
    project_repository: StStr,
}

fn get_env_var_pairs(source: StStr) -> HashMap<&'static str, &'static str> {
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

impl KojampEnvStruct {
    pub fn new() -> Self {
        let hash_source = get_env_var_pairs(KOJAMP_ENV_VARS);

        let project_name = hash_source.get("PROJECT_NAME").map_or("kojamp", |v| v);

        let project_author = hash_source
            .get(&"AUTHOR")
            .map_or("nasccped <pdbt.contact@gmail.com>", |v| v);

        let project_current_version = hash_source
            .get("CURRENT_VERSION")
            .map_or("UNDEFINED VERSION AT `kojamp.env`", |v| v);

        let project_repository = hash_source
            .get("REPOSITORY")
            .map_or("https://github.com/nasccped/kojamp", |v| v);

        let project_about = hash_source
            .get("ABOUT")
            .map_or("UNDEFINED ABOUT AT `kojamp.env`", |v| v);

        Self {
            project_name,
            project_about,
            project_repository,
            project_author,
            project_current_version,
        }
    }

    pub fn get_name(&self) -> StStr {
        self.project_name
    }

    pub fn get_author(&self) -> StStr {
        self.project_author
    }

    pub fn get_cur_version(&self) -> StStr {
        self.project_current_version
    }

    pub fn get_repo(&self) -> StStr {
        self.project_repository
    }

    pub fn get_about(&self) -> StStr {
        self.project_about
    }
}
