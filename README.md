# Introduction
Weztermocil allows you to setup pre-configured layouts of windows and panes in [Wezterm](https://wezfurlong.org/wezterm/index.html), having each open in a specified directory and execute specified commands. You do this by writing YAML files to save your layouts. 

This project was inspired by [Teamocil](https://github.com/remi/teamocil) and [iTermocil](https://github.com/TomAnthony/itermocil), and was designed to be able to use your existing Teamocil and iTermocil configuration files.

https://github.com/alexcaza/weztermocil/assets/6993273/c4e009ee-dcff-46b2-87f3-a4f0d476830f




**Weztermocil is still very early days, and things may break!**

## Installation

### Homebrew
```bash
# Install `weztermocil` via Homebrew
# This may take a while to complete as it's building from source
$ brew update
$ brew install alexcaza/weztermocil 
```

### Nix
The recommended way is to use an overlay within NixOS or home-manager.

If this package becomes stable, I might release it officially through nixpkgs.

```nix
(self: super: 
  let
    src = super.fetchFromGitHub {
      owner = "alexcaza";
      repo = "weztermocil";
      rev = "v0.1.4";
      hash = "sha256-9NWhGnLxZtHKHAUZ3Ha5NlMMZ7RZ0Irc50HyILX1MjA=";
    };
in {
    weztermocil = super.callPackage src {};
})
```

### Post-install
```shell
# Create your layout directory
$ mkdir ~/.weztermocil

# Open a new sample file with your editor of choice (look for sample layouts in this very `README.md`)
# There are also a variety of example files in the 'samples' directory of this repo
$ $EDITOR ~/.weztermocil/sample.yml

# Run your newly-created sample layout
$ weztermocil sample

# Note that you can also use ~/.teamocil or ~/.itermocil as your directory, if you're a teamocil/itermocil user.
```


## Usage

```shell
$ weztermocil [options] [layout-name]
```

Alternatively, if you have a `weztermocil.yml` file in the current directory you can simply run weztermocil and it will use that file, so you can have files inside your projects and sync via Github etc:

```shell
$ cd my_project
$ weztermocil
```

You can also add local layouts to a project by adding a `.weztermocil` directory. When calling `weztermocil --list`, you'll see the available layouts in the current directory listed as well.

To use a local layout, simply call it by name.
```shell
$ mkdir .weztermocil
$ touch .weztermocil/sample.yml
$ weztermocil sample
```

Weztermocil will follow this lookup path:
- No layout name supplied
  - look for weztermocil.yml file in current directory.
- Layout name supplied
  - Look for layout in local `.weztermocil` folder
  - Look for layout in global `~/.weztermocil` folder

### Global options

Weztermocil _should be_ compatible with all of teamocil and itermocil's flags, and they _should_ all work the same way.

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
