use std::env;

pub fn is_zsh() -> bool {
    // only set on zsh
    env::var("CHEST_SHELL_ZSH").is_ok()
}
