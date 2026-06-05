#!/usr/bin/env zsh

prej() {
  if sudo -nv 2>/dev/null; then
    echo "sudo privileges are available"
  else
    echo "prej requires sudo privileges to run. please input your password:"
    sudo -v
  fi

  case "$1" in
    go)
      sudo /usr/local/bin/prej go "$2"
      cd "$(sudo /usr/local/bin/prej dir "$2")"

      ;;
    *)
      sudo $HOME/Dev/Projects/pm/target/debug/prej "$@"

      ;;
  esac
}

