use std::env;
use ansi_term::Colour::{Cyan, Blue, Red};
use ansi_term::ANSIStrings;
use git2::{self, Repository, StatusOptions};
use regex::Regex;
use clap::{ArgMatches, App, SubCommand};
use tico::tico;

pub fn shorten_path(cwd: &str) -> String {
  let friendly_path = match env::home_dir() {
    Some(path) => Regex::new(path.to_str().unwrap()).unwrap().replace(cwd, "~"),
    _ => return String::from("")
  };

  tico(&friendly_path)
}

pub fn repo_status(r: &Repository) -> bool {
  let mut opts = StatusOptions::new();
  opts.include_untracked(true);
  let head = match r.head() {
    Ok(head) => head,
    Err(_) => return false
  };

  let statuses = match r.statuses(Some(&mut opts)) {
    Ok(statuses) => statuses,
    Err(_) => return false
  };

  let mut is_dirty = false;

  for entry in statuses.iter().filter(
    |e| e.status() != git2::STATUS_CURRENT,
  ) {
    is_dirty = match entry.status() {
      s if s.contains(git2::STATUS_INDEX_NEW) => true,
      s if s.contains(git2::STATUS_INDEX_MODIFIED) => true,
      s if s.contains(git2::STATUS_INDEX_DELETED) => true,
      s if s.contains(git2::STATUS_INDEX_RENAMED) => true,
      s if s.contains(git2::STATUS_INDEX_TYPECHANGE) => true,
      s if s.contains(git2::STATUS_WT_NEW) => true,
      s if s.contains(git2::STATUS_WT_MODIFIED) => true,
      s if s.contains(git2::STATUS_WT_DELETED) => true,
      s if s.contains(git2::STATUS_WT_RENAMED) => true,
      s if s.contains(git2::STATUS_WT_TYPECHANGE) => true,
      _ => false,
    };

    if is_dirty { break }
  }
  return is_dirty
}


pub fn cli_arguments<'a>() -> App<'a, 'a> {
  SubCommand::with_name("precmd")
}
