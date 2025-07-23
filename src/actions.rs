use crate::config::{Config, CONFIG_FILE_NAME, DEFAULT_DOTFILES_DIR};
use anyhow::{bail, Context, Result};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

fn get_home_dir() -> Result<PathBuf> {
    home::home_dir().context("Could not find home directory.")
}

fn resolve_path(path: &Path) -> Result<PathBuf> {
    if path.starts_with("~") {
        let home = get_home_dir()?;
        let relative_path = path.strip_prefix("~")?;
        return Ok(home.join(relative_path));
    }
    Ok(path.to_path_buf())
}

pub fn init(dir: Option<PathBuf>) -> Result<()> {
    let home_dir = get_home_dir()?;
    let dotfiles_dir = dir.unwrap_or_else(|| home_dir.join(DEFAULT_DOTFILES_DIR));

    if dotfiles_dir.exists() {
        println!(
            "{} Directory '{}' already exists. Nothing to do.",
            "Warning:".yellow(),
            dotfiles_dir.display()
        );
        return Ok(());
    }

    fs::create_dir_all(&dotfiles_dir)
        .with_context(|| format!("Failed to create dotfiles directory at {}", dotfiles_dir.display()))?;

    let config = Config {
        dotfiles_dir: dotfiles_dir.clone(),
        ..Default::default()
    };
    config.save()?;

    println!(
        "{} Initialized dotman repository at '{}'.",
        "Success:".green(),
        dotfiles_dir.display()
    );
    println!("You can now add files using `dotman add <path-to-file>`.");
    Ok(())
}

pub fn add(path_to_add: PathBuf) -> Result<()> {
    let home_dir = get_home_dir()?;
    let dotfiles_dir = home_dir.join(DEFAULT_DOTFILES_DIR);
    if !dotfiles_dir.exists() {
        bail!("Dotfiles directory not found. Please run `dotman init` first.");
    }
    let mut config = Config::load(&dotfiles_dir)?;

    let source_path = resolve_path(&path_to_add)?;
    if !source_path.exists() {
        bail!("File not found at '{}'", source_path.display());
    }

    let file_name = source_path
        .file_name()
        .context("Could not get file name.")?
        .to_str()
        .context("Invalid file name format.")?;

    let dest_path_in_repo = dotfiles_dir.join(file_name);

    println!("Moving '{}' to '{}'...", source_path.display(), dest_path_in_repo.display());
    fs::rename(&source_path, &dest_path_in_repo)
        .with_context(|| format!("Failed to move file to dotfiles directory."))?;

    create_symlink(&dest_path_in_repo, &source_path, true)?;

    config.files.insert(file_name.to_string(), source_path.clone());
    config.save()?;

    println!("{} Successfully added and linked '{}'.", "Success:".green(), file_name);
    Ok(())
}

pub fn link(force: bool) -> Result<()> {
    let home_dir = get_home_dir()?;
    let dotfiles_dir = home_dir.join(DEFAULT_DOTFILES_DIR);
    if !dotfiles_dir.exists() {
        bail!("Dotfiles directory not found. Please run `dotman init` first.");
    }
    let config = Config::load(&dotfiles_dir)?;

    if config.files.is_empty() {
        println!("{}", "No files to link. Add some with `dotman add`.".yellow());
        return Ok(());
    }

    println!("Linking all managed dotfiles...");
    for (file_name, target_path) in &config.files {
        let source_path_in_repo = dotfiles_dir.join(file_name);
        create_symlink(&source_path_in_repo, target_path, force)?;
    }
    println!("{}", "All files linked successfully.".green());
    Ok(())
}

pub fn unlink() -> Result<()> {
    let home_dir = get_home_dir()?;
    let dotfiles_dir = home_dir.join(DEFAULT_DOTFILES_DIR);
    if !dotfiles_dir.exists() {
        bail!("Dotfiles directory not found.");
    }
    let config = Config::load(&dotfiles_dir)?;

    if config.files.is_empty() {
        println!("{}", "No files to unlink.".yellow());
        return Ok(());
    }

    println!("Unlinking all managed dotfiles...");
    for (_, target_path) in &config.files {
        if target_path.exists() && target_path.is_symlink() {
            fs::remove_file(target_path)
                .with_context(|| format!("Failed to remove symlink at {}", target_path.display()))?;
            println!("Removed symlink '{}'", target_path.display());
        } else {
            println!("{} Skipped '{}' (not a symlink or does not exist).", "Warning:".yellow(), target_path.display());
        }
    }
    println!("{}", "All symlinks removed.".green());
    Ok(())
}

pub fn list() -> Result<()> {
    let home_dir = get_home_dir()?;
    let dotfiles_dir = home_dir.join(DEFAULT_DOTFILES_DIR);
    if !dotfiles_dir.exists() {
        bail!("Dotfiles directory not found. Please run `dotman init` first.");
    }
    let config = Config::load(&dotfiles_dir)?;

    if config.files.is_empty() {
        println!("No files are currently managed by dotman.");
        return Ok(());
    }

    println!("{}", "Managed dotfiles:".bold());
    for (file_name, target_path) in &config.files {
        let source_path = config.dotfiles_dir.join(file_name);
        println!("  {} -> {}", source_path.display(), target_path.display());
    }
    Ok(())
}

#[cfg(unix)]
fn create_symlink(source: &Path, dest: &Path, force: bool) -> Result<()> {
    use std::os::unix::fs::symlink;
    if dest.exists() || dest.is_symlink() {
        if force {
            println!("{} '{}' already exists. Overwriting due to --force flag.", "Warning:".yellow(), dest.display());
            fs::remove_file(dest)?;
        } else {
            println!("{} Skipped '{}' (already exists). Use --force to overwrite.", "Warning:".yellow(), dest.display());
            return Ok(());
        }
    }
    symlink(source, dest).with_context(|| format!("Failed to create symlink from {} to {}", source.display(), dest.display()))?;
    println!("Linked '{}' -> '{}'", dest.display(), source.display());
    Ok(())
}

#[cfg(windows)]
fn create_symlink(source: &Path, dest: &Path, force: bool) -> Result<()> {
    use std::os::windows::fs::symlink_file;
    if dest.exists() || dest.is_symlink() {
        if force {
            println!("{} '{}' already exists. Overwriting due to --force flag.", "Warning:".yellow(), dest.display());
            fs::remove_file(dest)?;
        } else {
            println!("{} Skipped '{}' (already exists). Use --force to overwrite.", "Warning:".yellow(), dest.display());
            return Ok(());
        }
    }
    symlink_file(source, dest).with_context(|| format!("Failed to create symlink from {} to {}", source.display(), dest.display()))?;
    println!("Linked '{}' -> '{}'", dest.display(), source.display());
    Ok(())
}