use std::{error::Error, fs, panic::Location, path::Path, time::Instant};

#[track_caller]
pub fn read_file<T, F>(mut select: F) -> Result<T, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let day = Path::new(Location::caller().file())
        .file_stem()
        .ok_or("caller has no file name")?;

    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("inputs")
        .join(day)
        .with_extension("txt");

    let contents = fs::read_to_string(&file)
        .map_err(|error| format!("cannot read {}: {error}", file.display()))?;

    if contents.trim().is_empty() {
        return Err(format!("{} is empty, paste your puzzle input", file.display()).into());
    }

    Ok(select(contents))
}

#[derive(Default)]
pub struct StopWatch {
    instant: Option<Instant>,
}

impl StopWatch {
    pub fn new() -> Self {
        Self { instant: None }
    }

    pub fn start(&mut self) {
        self.instant = Some(Instant::now());
    }

    pub fn stop(&self) {
        if let Some(duration) = self.instant {
            let elapsed = duration.elapsed();

            println!("Duration: {elapsed:?}")
        } else {
            println!("Stop was called before start")
        }
    }
}
