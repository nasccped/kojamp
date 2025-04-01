use std::collections::HashMap;

type StStr = &'static str;

pub struct KojampEnvStruct {
    project_name: StStr,
    project_about: StStr,
    project_author: StStr,
    project_current_version: StStr,
}

impl KojampEnvStruct {
    pub fn new(env_values: HashMap<StStr, StStr>) -> Self {
        let project_name = env_values.get("PROJECT_NAME").map_or("kojamp", |v| v);

        let project_author = env_values
            .get("AUTHOR")
            .map_or("nasccped <pdbt.contact@gmail.com>", |v| v);

        let project_current_version = env_values
            .get("CURRENT_VERSION")
            .map_or("UNDEFINED VERSION AT `kojamp.env`", |v| v);

        let project_about = env_values
            .get("ABOUT")
            .map_or("UNDEFINED ABOUT AT `kojamp.env`", |v| v);

        Self {
            project_name,
            project_about,
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

    pub fn get_about(&self) -> StStr {
        self.project_about
    }
}
