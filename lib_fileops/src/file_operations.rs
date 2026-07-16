/// Module to implement basic file operations such as copy files, copy dirs,
/// create dirs, ...
use anyhow::Context;
use folder_compare::FolderCompare;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::sync_options::SyncOptions;

pub fn sync_dir(from: PathBuf, to: PathBuf, sync_options: SyncOptions) -> anyhow::Result<()> {
    let empty = Vec::new();
    let excludes: &[String] = sync_options
        .exclude_patterns()
        .map(|v| v.as_slice())
        .unwrap_or(&empty);

    fs::create_dir_all(&to).with_context(|| format!("Cannot create destination dir {:?}", to))?;

    let mut builder = GlobSetBuilder::new();
    for pattern in excludes {
        builder.add(
            Glob::new(pattern).with_context(|| format!("Invalid exclude pattern {:?}", pattern))?,
        );
    }
    let exclude_set = builder
        .build()
        .context("Could not build exclude pattern set")?;

    copy_dir_recursive(&from, &to, &from, &exclude_set)?;

    if sync_options.delete_files_destination() {
        remove_extra_files(&from, &to, &to)?;
    }

    Ok(())
}

fn copy_dir_recursive(
    root_from: &Path,
    root_to: &Path,
    current_from: &Path,
    excludes: &GlobSet,
) -> anyhow::Result<()> {
    for entry in
        fs::read_dir(current_from).with_context(|| format!("Cannot read dir {:?}", current_from))?
    {
        let entry = entry?;
        let path = entry.path();

        // Skip the destination tree if it lives inside the source tree
        if path.starts_with(root_to) {
            continue;
        }

        let relative = path
            .strip_prefix(root_from)
            .context("Could not strip prefix")?;
        let rel_str = relative.to_str().context("Non-UTF-8 path")?;

        if excludes.is_match(rel_str) {
            continue;
        }

        let dest = root_to.join(relative);
        if path.is_dir() {
            fs::create_dir_all(&dest).with_context(|| format!("Cannot create dir {:?}", dest))?;
            copy_dir_recursive(root_from, root_to, &path, excludes)?;
        } else {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&path, &dest)
                .with_context(|| format!("Cannot copy {:?} -> {:?}", path, dest))?;
        }
    }
    Ok(())
}

fn remove_extra_files(root_from: &Path, root_to: &Path, current_to: &Path) -> anyhow::Result<()> {
    for entry in
        fs::read_dir(current_to).with_context(|| format!("Cannot read dir {:?}", current_to))?
    {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(root_to).context("strip prefix")?;
        let source_counterpart = root_from.join(relative);

        if !source_counterpart.exists() {
            if path.is_dir() {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(&path)?;
            }
        } else if path.is_dir() {
            remove_extra_files(root_from, root_to, &path)?;
        }
    }
    Ok(())
}

/// Copies one file to another location
/// Creates the `to` folder if it does not exist
pub fn sync_file(from: &str, to: &str) -> anyhow::Result<()> {
    // Get the path to the parent dir of `to` file
    let parent_dir = Path::new(to)
        .parent()
        .with_context(|| {
            format!(
                "Could not get the path of the parent dir of dest. file {}",
                to
            )
        })?
        .to_str()
        .with_context(|| {
            format!(
                "Could not get the string of the parent dir of dest file {}",
                to
            )
        })?;

    // Create the dir for the new file
    fs::create_dir_all(parent_dir)
        .with_context(|| format!("Could not create dir {} to store new file", parent_dir))?;

    // Copy the file to the new dir
    fs::copy(from, to).context(format!("Failed to copy file from {} to {}", from, to))?;

    return Ok(());
}

/// Joins two paths given in strings
///
/// # Examples
/// ```
/// use lib_fileops::join_two_paths;
/// let joined = join_two_paths("first_part", "second_part");
/// let expected = "first_part/second_part";
/// assert_eq!(expected, joined, "Join two paths func did not work properly");
/// ```
pub fn join_two_paths(first: &str, second: &str) -> String {
    let second_sanitized = sanitize_relative_path(second);
    let joined_path = std::path::Path::new(first)
        .join(second_sanitized)
        .to_str()
        .unwrap()
        .to_string();
    return joined_path;
}

/// Relative paths that are stored in a string in the form of "./something" are
/// dangerous. Some functions fail when passing relative paths like that
/// So here we sanitize that relative paths
///
/// Also, joining two paths in the form of "/something" "/other" is problematic,
/// so we also handle that case
///
/// # Examples
///
/// ```
/// use lib_fileops::sanitize_relative_path;
/// let computed = sanitize_relative_path("./some/rel/path");
/// let expected = "some/rel/path";
/// assert_eq!(expected, computed, "Relative path sanitizer did not work well");
///
/// let computed = sanitize_relative_path("/some/rel/path");
/// let expected = "some/rel/path";
/// assert_eq!(expected, computed, "Relative path sanitizer did not work well");
/// ```
pub fn sanitize_relative_path(rel_path: &str) -> String {
    if &rel_path[0..1] == "/" {
        let sanitized: &str = &rel_path[1..rel_path.len()];
        return sanitized.to_string();
    }

    if &rel_path[0..2] == "./" {
        let sanitized: &str = &rel_path[2..rel_path.len()];
        return sanitized.to_string();
    }

    return rel_path.to_string();
}

/// `folder_compare::Error` does not implement `Error` trait, which is needed
/// for using anyhow. So this enum takes a `folder_compare::Error` and implements
/// the traits we need
#[derive(Error, Debug)]
enum DiffError {
    #[error("Error while computing the diff between two dirs, reason: {inner_error:?}")]
    DiffError { inner_error: folder_compare::Error },
}

/// Given two folders, defined by paths `first_path` and `second_path`, returns
/// the list of files that are present in the second dir but not present in the
/// first dir
pub fn get_dir_diff(first_path: &str, second_path: &str) -> anyhow::Result<Vec<String>> {
    let excluded = vec![];
    let new_files = FolderCompare::new(Path::new(second_path), Path::new(first_path), &excluded)
        // Use our custom error type so we can use anyhow
        .map_err(|inner| DiffError::DiffError { inner_error: inner })
        .context(format!(
            "An error ocurred while diffing {first_path} and {second_path}"
        ))?
        .new_files;

    // We want the strings out of the `PathBuf` objects
    let new_files: Vec<String> = new_files
        .iter()
        .map(|pathbuf| {
            pathbuf
                .to_str()
                .context("Could not convert pathbuf {pathbuf:?} to string")
        })
        .collect::<anyhow::Result<Vec<&str>>>()? // some paths could faild to be converted to `&str`
        .iter()
        .map(|path| path.to_string())
        .collect(); // `&str -> String`

    return Ok(new_files);
}
