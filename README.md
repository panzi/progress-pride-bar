Pride Progress Bar
==================

A progress bar for the terminal that looks like the progress pride flag. I just
thought this is fun, because "progress" in both, and I think the flag works as a
progress bar.

It's not the intersex-inclusive version because I don't know of a way to render
the circle like that across two characters in the terminal. There seems to be no
fitting pair of Unicode symbols.

**Note:** For this to render right you need to use a font in your terminal that
supports Unicode [Symbols for Legacy Computing](https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing).
[Konsole](https://konsole.kde.org/)'s default font
[Hack](https://github.com/source-foundry/Hack) is an example of such a font.

Demo video:

[![Demo video](https://i3.ytimg.com/vi/kK4r7wW1X4c/maxresdefault.jpg)](https://www.youtube.com/watch?v=kK4r7wW1X4c)

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

Related Projects
----------------

Other things I made that render Uinocde characters to the terminal:

- [Color Cycling](https://github.com/panzi/rust-color-cycle) (Rust): This is a
  method to give otherwise static pixel art images some kind of animation using
  its color palette.
- [Term Flags](https://github.com/panzi/python-term-flags) (Python): A primitive
  sytem to render simple scalable flags on the terminal using Unicode.
- [Bad Apple!! but its the Unix Terminal](https://github.com/panzi/rust-color-cycle)
  (C): A program that displays the Bad Apple!! animation on the terminal.
- [ANSI IMG](https://github.com/panzi/ansi-img) (Rust): Display images (including
  animated GIFs) on the terminal.
- [Unicode Bar Charts](https://github.com/panzi/js-unicode-bar-chart)
  (JavaScript): Draw bar charts on the terminal. With 8 steps per character and
  with colors.
- [Unicode Progress Bars](https://github.com/panzi/js-unicode-progress-bar)
  (JavaScript): Draw bar charts on the terminal. With 8 steps per character,
  border styles, and colors.
- [Unicode Unicode Plots](https://github.com/panzi/js-unicode-plot) (JavaScript):
  Very simple plotting on the terminal. No colors.
