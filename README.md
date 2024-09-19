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

```sh
pray <path-to-config.yml> <path-to-templates-folder> <path-to-target-folder>
```

It will read the template folder and expect to find a file name "spawner.cairo.tera" in it

It will then write the resulting file into `<path-to-target-folde>/spawner.cairo`

### Example

in the `TheOrugginTrail-DoJo` repo, we have a `spawner` folder that contains the `config.yml` file and a `templates` folder with `spanwer.cairo.tera` in it.

To generate the `spawner.cairo` in `src/systems` you just execute the following:

```sh
pray spawner/config.yml spawner/templates/ src/systems/
```

Remember to have `pray` in your path

## watch feature

The Pray Machine can also work in watch mode, where every time you modify and save the config or template file, the resulting file is regenerated

To do so, just append `--watch` to your command

```sh
pray <path-to-config.yml> <path-to-templates-folder> <path-to-target-folder> --watch
```
