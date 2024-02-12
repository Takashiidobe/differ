use diffy::create_patch;

use std::{
    fs::{self, read_to_string, File},
    path::Path,
    process::exit,
    sync::mpsc,
    time::Duration,
};

use notify::{self, RecursiveMode, Result};
use notify_debouncer_mini::{new_debouncer_opt, Config};
use std::env;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Please provide a file name as the first argument to this program to create diffs for"
        );
        exit(1);
    }

    let file_name = &args[1];

    match Path::try_exists(Path::new(file_name)) {
        Ok(true) => {}
        Err(_) | Ok(false) => {
            File::create(file_name)?;
        }
    }

    let (tx, rx) = mpsc::channel();
    let backend_config = notify::Config::default().with_poll_interval(Duration::from_secs(1));
    let debouncer_config = Config::default()
        .with_timeout(Duration::from_secs(1))
        .with_notify_config(backend_config);
    let mut debouncer = new_debouncer_opt::<_, notify::PollWatcher>(debouncer_config, tx)?;

    let mut prev = read_to_string(file_name)?;
    fs::write("base.diff", prev.clone())?;
    let mut epoch = 1;
    debouncer
        .watcher()
        .watch(Path::new(&file_name), RecursiveMode::NonRecursive)?;

    for result in rx {
        match result {
            Ok(events) => {
                let file_path = &events.first().unwrap().path;
                let metadata = fs::metadata(file_path)?;
                if !metadata.is_file() {
                    continue;
                }

                let curr = read_to_string(file_name)?;
                if prev == curr {
                    continue;
                }

                let diff = create_patch(&prev, &curr);

                let diff_file_name = format!("{}.diff", epoch);
                fs::write(diff_file_name, diff.to_string().as_bytes()).unwrap();
                epoch += 1;
                prev = curr;
            }
            Err(error) => println!("Error {error:?}"),
        }
    }
    Ok(())
}
