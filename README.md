# chest
chest is a multi-platform CLI Command organizer inspired by hoard.

It allows you to save commands that are too complicated or long to remember.
When you add a command to chest, it remembers:
- the text of the command
- a description you provide

Next time you need a command, search for it using chest's search feature to quickly find it again.

## Install
### From source
Set up a Rust toolchain using [rustup](https://rustup.rs/), clone the repository, then run:
```
cargo install --path .
```

## Autocomplete with Shell plugin
Install `chest` as a plugin to enable deeper shell integration.

#### Nushell
Run the following command.
```
chest init nu | save ~/init-chest.nu
```
Then add the following to your Nushell config file.
```
source ~/init-chest.nu
```

## Usage
### Add
In order to add a command to your chest database, use the following command.
```
chest add <COMMAND_TEXT> <COMMAND_DESCRIPTION>
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
Optionnally, a query can be added in order to populate the search box with an initial search term.
```
chest search --interactive <QUERY>
```

Finally, if chest is installed as a **shell plugin**, interactive search is available through a keybinding (default Ctrl-h).
Additionaly, if chest is invoked through the keybinding, confirming a selection adds the selected command to your next prompt.


## Keybindings
#### Open from shell
```
<Ctrl-h>
```
#### Edit search query
```
<s>
```
#### Return to list
```
<Esc>
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
