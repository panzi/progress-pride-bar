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
    #[arg(short, long)]
    value: Option<f64>,

    #[arg(short, long, default_value_t = BLACK)]
    background: Rgb,

    #[arg(short, long)]
    width: Option<usize>,

    #[arg(short, long, value_name = "DURATION")]
    animate: Option<Duration>,

    #[arg(short, long, default_value_t = 1000)]
    steps: u32,
}

impl Args {
    #[inline]
    pub fn width(&self) -> usize {
        let Some(width) = self.width else {
            let (term_width, _term_heigth) = term_size::dimensions().unwrap_or((0, 0));
            if term_width == 0 {
                return 80;
            }
            return term_width;
        };

        width
    }
}

fn main() {
    let args = Args::parse();

    let mut out = std::io::stdout().lock();

    if let Some(Duration (animation_duration)) = args.animate {
        let steps = args.steps;

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
        let _ = out.flush();
    }
}
