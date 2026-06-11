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
      cd "$(sudo /usr/local/bin/prej dir "$2")"
      sudo /usr/local/bin/prej go "$2"

      ;;
    run)
      cd "$(sudo /usr/local/bin/prej dir "$(sudo /usr/local/bin/prej active)")"
      sudo /usr/local/bin/prej run "$2"

      ;;
    close)
      cd "$(sudo /usr/local/bin/prej dir "$(sudo /usr/local/bin/prej active)")"
      sudo /usr/local/bin/prej close

      ;;
    *)
      sudo /usr/local/bin/prej "$@"

      ;;
  esac
}

