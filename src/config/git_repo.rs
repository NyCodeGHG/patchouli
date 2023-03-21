use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{de::Visitor, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitRepo {
    inner: GitRepoInner,
}

impl GitRepo {
    pub fn https_url(&self) -> Option<String> {
        match &self.inner {
            GitRepoInner::GitHub { owner, name } => {
                Some(format!("https://github.com/{owner}/{name}.git"))
            }
            GitRepoInner::GitLab { owner, name } => {
                Some(format!("https://gitlab.com/{owner}/{name}.git"))
            }
            GitRepoInner::Https(url) => Some(url.clone()),
            _ => None,
        }
    }

    pub fn ssh_url(&self) -> Option<String> {
        match &self.inner {
            GitRepoInner::GitHub { owner, name } => Some(format!("git@github.com:{owner}/{name}")),
            GitRepoInner::GitLab { .. } => todo!("SSH not yet supported for GitLab."),
            GitRepoInner::Ssh(url) => Some(url.clone()),
            _ => None,
        }
    }
}

lazy_static! {
    static ref GITHUB_REGEX: Regex =
        Regex::new(r"(?:https://github\.com/)?(?P<owner>[^/]+)/(?P<name>.+)").unwrap();
}

impl FromStr for GitRepo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = GITHUB_REGEX.captures(s) {
            return Ok(GitRepo {
                inner: GitRepoInner::GitHub {
                    owner: captures["owner"].to_string(),
                    name: captures["name"].to_string(),
                },
            });
        }
        todo!()
    }
}

impl<'de> Deserialize<'de> for GitRepo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(GitRepoVisitor)
    }
}

struct GitRepoVisitor;

impl<'de> Visitor<'de> for GitRepoVisitor {
    type Value = GitRepo;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a git repo url")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(|e| E::custom(e))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
enum GitRepoInner {
    GitHub { owner: String, name: String },
    GitLab { owner: String, name: String },
    Https(String),
    Ssh(String),
}

#[cfg(test)]
mod tests {
    use crate::config::git_repo::GitRepo;
    use crate::config::git_repo::GitRepoInner;

    #[test]
    fn parse_simple_github_repo() {
        assert_eq!(
            GitRepo {
                inner: GitRepoInner::GitHub {
                    owner: "NyCodeGHG".to_string(),
                    name: "stellwerksim-source".to_string()
                }
            },
            "NyCodeGHG/stellwerksim-source".parse().unwrap()
        )
    }

    #[test]
    fn parse_full_github_repo() {
        assert_eq!(
            GitRepo {
                inner: GitRepoInner::GitHub {
                    owner: "NyCodeGHG".to_string(),
                    name: "stellwerksim-source".to_string()
                }
            },
            "https://github.com/NyCodeGHG/stellwerksim-source"
                .parse()
                .unwrap()
        )
    }
}
