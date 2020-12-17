//! <https://pre-commit.com/>

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub repos: Vec<Repo>,
}

#[derive(Deserialize)]
pub struct Repo {
    pub hooks: Vec<Hook>,
}

#[derive(Deserialize)]
pub struct Hook {
    pub entry: String,
}
