use std::path::PathBuf;

use directories::ProjectDirs;

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", "flit").expect("could not determine home directory")
}

pub fn db_path() -> PathBuf {
    project_dirs().data_dir().join("db.sqlite3")
}

pub fn config_path() -> PathBuf {
    project_dirs().config_dir().join("config.toml")
}

pub fn log_path() -> PathBuf {
    project_dirs().cache_dir().join("flit.log")
}
