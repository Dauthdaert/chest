<div class="oranda-hide">

# Chest

</div>

Chest is a multi-platform CLI Command organizer inspired by hoard and built in Rust.

It allows you to save commands that are too complicated or long to remember.
When you add a command to chest, it remembers:
- a name for the command
- the command
- a description you provide

Next time you need a command, search for it using chest's search feature to quickly find it again.

<a name="toc" id="toc"/>

## [Table of contents](#)

<div class="oranda-hide" style="margin-bottom: -12px">

- [Install](#install)

</div>

- [Shell integration](#shell-integration)
- [Usage](#usage)
- [Keybindings](#keybindings)

<a name="install" id="install"/>

<div class="oranda-hide">

## [Install](#toc)
Current release: 0.2.0

### Using crates.io
Set up a Rust toolchain using [rustup](https://rustup.rs/), then run the following command.
```
cargo install chest-rs --locked --profile=dist
```

### Using cargo-binstall
Install [cargo-binstall](https://crates.io/crates/cargo-binstall), then run the following command.
```
cargo binstall chest-rs
```

### Using Github Releases
Archive formats and script installers are available in [Github Releases](https://github.com/Dauthdaert/chest/releases). Make sure the installed executable is in your PATH.

</div>

<a name="shell-integration" id="shell-integration"/>

## [Shell Integration](#toc)
Install `chest` as a plugin to enable deeper shell integration.

#### Nushell
Run the following command.
```
chest init nu | save ~/init-chest.nu
```
Then add the following to your Nushell config file.
```
source ~/init-chest.nu

#### Zsh
Add the following to your zsh profile.

source chest init zsh

#### PowerShell
Add the following to your PowerShell profile.

Invoke-Expression (& { (chest init powershell | Out-String) })
```
<a name="usage" id="usage"/>

## [Usage](#toc)
### Add
In order to add a command to your chest database, use the following command.
```
chest add
```
You will be prompted to fill in the necessary information.

### Update
In order to update a command in your chest database, use the following command.
```
chest update
```
You will be prompted to fill in the updated information.

### Remove/Delete
In order to remove a command from your chest database, use the following command.
```
chest remove
```
You will be prompted for the name of the command to remove.

A delete alias is also provided.
```
chest delete
```

### Search
Once a command has been added to your chest, there are two ways to search for it.
#### Non-interactive search
Using the following command, the top 5 commands that match the query are returned.
```
chest search <QUERY>
```
#### Interactive search
Using the following command, a TUI opens that allows for interactive searching.
```
chest search --interactive
```
Optionally, a query can be added in order to populate the search box with an initial search term.
```
chest search --interactive <QUERY>
```

Finally, if chest is installed as a **shell plugin**, interactive search is available through a keybinding (default Ctrl-h).
Additionally, if chest is invoked through the keybinding, confirming a selection adds the selected command to your next prompt.

### Reset
If you get a database error after an update, it may be necessary to reset your saved commands. You can do so using the following command.
```
chest reset
```

<a name="keybindings" id="keybindings"/>

## [Keybindings](#toc)
#### Open from shell
```
<Ctrl-h>
```
#### Next or previous command in list
```
<Up-arrow> or <Down-arrow>
```
#### Select a command
```
<Enter>
```
#### Cancel selection and close chest
```
<Esc> or <Ctrl-c>
```
