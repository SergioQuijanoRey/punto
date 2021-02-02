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
