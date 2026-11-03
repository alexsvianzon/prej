# Prejfile spec
version: "0.3.3"

# A Prejfile defines tasks that are run with different lifecycle hooks
#
# Defining tasks start with namespaces. A namespace defines a group of tasks that can be run or
# invoked by other tasks. There are four built in namespaces: input, setup, demand, and close. They
# are defined below. You can also define your own namespaces which you can call manually from the
# CLI or by invoking them from other tasks. Tasks can depend on other tasks, which means that they
# will run after the tasks they depend on.
#
# The 'setup' namespace defines tasks that get run when the user jumps to a project.

setup:
  python-env:                # this is a namespace task
    cmd: source              # define the main command that this task runs
    args:                    # pass args to this task's command
      - .venv/bin/activate

      source .venv/bin/activate

# The 'demand' namespace defines tasks that are run when the user requests. They must be terminate
# to return the user to their terminal.

demand:
  test:
    cmd: cargo
    args:
      - test
      - -v

  build:
    cmd: cargo
    args:
      - build
    depends:
      - test

# The 'close' namespace lists commands that get run when the user calls it manually or jumps to
# another project.

close:
  cleanup:
    cmd: "rm"
    args:
      - "-rf"
      - "build/"