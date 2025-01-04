Kludged is CLI application and library for setting RGB and animation profiles for Royal Kludge keyboards.

# Installation
After cloning the repository you can install it via `cargo`.
```bash
cargo install --path kludged
```
You also might have to add it to your path to use it via the command line. You can follow the instructions for 
your operating system on how to do so. 

If you are using Linux, you will need to set some udev rules, you should run `kludged udev` as root and `kludged`
will handle everything for you. You will likely have to refresh your udev rules, follow your operating system 
documentation on how to do so.

# Usage

`kludged` will only show commands available for detected keyboards to avoid clutter. You run `kludged --help`
to see what settings are available.

There are a few common operations available on many keyboards:
- `set-anim`
- `set-color`

To see what arguments are available for each operating add `--help` at the end of the command.

## Example Help Output for RK68
```
Set the animation of the keyboard.

Usage: kludged set-anim [OPTIONS] --anim <anim>

Options:
  -m, --color-mix
          Enable color mix.
  -c, --color <COLOR>
          [default: red]
  -v, --verbose...
          Increase logging verbosity
  -q, --quiet...
          Decrease logging verbosity
  -s, --sleep <sleep>
          [default: ten-minutes] [possible values: five-minutes,
          ten-minutes, twenty-minutes, thirty-minutes, never]
      --speed <speed>
          [default: one] [possible values: one, two, three, four, five]
  -a, --anim <anim>
          [possible values: neon-stream, ripples-shining,
          rotating-windmill, sine-wave, rainbow-roulette, stars-twinkle,
          layer-upon-layer, rich-and-honored, marquee-effect,
          rotating-storm, serpentine-horse, retro-snake,
          diagonal-transformer, ambilight, streamer, steady, breathing,
          neon, shadow-disappear, flash-away]
  -b, --brightness <brightness>
          [default: five] [possible values: zero, one, two, three, four,
          five]
  -h, --help
          Print help (see more with '--help')
```

## Setting the Color to Red
```
kludged set-color -c red
# or "#FF0000" instead of "red"
```

## Setting the Animation
The animations available will depend on the model of the keyboard. 
The example below shows a common one present in Royal Kludge keyboards.
To see what animations are available for your keyboard you can run `kludged set-anim --help`.
```
kludged set-anim --anim neon-stream -m --speed five
```

# As a Library

The implementation is really light weight, and everything should work as expected as long as 
the target has `hidapi` bindings available on the system, this includes some embedded systems as well.

# Alternatives
To my knowledge there isn't any other application that provides the utilities through 
the command line other than this, and another project I worked on called [Regium Klavye](https://github.com/airblast-dev/Regium-Klavye). 
If you need bindings for python or prefer the application is written in it, it is a good option.
