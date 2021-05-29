# ellipsis: yet another dotfile manager

A host-driven, task-driven dotfile manager.

Ellipsis can:
- Install dotfiles according to tasks
- Rollback changes
- Update changes
- Build configuration files using templates

## Usage

Ellipsis determines your computer's host name by running `hostname`. If you would like to override this behavior, you may specify the flag `--hostname`.

Ellipsis expects to be run in the same directory as your Ellipsis configuration file, which should also live with your dotfiles.

### Configuration file

Your ellipsis configuration file is the heart of your dotfile task management. It can be either JSON, YAML, or TOML. It should be named `ellipsis.{yaml,yml,json,toml}` or should be specified with the global flag `--config`.

Here is an example in YAML:

``` yaml
vars:
    dotfile_root: ~/projects/dotfiles

hosts:
    my-macbook:
        tasks:
            # this references the task under the "tasks" heading
            - link_config
            - name: Install homebrew 
              exec: /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        links:
            - from: "{{dotfile_root}}/config"
              to: ~/.config
        

# these are global tasks that are only here to be referenced in the tasks heading
tasks:
    install_hammerspoon:
        name: "Installs hammerspoon"
        exec: |
            brew install hammerspoon
            mkdir ~/.hammerspoon

# these are "global" links and will be run every time for every host
links:
    - from: "~/projects/dotfiles/config"
      to: "~/.config"
```

### Install

`install` runs all of the tasks under each `tasks` heading for a host.

``` sh
ellipsis install
```

### Revert
