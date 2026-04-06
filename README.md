# Punto - Another dotfiles manager

## Relevant features

Manage the bidirectional synchronization of your dotfiles between your git repository and your local system.

- With version control in mind.
- No symlinks to manage the dotfiles, all copy from/to the repo/system.
- Heavily inspired on [dotbot](https://github.com/anishathalye/dotbot)
- This project is used on [my personal dotfiles](https://github.com/sergioquijanorey/dotfiles)
- **Not stable at the moment**. See the TODO list at the end.

## Usage

~~~bash
punto -- dotfiles manager 0.1
Sergio Quijano <sergiquijano@gmail.com>
Another dotfiles manager

USAGE:
    punto [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --check <yaml_file>       Checks for dir sync problems. Searches for files deleted in a repo (or system) dir
                                  that are still present in their system (or repo) dir
    -d, --download <yaml_file>    Syncs files and dirs from repo to your system
    -u, --upload <yaml_file>      Syncs files and dirs from your system to repo
~~~

## Examples

### `directories.yaml`

Run `punto --download directories.yaml` or `punto --upload directories.yaml`

~~~yaml
# Where the dotfiles repo is located
repo_base: /home/sergio/punto/

directories:
    - file_descr:
        # Default sync type is file
        # Therefore, this field does not need to be specified
        sync_type: file
        repo_path: ./src/main.rs
        system_path: /home/sergio/pruebas.rs
    - dir_descr:
        sync_type: dir
        repo_path: ./src/
        system_path: /home/sergio/codigo_de_pruebas
~~~

# TODOs

- See [issues](https://github.com/SergioQuijanoRey/punto/issues) for all bugs and feature requests
