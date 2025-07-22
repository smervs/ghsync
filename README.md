# GHSync

A command-line tool for synchronizing files between GitHub repositories. GHSync allows you to create sync pairs between local Git repositories and synchronize changes bidirectionally with proper Git workflow management.

## Features

- **Bidirectional sync**: Sync files from repository A to B or B to A
- **Git workflow integration**: Automatically handles branch creation, commits, and pushes
- **Configuration management**: Save and manage multiple sync configurations
- **Safe operations**: Confirmation prompts and validation checks
- **Branch management**: Automatic checkout and pull operations

## Installation

### Prerequisites

- Rust 1.70+ (edition 2024)
- Git
- `rsync` (for file synchronization)

### Build from source

```bash
git clone <repository-url>
cd ghsync
cargo build --release
```

The binary will be available at `target/release/ghsync`.

## Usage

### Adding a sync configuration

Create a new sync pair between two local Git repositories:

```bash
ghsync add --name <config-name> --source <path-to-repo-a> --destination <path-to-repo-b> --branch <base-branch>
```

Example:
```bash
ghsync add --name my-sync --source /path/to/repo-a --destination /path/to/repo-b --branch main
```

### Listing configurations

View all saved sync configurations:

```bash
ghsync list
```

### Synchronizing repositories

Sync files between configured repositories:

```bash
ghsync sync <config-name> --message "Sync commit message" [OPTIONS]
```

Options:
- `--direction <DIRECTION>`: Sync direction (`a2b` or `b2a`, default: `a2b`)
- `--branch <BRANCH>`: Target branch (default: `main`)

Examples:
```bash
# Sync from A to B on main branch
ghsync sync my-sync --message "Update from repo A"

# Sync from B to A on feature branch
ghsync sync my-sync --direction b2a --branch feature-branch --message "Backport changes"
```

### Removing a configuration

Delete a sync configuration:

```bash
ghsync remove --name <config-name>
```

## How it works

1. **Validation**: Verifies that both source and destination are valid Git repositories
2. **Fetch**: Fetches latest changes from remote origins
3. **Branch management**: Checks out base branch and target branch (creates if needed)
4. **File sync**: Uses `rsync` to synchronize files (excludes hidden files/directories)
5. **Git operations**: Commits changes and pushes to remote

## Configuration

Configurations are stored in JSON format at:
- `$XDG_CONFIG_HOME/ghsync/config.json` (Linux/macOS with XDG)
- `~/.config/ghsync/config.json` (fallback)

## Commands Reference

| Command | Description |
|---------|-------------|
| `add` | Add a new sync configuration |
| `list` | List all sync configurations |
| `remove` | Remove a sync configuration |
| `sync` | Perform synchronization |

## Safety Features

- **Confirmation prompts**: Asks for confirmation before performing sync operations
- **Repository validation**: Ensures paths are valid Git repositories
- **Change detection**: Only commits and pushes when there are actual changes
- **Branch protection**: Creates new branches safely without overwriting existing work

## Dependencies

- `clap`: Command-line argument parsing
- `colored`: Terminal color output
- `serde` + `serde_json`: Configuration serialization

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.