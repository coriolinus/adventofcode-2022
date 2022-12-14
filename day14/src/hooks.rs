use std::{path::Path, time::Duration};

use aoclib::geometry::map::{Animation, Style};

use crate::{cavern::Cavern, Error};

static mut ANIMATION: Option<Animation> = None;
const ANIMATION_SPEED_FACTOR: u32 = 4;

pub fn pre(cavern: &Cavern) -> Result<(), Error> {
    use std::env::var;

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
        let mut animation = cavern.map.prepare_animation(
            Path::new(&path),
            Duration::from_secs(1) / ANIMATION_SPEED_FACTOR,
            Style::Grid,
        )?;

        for _ in 0..ANIMATION_SPEED_FACTOR {
            animation.write_frame(&cavern.map)?;
        }

        unsafe { ANIMATION = Some(animation) };
    }

    Ok(())
}

pub fn trace(cavern: &Cavern) -> Result<(), Error> {
    unsafe {
        if let Some(animation) = ANIMATION.as_mut() {
            animation.write_frame(&cavern.map)?;
        }
    }
    Ok(())
}

pub fn post(cavern: &Cavern) -> Result<(), Error> {
    use std::env::var;

    if var("CONSOLE_POST")
        .map(|val| !val.is_empty())
        .unwrap_or_default()
    {
        println!("{cavern}");
    }

    if let Ok(path) = var("IMAGE_POST") {
        cavern.map.render(Path::new(&path), Style::Grid)?;
    }

    unsafe {
        if let Some(animation) = ANIMATION.as_mut() {
            for _ in 0..ANIMATION_SPEED_FACTOR {
                animation.write_frame(&cavern.map)?;
            }
        }
    }

    Ok(())
}
