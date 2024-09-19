# The Pray Machine

A tool to generate the spawner for TheOrugginTrail.

it takes as input the following:

- a config file describing the rooms, their objects (including exits) and actions
- a template that is feeded with the data from the parsed config file

## How to use:

1. build it

```
cd cli
cargo build
```

(Note: we use debug build)

2. add it to your path

(the binary is located at [./cli/target/debug/pray](./cli/target/debug/pray))

3. execute it

```
pray <path-to-config.yml> <path-to-templates-folder> <path-to-target-folder>
```
