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

sudo mkdir -p /usr/local/bin

case $SHELL in
  *zsh)
    sudo curl -o /usr/local/bin/prej.zsh https://raw.githubusercontent.com/alexsvianzon/prej/refs/heads/beta/scripts/prej.zsh
    sudo curl -o /usr/local/bin/prej https://raw.githubusercontent.com/alexsvianzon/prej/refs/heads/beta/build/prej

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


    
