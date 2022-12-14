use std::{env::var, path::Path, time::Duration};

use aoclib::geometry::map::{Animation, Style};

use crate::{cavern::Cavern, Error};

static mut ANIMATION: Option<Animation> = None;
static mut FRAME: usize = 0;
static mut EVERY_N_FRAMES: Option<usize> = None;

fn fps() -> u32 {
    var("FPS").ok().and_then(|s| s.parse().ok()).unwrap_or(60)
}

pub fn pre(cavern: &Cavern) -> Result<(), Error> {
    if var("CONSOLE_PRE")
        .map(|val| !val.is_empty())
        .unwrap_or_default()
    {
        println!("{cavern}");
    }

    if let Ok(path) = var("IMAGE_PRE") {
        cavern.map.render(Path::new(&path), Style::Grid)?;
    }

    if let Ok(path) = var("ANIMATION") {
        let fps = fps();

        let mut animation = cavern.map.prepare_animation(
            Path::new(&path),
            Duration::from_secs(1) / fps / 2,
            Style::Grid,
        )?;

        for _ in 0..fps {
            animation.write_frame(&cavern.map)?;
        }

        unsafe {
            ANIMATION = Some(animation);
            EVERY_N_FRAMES = var("EVERY_N_FRAMES").ok().and_then(|s| s.parse().ok());
        }
    }

    Ok(())
}

pub fn trace(cavern: &Cavern) -> Result<(), Error> {
    unsafe {
        if let Some(animation) = ANIMATION.as_mut() {
            if EVERY_N_FRAMES.is_none() || FRAME == 0 {
                animation.write_frame(&cavern.map)?;
            }
            FRAME += 1;
            if let Some(n) = EVERY_N_FRAMES {
                FRAME %= n;
            }
        }
    }
    Ok(())
}

pub fn post(cavern: &Cavern) -> Result<(), Error> {
    if var("CONSOLE_POST")
        .map(|val| !val.is_empty())
        .unwrap_or_default()
    {
        println!("{cavern}");
    }

    if let Ok(path) = var("IMAGE_POST") {
        cavern.map.render(Path::new(&path), Style::Grid)?;
    }

    // take the animation so it drops/writes when this function terminates, not at program exit.
    let animation = unsafe { ANIMATION.take() };
    if let Some(mut animation) = animation {
        for _ in 0..fps() {
            animation.write_frame(&cavern.map)?;
        }
    }

    Ok(())
}
