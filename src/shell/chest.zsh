# source this in your ~/.zshrc
_chest_search() {
  emulate -L zsh
  zle -I

  local output
  output=$(CHEST_SHELL_ZSH=y chest search $* -i -- $BUFFER)

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

zle -N _chest_search_widget _chest_search

bindkey -M emacs '^h' _chest_search_widget
bindkey -M vicmd '^h' _chest_search_widget
bindkey -M viins '^h' _chest_search_widget
