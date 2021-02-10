# Punto - Another dotfiles manager

## Relevant features

* With version control in mind
* No symlinks to manage the dotfiles, all copy from/to the repo/system
* All actions performed with the same cli app
* Heavily inspired on [dotbot](https://github.com/anishathalye/dotbot)
* All actions that can be performed will be described in `yaml` config files that punto reads and executes
* When usable, this project will be used in [my dotfiles](https://github.com/sergioquijanorey/dotfiles)

## Actions that I want to implement -- All of them described in yaml files

* Create a directory structure
* Sync dotfiles from repo to system
* Sync dotfiles from system to repo
* Install packages
    * With different package managers in mind
* Execute custom shell scripts
* Test that your dotfiles can be installed inside an isolated container

## Examples

### `shell.yaml`

Run `punto --shell`

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

Run `punto --install`

~~~yaml
common:
    install_command: sudo pacman -S --noconfirm
    packages:
        - git
        - htop
pacman:
    install_command: sudo pacman -S --noconfirm
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
    packages:
        - bat         # A better cat alternative
        - bottom      # A better top alternative

        # Others
        #===============================================================================
        - spotify
~~~

### `directories.yaml`

Run `punto --download` or `punto --upload`

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
