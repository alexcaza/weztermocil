# Introduction
Weztermocil allows you to setup pre-configured layouts of windows and panes in [Wezterm](https://wezfurlong.org/wezterm/index.html), having each open in a specified directory and execute specified commands. You do this by writing YAML files to save your layouts. 

This project was inspired by [Teamocil](https://github.com/remi/teamocil) and [iTermocil](https://github.com/TomAnthony/itermocil), and was designed to be able to use your existing Teamocil and iTermocil configuration files.

**Weztermocil is still very early days, and things may break!**

## Installation

TODO!

## Homebrew

## Nix

## Usage

```bash
$ weztermocil [options] [layout-name]
```

### Global options

| Option      | Description
|-------------|----------------------------
| `--list`    | Lists all available layouts in `~/.weztermocil`
| `--help`    | Show all the options available to you

### Layout options

| Option      | Description
|-------------|----------------------------
| `--layout`  | Takes a custom file path to a YAML layout file instead of `[layout-name]`
| `--here`    | Uses the current window as the layout’s first window
| `--edit`    | Opens the layout file with `$EDITOR` instead of executing it
| `--show`    | Shows the layout content instead of executing it

## YAML Options

### Session

| Key       | Description
|-----------|----------------------------
| `windows` | An `Array` of windows/tabs

### Windows

| Key       | Description
|-----------|----------------------------
| `name`    | The window/tab name
| `root`    | The path where all panes in the window will be started
| `layout`  | The layout that will be used by Weztermocil
| `panes`   | An `Array` of panes
| `focus`   | If set to `true`, the window will be selected after the layout has been executed

### Panes

A pane can either be a `String` or a `Hash`. If it’s a `String`, Weztermocil will
treat it as a single-command pane.

| Key        | Description
|------------|----------------------------
| `commands` | An `Array` of commands that will be ran when the pane is created
| `focus`    | If set to `true`, the pane will be selected after the layout has been executed

## Examples

### Simple two pane window

```yaml
windows:
  - name: sample-two-panes
    root: ~/Code/sample/www
    layout: even-horizontal
    panes:
      - git status
      - rails server
```

```
.------------------.------------------.
| (0)              | (1)              |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
'------------------'------------------'
```

### Simple three pane window

```yaml
windows:
  - name: sample-three-panes
    root: ~/Code/sample/www
    layout: main-vertical
    panes:
      - vim
      - commands:
        - git pull
        - git status
      - rails server
```

```
.------------------.------------------.
| (0)              | (1)              |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |------------------|
|                  | (2)              |
|                  |                  |
|                  |                  |
|                  |                  |
'------------------'------------------'
```

### Simple four pane window

```yaml
windows:
  - name: sample-four-panes
    root: ~/Code/sample/www
    layout: tiled
    panes:
      - vim
      - foreman start web
      - git status
      - foreman start worker
```

```
.------------------.------------------.
| (0)              | (1)              |
|                  |                  |
|                  |                  |
|                  |                  |
|------------------|------------------|
| (2)              | (3)              |
|                  |                  |
|                  |                  |
|                  |                  |
'------------------'------------------'
```

### Two pane window with focus in second pane

```yaml
windows:
  - name: sample-two-panes
    root: ~/Code/sample/www
    layout: even-horizontal
    panes:
      - rails server
      - commands:
          - rails console
        focus: true
```

```
.------------------.------------------.
| (0)              | (1) <focus here> |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
|                  |                  |
'------------------'------------------'
```

## Extras
### Zsh autocompletion

To get autocompletion when typing `weztermocil <Tab>` in a zsh session, add this line to your `~/.zshrc` file:

```zsh
compctl -g '~/.weztermocil/*(:t:r)' weztermocil
```

[zsh-completions](https://github.com/zsh-users/zsh-completions) also provides
additional completion definitions for Teamocil.

### Bash autocompletion

To get autocompletion when typing `weztermocil <Tab>` in a bash session, add this line to your `~/.bashrc` file:

```bash
complete -W "$(weztermocil --list)" weztermocil
```

### Fish autocompletion

To get autocompletion when typing `weztermocil <Tab>` in a fish session,
add the following file `~/.config/fish/completions/weztermocil.fish` with
the following content:

```fish
complete -x -c weztermocil -a '(weztermocil --list)'
```

# Thanks
This was heavily inspired by [Teamocil](https://github.com/remi/teamocil) and [iTermocil](https://github.com/TomAnthony/itermocil). Without these 2 projects, I wouldn't have decided to make the same thing for Wezterm.

# License
Weztermocil is © 2024 [Alex Caza](https://alexcaza.com) and may be freely
distributed under the [MIT license](https://github.com/alexcaza/weztermocil/blob/master/LICENSE.md).
See the `LICENSE.md` file for more information.
