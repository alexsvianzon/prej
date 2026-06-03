#!/usr/bin/env bash

prej() {
  if sudo -nv 2>/dev/null; then
    echo "sudo privileges are available"
  else
    echo "prej requires sudo privileges to run. please input your password:"
    sudo -v
  fi

  case "$1" in
    go)
      sudo $HOME/Dev/Projects/pm/target/debug/prej go "$2"
      cd "$(sudo $HOME/Dev/Projects/pm/target/debug/prej dir "$2")"

      ;;
    *)
      sudo $HOME/Dev/Projects/pm/target/debug/prej "$@"

      ;;
  esac
}

