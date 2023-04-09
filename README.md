# Disable
A simple program that "disables" a file. This is something
I do regularly.

It just adds `.disabled` to the end of the
file. If the file is already disabled, then it will remove
the `.disabled` from it.

Optionally, the program can be given a second argument,
and this will determine what ending to add or remove from
the file. The added arg must contain a `.`, if you want
it. This is really because rust is a slight pain with
concat strings. But this is actually better.

And for once, this program does not have a plant ending.

## Features


## Use
This program needs to be called like a normal program
would (`./disabled`), however you can add it to `/usr/bin`
to run it anywhere like a normal program.

### General Use
```shell
# run the program on a normal file to disable it
disable test.txt
# will rename to `test.txt.disabled`
```

```shell
# run the program on any file
disable ~/Documents/Code/project1/bin/test.dyllib

disable ~/mincraft/modpack/mods/jei.jar
```

```shell
# re-enable a file
disable test.txt.disabled
# will rename to `test.txt`
```

### Custom Extensions

```shell
# you can disable with a custom extension as well
disable test.txt .dis
# will rename to `test.txt.dis`
```

```shell
# to re-enable a custom ext
disable test.txt.dis .dis
# will rename to `test.txt`
```