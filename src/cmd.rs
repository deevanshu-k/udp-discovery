use std::io::{self, BufRead, Write};

pub fn read_commands(host: &String, port: &u16) {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);
    let mut reader = io::BufReader::new(stdin);

    let cmd_str = format!("{}:{}[cli]$ ", host, port);

    loop {
        // Show prompt
        write!(&mut writer, "{}", cmd_str).unwrap();
        writer.flush().unwrap();

        // Read command
        let mut buf = Vec::new();
        reader
            .read_until(b'\n', &mut buf)
            .expect("Failed to read command");
    }
}
