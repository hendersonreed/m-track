use std::error::Error;
use std::io::{stdin, stdout, Write};

use midly::{live::LiveEvent, MidiMessage};
use midir::{Ignore, MidiInput};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn on_midi(event: &[u8]) {
    let event = LiveEvent::parse(event).unwrap();
    match event {
        LiveEvent::Midi { channel, message } => match message {
            MidiMessage::NoteOn { key, vel } => {
                println!("hit note {} on channel {}, with velociy {}", key, channel, vel);
            }
            _ => {}
        },
        _ => {}
    }
}

fn run() -> Result<(), Box<dyn Error>> {

    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;
    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        |_stamp, message, _| {
            on_midi(message);
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
