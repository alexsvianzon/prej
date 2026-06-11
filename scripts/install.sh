#!/bin/sh

$os="$(uname -s)"
$ver="0.8.0"
$source="https://raw.githubusercontent.com/alexsvianzon/prej/refs/tags/$ver"

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
    sudo curl -L -o /usr/local/bin/prej.zsh "$source/scripts/prej.zsh"
    sudo curl -L -o /usr/local/bin/prej "$source/build/prej"
    sudo chmod +x /usr/local/bin/prej

    if ! grep -q "# --> prej setup -->" ~/.zshrc; then
        cat <<EOF >> ~/.zshrc

# --> prej setup -->
source /usr/local/bin/prej.zsh
# <-- prej setup <--
EOF
    fi

    ;;
  *bash)
    sudo curl -L -o /usr/local/bin/prej.bash "$source/scripts/prej.bash"
    sudo curl -L -o /usr/local/bin/prej "$source/build/prej"
    sudo chmod +x /usr/local/bin/prej

    if ! grep -q "# --> prej setup -->" ~/.bashrc; then
      cat <<EOF >> ~/.zshrc

# --> prej setup -->
source /usr/local/bin/prej.bash
# <-- prej setup <--
EOF
    fi

    ;;
  *)
    echo "unsupported shell"
    exit 1

    ;;
esac


    
