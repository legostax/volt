use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Package {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_rev")]
    pub rev: String,
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
    pub versions: HashMap<String, Version>,
    pub time: HashMap<String, String>,
    pub maintainers: Vec<Maintainer>,
    pub description: String,
    pub homepage: String,
    pub repository: Repository,
    pub author: Author,
    pub bugs: Bugs,
    pub license: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DistTags {
    pub latest: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Author,
    pub license: String,
    pub repository: Repository,
    pub main: String,
    pub module: String,
    #[serde(rename = "jsnext:main")]
    pub jsnext_main: String,
    pub engines: Engines,
    pub scripts: Scripts,
    pub dependencies: HashMap<String, String>,
    pub peer_dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub git_head: String,
    pub bugs: Bugs,
    pub homepage: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_nodeVersion")]
    pub node_version: String,
    #[serde(rename = "_npmVersion")]
    pub npm_version: String,
    pub dist: Dist,
    pub maintainers: Vec<Maintainer>,
    #[serde(rename = "_npmUser")]
    pub npm_user: NpmUser,
    pub directories: Directories,
    #[serde(rename = "_npmOperationalInternal")]
    pub npm_operational_internal: NpmOperationalInternal,
    #[serde(rename = "_hasShrinkwrap")]
    pub has_shrinkwrap: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Author {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Repository {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Engines {
    pub node: String,
    pub npm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Scripts {
    pub test: String,
    #[serde(rename = "test:watch")]
    pub test_watch: String,
    pub build: String,
    pub start: String,
    pub prepare: String,
    pub predeploy: String,
    pub deploy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Bugs {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Dist {
    pub integrity: String,
    pub shasum: String,
    pub tarball: String,
    pub file_count: i64,
    pub unpacked_size: i64,
    #[serde(rename = "npm-signature")]
    pub npm_signature: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Maintainer {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NpmUser {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Directories {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NpmOperationalInternal {
    pub host: String,
    pub tmp: String,
}