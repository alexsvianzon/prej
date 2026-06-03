#!/usr/bin/env zsh

prej() {
  if sudo -nv 2>/dev/null; then
    echo "sudo privileges are available"
  else
    echo "prej requires sudo privileges to run. please input your password:"
    sudo -v
  fi

  echo "running prej"
  sudo ./target/debug/prej "$@"
}

