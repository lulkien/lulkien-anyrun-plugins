use crate::entry::PowerEntry;

pub fn run(entry: &PowerEntry) {
    if std::process::Command::new(&entry.command)
        .args(&entry.args)
        .spawn()
        .is_err()
    {
        eprintln!("Failed to run {}.", entry.name);
    }

    println!("Run {} with args {:?}", entry.command, entry.args);
}
