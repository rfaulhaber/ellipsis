# ellipsis: yet another dotfile manager

A host-driven, task-driven dotfile manager.

## Usage

Ellipsis expects to be run in the same directory as your Ellipsis configuration file, which should also live with your dotfiles.

### Configuration file

Your ellipsis configuration file is the heart of your dotfile task management. It can be either JSON or YAML. It should be named `ellipsis.{yaml,yml,json}` or should be specified with the global flag `--config`.

Here is an example in YAML:

```yaml
vars:
  dotfile_root: ~/projects/dotfiles

hosts:
  my-macbook:
    tasks:
      # this references the task under the "tasks" heading
      - install_hammerspoon
      - name: install_homebrew # names are optional but required if you want to run with "exec"
        exec: /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    links:
      - from: "{{dotfile_root}}/config"
        to: ~/.config

# these are global tasks that are only here to be referenced in the tasks heading
tasks:
  install_hammerspoon:
    exec: |
      brew install hammerspoon
      mkdir ~/.hammerspoon
```

### Install

`install` runs both the tasks and links under a host.

```sh
ellipsis install my-macbook
```

### Link

Creates symlinks for all `link` entries.

```sh
ellipsis link my-macbook

```

### Exec

Executes particular tasks for a host.

```sh
ellipsis exec my-macbook install_hammerspoon
```
