# Prejfile spec
version: "0.3.2"

# a Prejfile defines tasks that are run with different lifecycle hooks
#
# defining tasks start with namespaces. a namespace provides tasks that can be run or invoked by
# other tasks. there are four built in namespaces: input, setup, demand, and close. they are defined
# below. you can also make your own namespaces which you can call manually from the CLI or by
# invoking them from other tasks (!notmvp)
#
# the 'setup' namespace lists tasks that get run when the user jumps to a project. they are run in
# the background by the daemon by default (but can be changed), and either stop when you leave the 
# project or when you stop them manually

setup:
  git-pull: # this is a namespace task
    cmd: "git" # define the main command that this task runs
    args: # pass args to this task's command
      - "pull"
      - "origin"

  print-message:
    cmd: "echo"
    args:
      - "message.txt"

# the 'demand' namespace lists commands that are run when the user requests. they run in the
# foreground by default and must be stopped before you can continue working (unless you run it in a
# seperate terminal instance)

demand:
  test:
    cmd: "cargo"
    args:
      - "test"
      - "-v"

# the 'close' namespace lists commands that get run when the user:
#   - calls the service manually
#   - jumps to another project
#
# beyond the user-defined tasks listed, the 'close' namespace also kills the processes started by the
# 'setup' service
#
# tasks can depend on other tasks, which means that they will run after the tasks they depend on

close:
  cleanup:
    cmd: "rm"
    args:
      - "-rf"
      - "build/"

  touch-message:
    cmd: "touch"
    args:
      - "message.txt"

  note:
    cmd: "echo"
    args:
      - input::commit-message
      - "> message.txt"

    depends:
      - "touch-message"

# the 'input' namespace lists tasks that get run when the user is prompted for input. they are
# special tasks that you should not call manually, but invoke into other tasks.

input:
  commit-message:
    prompt: "note for next session:" # the message that gets printed to the terminal

