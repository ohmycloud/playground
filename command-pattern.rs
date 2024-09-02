use std::fs::File;
use std::io::{BufRead, BufReader, Error, Seek, Write};

trait Command {
    fn execute(&self) -> Result<(), Error>;
}

struct FileReader {
    // receiver is the receiver(or target) of our command.
    receiver: File,
}

struct FileWriter {
    content: String,
    receiver: File,
}


impl FileReader {
    fn new(receiver: File) -> Box<Self> {
        // We’re returning a boxed object so we can use trait objects later
        Box::new(Self { receiver })
    }
}

impl FileWriter {
    fn new(content: String, receiver: File) -> Box<Self> {
        Box::new(Self { content, receiver })
    }
}

impl Command for FileWriter {
    fn execute(&self) -> Result<(), Error> {
        println!("Writing new content to file");
        let mut writer = self.receiver.try_clone()?;

        writer.write_all(self.content.as_bytes())?;
        writer.flush()?;

        Ok(())
    }
}

impl Command for FileReader {
    fn execute(&self) -> Result<(), Error> {
        println!("Reading from start of file");
        let mut reader = BufReader::new(&self.receiver);
        reader.seek(std::io::SeekFrom::Start(0))?;

        for (count, line) in reader.lines().enumerate() {
            println!("{:2}: {}", count + 1, line?);
        }
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("file.txt")?;

    let commands: Vec<Box<dyn Command>> = vec![
        FileReader::new(file.try_clone()?),
        FileWriter::new(
            "Rakudo Star".into(), file.try_clone()?
        ),
        FileReader::new(file.try_clone()?),
    ];

    for command in commands {
        command.execute()?;
    }
    Ok(())
}