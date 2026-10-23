#!/bin/sh

os="$(uname -s)"
ver="0.8.0"
src="https://raw.githubusercontent.com/alexsvianzon/prej/refs/tags/$ver"

case $os in
  Darwin*)
    echo "found macOS"
    
    ;;
  *)
    echo "unsupported OS"
    exit 1

    ;;
esac

sudo mkdir -p /usr/local/bin
sudo curl -Lo /usr/local/bin/prej "$src/build/prej"
sudo chmod +x /usr/local/bin/prej

case $SHELL in
  *zsh)
    sudo curl -L -o /usr/local/bin/prej.zsh "$src/scripts/prej.zsh"

    if ! grep -q "# --> prej setup -->" ~/.zshrc; then
        cat <<EOF >> ~/.zshrc

# --> prej setup -->
source /usr/local/bin/prej.zsh
# <-- prej setup <--
EOF
    fi

    ;;
  *bash)
    sudo curl -L -o /usr/local/bin/prej.bash "$src/scripts/prej.bash"

    if ! grep -q "# --> prej setup -->" ~/.bash_profile; then
      cat <<EOF >> ~/.bash_profile

# --> prej setup -->
source /usr/local/bin/prej.bash
# <-- prej setup <--
EOF
    fi

    ;;
  *nu) # heads up! this might not work, like at all. TEST THIS
    sudo curl -L -o /usr/local/bin/prej.nu "$src/scripts/prej.nu"

    if ! grep -q "# --> prej setup -->" !env.nu!; then
      cat <<EOF >> !env.nu!

# --> prej setup -->
source /usr/local/bin/prej.nu
# <-- prej setup <--
EOF
    fi 

    ;;
  *)
    echo "unsupported shell"
    exit 1

    ;;
esac


    
