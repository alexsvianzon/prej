#!/usr/bin/env fish

function prej
    if sudo -nv 2>/dev/null
        echo "sudo privileges are available"
    else
        echo "prej requires sudo privileges to run. please input your password:"
        sudo -v
    end

    switch $argv[1]
    case go
        cd "$(sudo /usr/local/bin/prej dir $argv[2])"
    case '*'
        sudo /usr/local/bin/prej $argv
    end
end