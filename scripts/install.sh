#!/bin/sh

case $OSTYPE in
  darwin*)
    echo "found macOS"
    
    ;;
  *)
    echo "unsupported OS"
    exit 1

    ;;
esac

case $SHELL in
  *zsh)
    curl -o /usr/local/bin/prej.zsh https://raw.githubusercontent.com/alexsvianzon/prej/refs/heads/master/scripts/prej.zsh
    curl -o /usr/local/bin/prej https://raw.githubusercontent.com/alexsvianzon/prej/refs/heads/master/scripts/prej.zsh

    if ! grep -q "# --> prej setup -->" ~/.zshrc; then
        cat <<EOF >> ~/.zshrc

      # --> prej setup -->
      source /usr/local/bin/prej.zsh
      # <-- prej setup <--
      EOF
    fi

    ;;
  *)
    exit 1

    ;;
esac


    
