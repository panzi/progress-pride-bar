Pride Progress Bar
==================

A progress bar for the terminal that looks like the progress pride flag. I just
thought this is fun, because "progress" in both, and I think the flag works as a
progress bar.

It's not the intersex-inclusive version because I don't know of a way to render
the circle like that across two characters in the terminal. There seems to be no
fitting pair of Unicode symbols.


Usage
-----

```
Usage: progress-pride-bar [OPTIONS]

Options:
  -v, --value <VALUE>
          Display the porgress pride bar at this value or start animation at
          this value. Value must be in the range of 0.0 to 1.0. [default: 1.0]
  -b, --background <BACKGROUND>
          The background color needs to be known because some of the shapes are
          only available inverted, where what is supposed to be the background
          is rendered with a character and the background is colored-in.
          [default: #000000]
  -w, --width <WIDTH>
          Width of the full bar. [default: <the terminal's width>]
  -a, --animate[=<DURATION>]
          Play an animation. Unit suffixes: d, h, m, s, ms, ns [default: 5s]
  -s, --steps <STEPS>
          Number of steps in the animation.
  -f, --fps <FPS>
          Frame rate of the animation. Conflicts with --steps. [default: 60]
  -h, --help
          Print help
  -V, --version
          Print version
```
