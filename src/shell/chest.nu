# Source this in your ~/.config/nushell/config.nu

def _chest_search_cmd [... flags: string] {
  ([
    `commandline (run-external --redirect-stderr chest search`,
    ($flags | append [--interactive, --] | each {|e| $'"($e)"'}),
    `(commandline) | complete | $in.stderr | str substring ..-1)`,
  ] | flatten | str join ' ')
}

$env.config = (
  $env.config | upsert keybindings (
    $env.config.keybindings
    | append {
      name: chest
      modifier: control
      keycode: char_h
      mode: [emacs, vi_normal, vi_insert]
      event: { send: executehostcommand cmd: (_chest_search_cmd) }
    }
  )
)