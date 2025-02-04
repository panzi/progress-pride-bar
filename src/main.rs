use clap::Parser;
use color::{Rgb, BLACK};
use progress_pride_bar::print_progress_pride_bar;
use std::{fmt::Debug, io::Write, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Instant};

pub mod color;
pub mod duration;
pub mod progress_pride_bar;

use duration::Duration;

fn interruptable_sleep(duration: std::time::Duration) -> bool {
    #[cfg(target_family = "unix")]
    {
        let req = libc::timespec {
            tv_sec:  duration.as_secs() as libc::time_t,
            tv_nsec: duration.subsec_nanos() as i64,
        };
        let ret = unsafe { libc::nanosleep(&req, std::ptr::null_mut()) };
        return ret == 0;
    }

    #[cfg(not(target_family = "unix"))]
    {
        std::thread::sleep(duration);
        return true;
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long,
        value_parser = |s: &str| {
            let Ok(v) = s.parse::<f64>() else {
                return Err(format!("not a number"));
            };

            if v.is_nan() {
                return Err(format!("not a number"));
            }

            if v < 0.0 {
                return Err(format!("less than minimum of 0.0"));
            }

            if v > 1.0 {
                return Err(format!("greater than maximum of 1.0"));
            }

            Ok(v)
        },
        help = "\
            Display the porgress pride bar at this value or start animation at \
            this value. Value must be in the range of 0.0 to 1.0. [default: 1.0]"
    )]
    value: Option<f64>,

    #[arg(short, long, default_value_t = BLACK,
        help = "\
            The background color needs to be known because some of the shapes \
            are only available inverted, where what is supposed to be the \
            background is rendered with a character and the background is \
            colored-in."
    )]
    background: Rgb,

    #[arg(short, long,
        help = "Width of the full bar. [default: <the terminal's width>]"
    )]
    width: Option<usize>,

    #[arg(short, long, value_name = "DURATION",
        num_args = 0..=1, require_equals = true, default_missing_value = "5s",
        help = "Play an animation. Unit suffixes: d, h, m, s, ms, ns [default: 5s]"
    )]
    animate: Option<Duration>,

    #[arg(short, long, requires = "animate",
        help = "Number of steps in the animation."
    )]
    steps: Option<u32>,

    #[arg(short, long, conflicts_with = "steps", requires = "animate",
        help = "\
            Frame rate of the animation. Conflicts with --steps. [default: 60]"
    )]
    fps: Option<u32>,
}

impl Args {
    #[inline]
    pub fn width(&self) -> usize {
        let Some(width) = self.width else {
            if let Some((terminal_size::Width(term_width), _)) = terminal_size::terminal_size() {
                if term_width > 0 {
                    return term_width as usize;
                }
            }

            return 80;
        };

        width
    }
}

fn main() {
    let args = Args::parse();

    let mut out = std::io::stdout().lock();

    if let Some(Duration (animation_duration)) = args.animate {
        let steps = if let Some(steps) = args.steps {
            steps
        } else if let Some(fps) = args.fps {
            (animation_duration.as_secs_f64() * fps as f64) as u32
        } else {
            (animation_duration.as_secs_f64() * 60.0) as u32
        };

        // CSI ?  7 l     No Auto-Wrap Mode (DECAWM), VT100.
        // CSI ? 25 l     Hide cursor (DECTCEM), VT220
        print!("\x1B[?25l\x1B[?7l");

        let fstart = args.value.unwrap_or(0.0);
        let istart = (fstart * steps as f64) as u32;

        let running = Arc::new(AtomicBool::new(true));

        {
            let running = running.clone();
            let _ = ctrlc::set_handler(move || {
                running.store(false, Ordering::Relaxed);
            });
        }

        let animation_start_ts = Instant::now();
        for i in istart..=steps {
            if !running.load(Ordering::Relaxed) {
                break;
            }

            if i != 0 {
                let _ = write!(out, "\r\x1B[5A");
            } else {
                let _ = write!(out, "\r");
            }

            let width = args.width();
            let _ = print_progress_pride_bar(&mut out, width, i as f64 / steps as f64, args.background);
            let _ = write!(out, "\x1B[0m");
            let _ = out.flush();

            let sleep_duration = animation_duration * (i + 1) / steps;
            let elapsed = animation_start_ts.elapsed();
            if sleep_duration > elapsed && !interruptable_sleep(sleep_duration - elapsed) {
                break;
            }
        }

        // CSI 0 m        Reset or normal, all attributes become turned off
        // CSI ?  7 h     Auto-Wrap Mode (DECAWM), VT100
        // CSI ? 25 h     Show cursor (DECTCEM), VT220
        println!("\x1B[0m\x1B[?25h\x1B[?7h");
    } else {
        let width = args.width();
        let _ = print_progress_pride_bar(&mut out, width, args.value.unwrap_or(1.0), args.background);
        let _ = write!(out, "\x1B[0m\n");
        let _ = out.flush();
    }
}
