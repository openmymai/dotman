````markdown
<div align="center">
  <a href="https://www.buymeacoffee.com/maicmi" target="_blank">
    <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me a Coffee" height="45">
  </a>
  <br/>
  <small>If you find dotman useful, consider supporting its development!</small>
</div>

<br/>

# dotman ⚙️

**dotman** is a simple, fast, and reliable dotfiles manager written in Rust. It helps you keep your configuration files (dotfiles) synchronized across multiple machines by storing them in a single directory, which you can then track with Git.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![GitHub Actions](https://img.shields.io/github/actions/workflow/status/your-username/dotman/rust.yml?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/dotman?style=for-the-badge)

---

## Why dotman?

- **Simple Command Structure**: Easy-to-understand commands for initializing, adding, and linking files. No complex configuration needed.
- **Fast and Safe**: Built with Rust for blazing-fast performance and memory safety, ensuring it won't mess up your precious configs.
- **Cross-Platform**: Works seamlessly on Linux, macOS, and Windows.
- **Configuration-driven**: Uses a simple, human-readable `dotfiles.toml` file to track your managed files.
- **Non-destructive**: Warns before overwriting existing files, with a `--force` option available for when you know what you're doing.

---

## Installation

You can install `dotman` directly from the source code or from Crates.io.

### From Crates.io (Recommended)

Once the project is published, you can install it with a single command:

```bash
cargo install dotman
```
````

```

```

### From Source

1.  **Prerequisites**:

    - [Rust Toolchain](https://www.rust-lang.org/tools/install)
    - [Git](https://git-scm.com/)

2.  **Clone and Install**:
    ```bash
    git clone https://github.com/openmymai/dotman.git
    cd dotman
    cargo install --path .
    ```
    This will compile and install the `dotman` binary into your Cargo bin path (`~/.cargo/bin`), making it available from anywhere in your terminal.

---

## Usage Workflow

Here's a typical workflow for using `dotman`.

### 1. Initialize `dotman`

First, create a directory where you want to store your dotfiles. This is a one-time setup.

```bash
# This will create a '~/dotfiles' directory by default
dotman init

# Or specify a custom directory
dotman init --dir ~/my-configs
```

This creates the directory and a `dotfiles.toml` configuration file inside it. You should `cd` into this directory and initialize a Git repository to start tracking your files.

```bash
cd ~/dotfiles
git init
git add .
git commit -m "Initial commit: Setup dotman"
```

### 2. Add Files

Now, you can start adding your existing dotfiles to be managed by `dotman`.

For example, to add your `.zshrc`:

```bash
dotman add ~/.zshrc
```

This command will:

1.  **Move** `~/.zshrc` into your `~/dotfiles` directory.
2.  **Create a symbolic link** from `~/.zshrc` back to `~/dotfiles/.zshrc`.
3.  **Update** `dotfiles.toml` to track this new file.

Now you can commit the change to Git:

```bash
git add .
git commit -m "Add .zshrc"
```

### 3. Link Files on a New Machine

After cloning your dotfiles repository onto a new machine, you just need to run one command to set everything up:

```bash
# Make sure you are in your dotfiles repository directory
cd ~/dotfiles

# Create all the symlinks
dotman link
```

This will read your `dotfiles.toml` and create symlinks for all your managed files. If a file already exists at the target location, `dotman` will skip it. Use `--force` to overwrite.

````bash
dotman link --force```

### All Commands

-   `init`: Initializes the dotfiles repository.
-   `add <PATH>`: Adds a new file to be managed.
-   `link [--force]`: Creates symlinks for all tracked dotfiles.
-   `unlink`: Removes all managed symlinks.
-   `list`: Lists all dotfiles currently managed.

---

## How It Works

`dotman` maintains a central configuration file named `dotfiles.toml` inside your chosen dotfiles directory. This file keeps a record of which files are being managed and where their symlinks should point.

A typical `dotfiles.toml` looks like this:

```toml
# The root directory where your managed files are stored.
dotfiles_dir = "/home/user/dotfiles"

# A map of [filename-in-repo] -> [target-symlink-path]
[files]
".zshrc" = "/home/user/.zshrc"
".gitconfig" = "/home/user/.gitconfig"
````

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/openmymai/dotman/issues).

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

```

```
