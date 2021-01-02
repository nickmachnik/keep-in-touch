#!/usr/bin/bash

_names() {
    local namepos="${COMP_WORDS[COMP_CWORD]}"
    COMPREPLY=($(compgen -W "KIT_SAVED_NAMES" -- "$namepos"))
}

_main() {
  if [[ "$COMP_CWORD" -eq 1 ]]
  then
    local cur="${COMP_WORDS[COMP_CWORD]}"
    COMPREPLY=($(compgen -W "add help modify remove view just-talked-to view-active view-inactive suspend resume -h -V" "$cur"))
    return
  fi

  local cmd="${COMP_WORDS[1]}"

  # we've completed the 'current' command and now need to call the next completion function
  # subcommands have their own completion functions
  case "$cmd" in
    remove) _remove ;;
    modify) _modify ;;
    suspend) _remove ;;
    resume) _remove ;;
    just-talked-to) _just_talked_to ;;
    *)          ;;
  esac
}

_just_talked_to() {
  # kit just-talked-to <name>
  if [[ "$COMP_CWORD" -eq 2 ]]
  then
    _names
    return
  fi
}

_remove() {
  # kit remove <name>
  if [[ "$COMP_CWORD" -eq 2 ]]
  then
    _names
    return
  fi
}

_modify() {
  # kit modify <name> <field> <new value>
  if [[ "$COMP_CWORD" -eq 2 ]]
  then
    _names
    return
  fi

  if [[ "$COMP_CWORD" -eq 3 ]]
  then
    local cur="${COMP_WORDS[COMP_CWORD]}"
    COMPREPLY=($(compgen -W "name interval last" "$cur"))
    return
  fi
}

complete -F _main kit