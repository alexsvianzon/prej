#!/usr/bin/env nu

def --wrapped --env prej [...args: string] {
  match $args.0 {
    "go" => {
      cd $"(sudo /usr/local/bin/prej dir $args.1)"
      sudo /usr/local/bin/prej go $args.1
    },

    "run" => {
      cd $"(sudo /usr/local/bin/prej dir $args.1)"
      sudo /usr/local/bin/prej run $args.1
    },

    "close" => {
      cd $"(sudo /usr/local/bin/prej dir $args.1)"
      sudo /usr/local/bin/prej close $args.1
    },

    _ => {
      sudo /usr/local/bin/prej ...$args
    }
  }
}
