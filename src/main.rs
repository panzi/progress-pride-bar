use clap::Parser;
use std::{fmt::Debug, io::Write, sync::{atomic::{AtomicBool, Ordering}, Arc}};

pub mod color;
pub mod duration;

use color::{Rgb, blend, BLACK, WHITE, TRANS_PINK, TRANS_BLUE, BROWN, RED, ORANGE, BLUE, PURPLE};
use duration::Duration;

fn interruptable_sleep(duration: std::time::Duration) -> bool {
    #[cfg(target_family = "unix")]
    {
        let nanos = duration.as_nanos();
        let sec = nanos / 1_000_000_000u128;
        let req = libc::timespec {
            tv_sec:  sec as i64,
            tv_nsec: (nanos - (sec * 1_000_000_000u128)) as i64,
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

// 🭏🬼🭏🬼🭏🬼🭏🬼██████████
// 🭕🭢🭕🭢🭕🭢🭕🭢🭕█████████
// █🭏🬼🭏🬼🭏🬼🭏🬼🭏🬼███████
// █🭠🭗🭠🭗🭠🭗🭠🭗🭠🭗███████
// 🭄🭇🭄🭇🭄🭇🭄🭇🭄█████████
// 🭠🭗🭠🭗🭠🭗🭠🭗██████████

const BLOCKS: [&str; 8] = [
    " ", "▏", "▎", "▍", "▌", "▋", "▊", "▉",
];

fn print_line1or6(mut out: impl Write, fchars: f64, tri: &str, bar_color: Rgb, background_color: Rgb) -> std::io::Result<()> {
    let Rgb([tp_r, tp_g, tp_b]) = TRANS_PINK;
    let Rgb([tb_r, tb_g, tb_b]) = TRANS_BLUE;
    let Rgb([br_r, br_g, br_b]) = BROWN;
    let Rgb([bk_r, bk_g, bk_b]) = BLACK;

    if fchars > 11.0 {
        // pink + blue + brown + black + red chunk + bar
        let bar_fchars = fchars - 11.0;
        let bar_ichars = bar_fchars as usize;
        let sub_char = ((bar_fchars - bar_ichars as f64) * 8.0) as usize;
        let Rgb([r, g, b]) = bar_color;

        write!(out, "\x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\x1B[48;2;{bk_r};{bk_g};{bk_b}m\x1B[38;2;{br_r};{br_g};{br_b}m{tri}\x1B[38;2;{bk_r};{bk_g};{bk_b}m\x1B[48;2;{r};{g};{b}m{tri}\x1B[0m\x1B[38;2;{r};{g};{b}m")?;

        for _ in 0..bar_ichars {
            write!(out, "█")?;
        }

        if sub_char > 0 {
            write!(out, "{}", BLOCKS[sub_char])?;
        }
        write!(out, "\x1B[0m\x1B[K\n")?;
    } else if fchars > 10.5 {
        // pink + blue + brown + black + red chunk
        let Rgb([r, g, b]) = blend(background_color, bar_color, (fchars - 10.5) * 2.0);
        write!(out, "\x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\x1B[48;2;{bk_r};{bk_g};{bk_b}m\x1B[38;2;{br_r};{br_g};{br_b}m{tri}\x1B[38;2;{bk_r};{bk_g};{bk_b}m\x1B[48;2;{r};{g};{b}m{tri}\x1B[0m\x1B[K\n")?;
    } else if fchars > 8.5 {
        // pink + blue + brown + black
        let Rgb([r, g, b]) = blend(background_color, BLACK, (fchars - 8.5) * 0.5);
        write!(out, "\x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\x1B[48;2;{r};{g};{b}m\x1B[38;2;{br_r};{br_g};{br_b}m{tri}\x1B[38;2;{r};{g};{b}m\x1B[49m{tri}\x1B[0m\x1B[K\n")?;
    } else if fchars > 6.5 {
        // pink + blue + brown
        let Rgb([r, g, b]) = blend(background_color, BROWN, (fchars - 6.5) * 0.5);
        write!(out, "\x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{r};{g};{b}m{tri}\x1B[49m\x1B[38;2;{r};{g};{b}m{tri}\x1B[0m\x1B[K\n")?;
    } else if fchars > 4.5 {
        // pink + blue
        let Rgb([r, g, b]) = blend(background_color, TRANS_BLUE, (fchars - 4.5) * 0.5);
        write!(out, "\x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{r};{g};{b}m{tri}\x1B[38;2;{r};{g};{b}m\x1B[49m{tri}\x1B[0m\x1B[K\n")?;
    } else if fchars > 2.5 {
        // pink
        let Rgb([r, g, b]) = blend(background_color, TRANS_PINK, (fchars - 2.5) * 0.5);
        write!(out, "\x1B[38;2;{r};{g};{b}m\x1B[49m{tri}\x1B[0m\x1B[K\n")?;
    } else {
        // nothing
        write!(out, "\x1B[0m\x1B[K\n")?;
    }

    Ok(())
}

fn print_line2or5(mut out: impl Write, fchars: f64, tri: &str, bar_color: Rgb, background_color: Rgb) -> std::io::Result<()> {
    let Rgb([tp_r, tp_g, tp_b]) = TRANS_PINK;
    let Rgb([tb_r, tb_g, tb_b]) = TRANS_BLUE;
    let Rgb([br_r, br_g, br_b]) = BROWN;
    let Rgb([bk_r, bk_g, bk_b]) = BLACK;
    let Rgb([wh_r, wh_g, wh_b]) = WHITE;
    let Rgb([bg_r, bg_g, bg_b]) = background_color;

    if fchars > 12.0 {
        // pink + blue + brown + black + red chunk + bar
        let bar_fchars = fchars - 12.0;
        let bar_ichars = bar_fchars as usize;
        let sub_char = ((bar_fchars - bar_ichars as f64) * 8.0) as usize;
        let Rgb([r, g, b]) = bar_color;

        write!(out, "\x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{tb_r};{tb_g};{tb_b}m{tri}\x1B[48;2;{tb_r};{tb_g};{tb_b}m\x1B[38;2;{br_r};{br_g};{br_b}m{tri}\x1B[38;2;{bk_r};{bk_g};{bk_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\x1B[48;2;{bk_r};{bk_g};{bk_b}m\x1B[38;2;{r};{g};{b}m{tri}\x1B[0m\x1B[38;2;{r};{g};{b}m", &tri[4..])?;

        for _ in 0..bar_ichars {
            write!(out, "█")?;
        }

        if sub_char > 0 {
            write!(out, "{}", BLOCKS[sub_char])?;
        }
        write!(out, "\x1B[0m\x1B[K\n")?;
    } else if fchars > 11.5 {
        // white + pink + blue + brown + black + red chunk
        let Rgb([r, g, b]) = blend(background_color, bar_color, (fchars - 11.5) * 2.0);
        write!(out, "\x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{tb_r};{tb_g};{tb_b}m{tri}\x1B[48;2;{tb_r};{tb_g};{tb_b}m\x1B[38;2;{br_r};{br_g};{br_b}m{tri}\x1B[38;2;{bk_r};{bk_g};{bk_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\x1B[48;2;{bk_r};{bk_g};{bk_b}m\x1B[38;2;{r};{g};{b}m{tri}\x1B[0m\x1B[K\n", &tri[4..])?;
    } else if fchars > 8.5 {
        // white + pink + blue + brown + black
        let Rgb([r, g, b]) = blend(background_color, BLACK, (fchars - 8.5).min(1.0) * 0.5);
        write!(out, "\x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{tb_r};{tb_g};{tb_b}m{tri}\x1B[48;2;{tb_r};{tb_g};{tb_b}m\x1B[38;2;{br_r};{br_g};{br_b}m{tri}\x1B[38;2;{r};{g};{b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\x1B[48;2;{r};{g};{b}m\x1B[38;2;{bg_r};{bg_g};{bg_b}m{tri}\x1B[0m\x1B[K\n", &tri[4..])?;
    } else if fchars > 6.5 {
        // white + pink + blue + brown
        let Rgb([r, g, b]) = blend(background_color, BROWN, (fchars - 6.5) * 0.5);
        write!(out, "\x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{tb_r};{tb_g};{tb_b}m{tri}\x1B[48;2;{tb_r};{tb_g};{tb_b}m\x1B[38;2;{r};{g};{b}m{tri}\x1B[38;2;{bg_r};{bg_g};{bg_b}m\x1B[48;2;{r};{g};{b}m{tri}\x1B[0m\x1B[K\n", &tri[4..])?;
    } else if fchars > 4.5 {
        // white + pink + blue
        let Rgb([r, g, b]) = blend(background_color, TRANS_BLUE, (fchars - 4.5) * 0.5);
        write!(out, "\x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{r};{g};{b}m{tri}\x1B[48;2;{r};{g};{b}m\x1B[38;2;{bg_r};{bg_g};{bg_b}m{tri}\x1B[0m\x1B[K\n", &tri[4..])?;
    } else if fchars > 2.5 {
        // white + pink
        let Rgb([r, g, b]) = blend(background_color, TRANS_PINK, (fchars - 2.5) * 0.5);
        write!(out, "\x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{r};{g};{b}m{}\x1B[48;2;{r};{g};{b}m\x1B[38;2;{bg_r};{bg_g};{bg_b}m{tri}\x1B[0m\x1B[K\n", &tri[4..])?;
    } else {
        // white
        let Rgb([r, g, b]) = blend(background_color, WHITE, fchars / 2.5);
        write!(out, "\x1B[48;2;{r};{g};{b}m\x1B[38;2;{bg_r};{bg_g};{bg_b}m{}\x1B[0m\x1B[K\n", &tri[4..])?;
    }

    Ok(())
}

pub fn print_flag(mut out: impl Write, width: usize, value: f64, background_color: Rgb) -> std::io::Result<()> {
    // trinangle widths
    // white: 2.5 chars
    // pink:  2   chars
    // blue:  2   chars
    // brown: 2   chars
    // black: 2   chars
    // 
    // then 0.5 chars at the start of the bars

    let fchars = (width as f64 + 2.5) * value;

    print_line1or6(&mut out, fchars, "🭏🬼", RED, background_color)?;
    print_line2or5(&mut out, fchars, "🭢🭕", ORANGE, background_color)?;
    // TODO: line 3
    // TODO: line 4
    print_line2or5(&mut out, fchars, "🭇🭄", BLUE, background_color)?;
    print_line1or6(&mut out, fchars, "🭠🭗", PURPLE, background_color)?;

    Ok(())
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

fn main() {
    let args = Args::parse();

    let width = args.width.unwrap_or_else(|| {
        let (term_width, _term_heigth) = term_size::dimensions().unwrap_or((0, 0));
        if term_width == 0 {
            return 80;
        }
        term_width
    });

    let mut out = std::io::stdout().lock();

    if let Some(Duration (animate)) = args.animate {
        let steps = args.steps;
        let sleep_duration = animate / steps;

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

        for i in istart..=steps {
            if !running.load(Ordering::Relaxed) {
                break;
            }

            if i != 0 {
                let _ = write!(out, "\x1B[4A");
            }

            let _ = print_flag(&mut out, width, i as f64 / steps as f64, args.background);
            let _ = out.flush();

            if !interruptable_sleep(sleep_duration) {
                break;
            }
        }

        // CSI 0 m        Reset or normal, all attributes become turned off
        // CSI ?  7 h     Auto-Wrap Mode (DECAWM), VT100
        // CSI ? 25 h     Show cursor (DECTCEM), VT220
        print!("\x1B[0m\x1B[?25h\x1B[?7h");
    } else {
        let _ = print_flag(&mut out, width, args.value.unwrap_or(1.0), args.background);
        let _ = out.flush();

    }


}
