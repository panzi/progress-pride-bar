use std::io::Write;

use crate::color::{blend, Rgb, BLACK, BLUE, BROWN, GREEN, ORANGE, PURPLE, RED, TRANS_BLUE, TRANS_PINK, WHITE, YELLOW};

// 🭏🬼🭏🬼🭏🬼🭏🬼██████████
// 🭕🭢🭕🭢🭕🭢🭕🭢🭕█████████
// █🭏🬼🭏🬼🭏🬼🭏🬼🭏🬼███████
// █🭠🭗🭠🭗🭠🭗🭠🭗🭠🭗███████
// 🭄🭇🭄🭇🭄🭇🭄🭇🭄█████████
// 🭠🭗🭠🭗🭠🭗🭠🭗██████████

const BLOCKS: [&str; 8] = [
    " ", "▏", "▎", "▍", "▌", "▋", "▊", "▉",
];

fn print_line1or6(mut out: impl Write, fchars: f64, tri: &str, bar_color: Rgb, background_color: Rgb, newline: bool) -> std::io::Result<()> {
    let Rgb([tp_r, tp_g, tp_b]) = TRANS_PINK;
    let Rgb([tb_r, tb_g, tb_b]) = TRANS_BLUE;
    let Rgb([br_r, br_g, br_b]) = BROWN;
    let Rgb([bk_r, bk_g, bk_b]) = BLACK;
    let Rgb([bg_r, bg_g, bg_b]) = background_color;
    let nl = if newline { "\n" } else { "" };

    if fchars > 12.5 {
        // pink + blue + brown + black + chunk + bar
        let bar_fchars = fchars - 12.5;
        let bar_ichars = bar_fchars as usize;
        let sub_char = ((bar_fchars - bar_ichars as f64) * 8.0) as usize;
        let Rgb([r, g, b]) = bar_color;

        write!(out, "\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\
            \x1B[38;2;{br_r};{br_g};{br_b}m\x1B[48;2;{bk_r};{bk_g};{bk_b}m{tri}\
            \x1B[38;2;{bk_r};{bk_g};{bk_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m")?;

        for _ in 0..bar_ichars {
            write!(out, "█")?;
        }

        if sub_char > 0 {
            write!(out, "{}", BLOCKS[sub_char])?;
        }
        write!(out, "\x1B[K{nl}")?;
    } else if fchars > 10.5 {
        // pink + blue + brown + black + chunk
        let Rgb([r, g, b]) = blend(background_color, bar_color, (fchars - 10.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\
            \x1B[38;2;{br_r};{br_g};{br_b}m\x1B[48;2;{bk_r};{bk_g};{bk_b}m{tri}\
            \x1B[38;2;{bk_r};{bk_g};{bk_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[K{nl}")?;
    } else if fchars > 8.5 {
        // pink + blue + brown + black
        let Rgb([r, g, b]) = blend(background_color, BLACK, (fchars - 8.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\
            \x1B[38;2;{br_r};{br_g};{br_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[K{nl}")?;
    } else if fchars > 6.5 {
        // pink + blue + brown
        let Rgb([r, g, b]) = blend(background_color, BROWN, (fchars - 6.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[K{nl}")?;
    } else if fchars > 4.5 {
        // pink + blue
        let Rgb([r, g, b]) = blend(background_color, TRANS_BLUE, (fchars - 4.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[K{nl}")?;
    } else if fchars > 2.5 {
        // pink
        let Rgb([r, g, b]) = blend(background_color, TRANS_PINK, (fchars - 2.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[K{nl}")?;
    } else {
        // nothing
        write!(out, "\x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[K{nl}")?;
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

    if fchars > 13.5 {
        // pink + blue + brown + black + chunk + bar
        let bar_fchars = fchars - 13.5;
        let bar_ichars = bar_fchars as usize;
        let sub_char = ((bar_fchars - bar_ichars as f64) * 8.0) as usize;
        let Rgb([r, g, b]) = bar_color;

        write!(out, "\
            \x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\
            \x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[48;2;{tb_r};{tb_g};{tb_b}m\x1B[38;2;{br_r};{br_g};{br_b}m{tri}\
            \x1B[48;2;{br_r};{br_g};{br_b}m\x1B[38;2;{bk_r};{bk_g};{bk_b}m{tri}\
            \x1B[48;2;{bk_r};{bk_g};{bk_b}m\x1B[38;2;{r};{g};{b}m{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m", &tri[4..])?;

        for _ in 0..bar_ichars {
            write!(out, "█")?;
        }

        if sub_char > 0 {
            write!(out, "{}", BLOCKS[sub_char])?;
        }
        write!(out, "\x1B[K\n")?;
    } else if fchars > 12.0 {
        // white + pink + blue + brown + black + chunk
        let Rgb([r, g, b]) = blend(background_color, bar_color, (fchars - 12.0) / 1.5);
        write!(out, "\
            \x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\
            \x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[48;2;{tb_r};{tb_g};{tb_b}m\x1B[38;2;{br_r};{br_g};{br_b}m{tri}\
            \x1B[48;2;{br_r};{br_g};{br_b}m\x1B[38;2;{bk_r};{bk_g};{bk_b}m{tri}\
            \x1B[48;2;{bk_r};{bk_g};{bk_b}m\x1B[38;2;{r};{g};{b}m{tri}\
            \x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[K\n", &tri[4..])?;
    } else if fchars > 8.5 {
        // white + pink + blue + brown + black
        let Rgb([r, g, b]) = blend(background_color, BLACK, (fchars - 8.5).min(2.0) * 0.5);
        write!(out, "\
            \x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\
            \x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[48;2;{tb_r};{tb_g};{tb_b}m\x1B[38;2;{br_r};{br_g};{br_b}m{tri}\
            \x1B[48;2;{br_r};{br_g};{br_b}m\x1B[38;2;{r};{g};{b}m{tri}\
            \x1B[48;2;{r};{g};{b}m\x1B[38;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[K\n", &tri[4..])?;
    } else if fchars > 6.5 {
        // white + pink + blue + brown
        let Rgb([r, g, b]) = blend(background_color, BROWN, (fchars - 6.5) * 0.5);
        write!(out, "\
            \x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\
            \x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[48;2;{tb_r};{tb_g};{tb_b}m\x1B[38;2;{r};{g};{b}m{tri}\
            \x1B[48;2;{r};{g};{b}m\x1B[38;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[K\n", &tri[4..])?;
    } else if fchars > 4.5 {
        // white + pink + blue
        let Rgb([r, g, b]) = blend(background_color, TRANS_BLUE, (fchars - 4.5) * 0.5);
        write!(out, "\
            \x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{tp_r};{tp_g};{tp_b}m{}\
            \x1B[48;2;{tp_r};{tp_g};{tp_b}m\x1B[38;2;{r};{g};{b}m{tri}\
            \x1B[48;2;{r};{g};{b}m\x1B[38;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[K\n", &tri[4..])?;
    } else if fchars > 2.5 {
        // white + pink
        let Rgb([r, g, b]) = blend(background_color, TRANS_PINK, (fchars - 2.5) * 0.5);
        write!(out, "\
            \x1B[48;2;{wh_r};{wh_g};{wh_b}m\x1B[38;2;{r};{g};{b}m{}\
            \x1B[48;2;{r};{g};{b}m\x1B[38;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[K\n", &tri[4..])?;
    } else {
        // white
        let Rgb([r, g, b]) = blend(background_color, WHITE, fchars / 2.5);
        write!(out, "\
            \x1B[48;2;{r};{g};{b}m\x1B[38;2;{bg_r};{bg_g};{bg_b}m{}\
            \x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[K\n", &tri[4..])?;
    }

    Ok(())
}

fn print_line3or4(mut out: impl Write, fchars: f64, tri: &str, bar_color: Rgb, background_color: Rgb) -> std::io::Result<()> {
    let Rgb([tp_r, tp_g, tp_b]) = TRANS_PINK;
    let Rgb([tb_r, tb_g, tb_b]) = TRANS_BLUE;
    let Rgb([br_r, br_g, br_b]) = BROWN;
    let Rgb([bk_r, bk_g, bk_b]) = BLACK;
    let Rgb([wh_r, wh_g, wh_b]) = WHITE;
    let Rgb([bg_r, bg_g, bg_b]) = background_color;

    if fchars > 15.5 {
        // white + pink + blue + brown + black + red chunk + bar
        let bar_fchars = fchars - 15.5;
        let bar_ichars = bar_fchars as usize;
        let sub_char = ((bar_fchars - bar_ichars as f64) * 8.0) as usize;
        let Rgb([r, g, b]) = bar_color;

        write!(out, "\
            \x1B[38;2;{wh_r};{wh_g};{wh_b}m\x1B[48;2;{tp_r};{tp_g};{tp_b}m█{tri}\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\
            \x1B[38;2;{br_r};{br_g};{br_b}m\x1B[48;2;{bk_r};{bk_g};{bk_b}m{tri}\
            \x1B[38;2;{bk_r};{bk_g};{bk_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m")?;

        for _ in 0..bar_ichars {
            write!(out, "█")?;
        }

        if sub_char > 0 {
            write!(out, "{}", BLOCKS[sub_char])?;
        }
        write!(out, "\x1B[K\n")?;
    } else if fchars > 13.5 {
        // white + pink + blue + brown + black + red chunk
        let Rgb([r, g, b]) = blend(background_color, bar_color, (fchars - 13.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{wh_r};{wh_g};{wh_b}m\x1B[48;2;{tp_r};{tp_g};{tp_b}m█{tri}\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\
            \x1B[38;2;{br_r};{br_g};{br_b}m\x1B[48;2;{bk_r};{bk_g};{bk_b}m{tri}\
            \x1B[38;2;{bk_r};{bk_g};{bk_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[K\n")?;
    } else if fchars > 8.5 {
        // white + pink + blue + brown + black
        let Rgb([r, g, b]) = blend(background_color, BLACK, (fchars - 8.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{wh_r};{wh_g};{wh_b}m\x1B[48;2;{tp_r};{tp_g};{tp_b}m█{tri}\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{br_r};{br_g};{br_b}m{tri}\
            \x1B[38;2;{br_r};{br_g};{br_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[K\n")?;
    } else if fchars > 6.5 {
        // white + pink + blue + brown
        let Rgb([r, g, b]) = blend(background_color, BROWN, (fchars - 6.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{wh_r};{wh_g};{wh_b}m\x1B[48;2;{tp_r};{tp_g};{tp_b}m█{tri}\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{tb_r};{tb_g};{tb_b}m{tri}\
            \x1B[38;2;{tb_r};{tb_g};{tb_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[K\n")?;
    } else if fchars > 4.5 {
        // white + pink + blue
        let Rgb([r, g, b]) = blend(background_color, TRANS_BLUE, (fchars - 4.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{wh_r};{wh_g};{wh_b}m\x1B[48;2;{tp_r};{tp_g};{tp_b}m█{tri}\
            \x1B[38;2;{tp_r};{tp_g};{tp_b}m\x1B[48;2;{r};{g};{b}m{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[K\n")?;
    } else if fchars > 2.5 {
        // white + pink
        let Rgb([r, g, b]) = blend(background_color, TRANS_PINK, (fchars - 2.5) * 0.5);
        write!(out, "\
            \x1B[38;2;{wh_r};{wh_g};{wh_b}m\x1B[48;2;{r};{g};{b}m█{tri}\
            \x1B[38;2;{r};{g};{b}m\x1B[48;2;{bg_r};{bg_g};{bg_b}m{tri}\
            \x1B[K\n")?;
    } else {
        // white
        let Rgb([r, g, b]) = blend(background_color, WHITE, fchars / 2.5);
        write!(out, "\
            \x1B[48;2;{bg_r};{bg_g};{bg_b}m\x1B[38;2;{r};{g};{b}m█{tri}\
            \x1B[K\n")?;
    }

    Ok(())
}

pub fn print_progress_pride_bar(mut out: impl Write, width: usize, value: f64, background_color: Rgb) -> std::io::Result<()> {
    // trinangle widths
    // white: 2.5 chars
    // pink:  2   chars
    // blue:  2   chars
    // brown: 2   chars
    // black: 2   chars
    // 
    // then 2.0 overlapping chars at the start of the full blocks

    let fchars = (width as f64 + 4.5) * value;

    print_line1or6(&mut out, fchars, "🭏🬼", RED,    background_color, true)?;
    print_line2or5(&mut out, fchars, "🭢🭕", ORANGE, background_color)?;
    print_line3or4(&mut out, fchars, "🭏🬼", YELLOW, background_color)?;
    print_line3or4(&mut out, fchars, "🭠🭗", GREEN,  background_color)?;
    print_line2or5(&mut out, fchars, "🭇🭄", BLUE,   background_color)?;
    print_line1or6(&mut out, fchars, "🭠🭗", PURPLE, background_color, false)?;

    Ok(())
}
