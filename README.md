# Punto - Another dotfiles manager

## Relevant features

* With version control in mind
* No symlinks to manage the dotfiles, all copy from/to the repo/system
* All actions performed with the same cli app
* Heavily inspired on [dotbot](https://github.com/anishathalye/dotbot)
* All actions that can be performed will be described in `yaml` config files that punto reads and executes
    * Therefore, you can create your own structures as you like, no fixed structure imposed to you
* This project is used on [my personal dotfiles](https://github.com/sergioquijanorey/dotfiles)
* **Not stable at the moment**. See the TODO list at the end.

## Actions that you can perform with punto

* Create a directory structure
* Sync dotfiles from repo to system
* Sync dotfiles from system to repo
* Install packages
    * With different package managers in mind
* Execute custom shell scripts

## Actions that you might be able to do in the future

* Test that your dotfiles can be installed inside an isolated container

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
    -d, --download <yaml_file>    Syncs files and dirs from repo to your system
    -i, --install <yaml_file>     Installs packages from yaml file
    -s, --shell <yaml_file>       Launchs shell commands from yaml file
    -u, --upload <yaml_file>      Syncs files and dirs from your system to repo
~~~

## Examples

### `shell.yaml`

Run `punto --shell shell.yaml`

~~~yaml
list_dir:
    description: List this project using exa
    quiet: false
    commands:
        - exa -T /home/sergio/GitProjects/punto/src/
    sudo: false

cat some file:
    description: Cat some file
    commands:
        - cat /home/sergio/.bashrc

bat some file:
    description: Cat with bat
    commands:
        - bat /home/sergio/.bashrc

htop which is interactive:
    commands:
        - htop

Git with colors:
    commands:
        - git log --oneline

More than one command:
    description: This is other proof of concept
    quiet: true
    commands:
        - echo "This is a command"
        - echo "This is other command"
        - echo "More commands yay"
    sudo: true
install other:
    description: This is other proof of concept
    quiet: true
    commands:
        - paru -S spotify
    sudo: true
~~~

### `packages.yaml`

Run `punto --install packages.yaml`

~~~yaml
common:
    install_command: pacman -S --noconfirm
    sudo: true
    packages:
        - git
        - htop
pacman:
    install_command: pacman -S --noconfirm
    sudo: true
    packages:
        - yay
        - sudo
        - base-devel
        - rsync
        - cronie
        - alacritty               # Preferred terminal
        - screen                  # For launching apps in the background
        - exa                     # Good replacement for ls and tree (exa -T)
        - fd                      # Good replacement for find
aur:
    install_command: paru -S --noconfirm
    sudo: false
    packages:
        - bat         # A better cat alternative
        - bottom      # A better top alternative

        # Others
        #===============================================================================
        - spotify
~~~

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

* See [issues](https://github.com/SergioQuijanoRey/punto/issues) for all bugs and feature requests
