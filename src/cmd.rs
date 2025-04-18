use std::io::{self, BufRead, Write};

use colored::Colorize;

use crate::structs;

pub fn read_commands(host: &String, port: &u16) {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);
    let mut reader = io::BufReader::new(stdin);

    let cmd_str = format!("{}:{}[cli]$ ", host.green().bold(), port.to_string().green().bold());

    loop {
        // Show prompt
        write!(&mut writer, "{}", cmd_str).unwrap();
        writer.flush().unwrap();

        // Read command
        let mut buf: Vec<u8> = Vec::new();
        reader
            .read_until(b'\n', &mut buf)
            .expect("Failed to read command");

        // Marshal the command
        let mut c = structs::command::new();
        c.marshal(&String::from_utf8(buf).expect("Invalid UTF-8 sequence")).unwrap();

        println!("Given command: {:?}", c);
    }
}
