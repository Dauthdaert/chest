# source this in your ~/.zshrc
_chest_search() {
  emulate -L zsh
  zle -I

  local output
  output=$(CHEST_SHELL_ZSH=y chest search $* -i)

  zle reset-prompt

  if [[ -n $output ]]; then
    RBUFFER=""
    LBUFFER=$output

    if [[ $LBUFFER == __chest_accept__:* ]]
    then
      LBUFFER=${LBUFFER#__chest_accept__:}
      zle accept-line
    fi
  fi

}

zle -N chest_search _chest_search

bindkey -M emacs '^h' chest_search
bindkey -M vicmd '^h' chest_search
bindkey -M viins '^h' chest_search
