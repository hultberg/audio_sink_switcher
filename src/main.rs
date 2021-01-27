use std::process::Command;

struct Sink {
    pub key: String,
    pub name: String,
}

fn main() {
    let all_sinks: Vec<Sink> = vec![
        Sink { key: "alsa_output.usb-Creative_Technology_USB_Sound_Blaster_HD_000001T4-00.analog-stereo".to_owned(), name: "dt".to_owned() },
        Sink { key: "alsa_output.usb-GN_Audio_A_S_Jabra_EVOLVE_30_II_0010D6BD783E08-00.analog-stereo".to_owned(), name: "jabra".to_owned() },
    ];

    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Missing first arg");
        std::process::exit(1);
    }

    for arg in args.iter() {
        if arg.eq("--print-active") {
            let info_output = String::from_utf8(
                Command::new("pactl").args(&["info"]).output().expect("Failed to get info").stdout
            ).unwrap();

            let line_by_line_iterator = info_output.lines();

            for line in line_by_line_iterator {
                if line.find("Default Sink: ") != None {
                    let s: &str = line.split(": ").collect::<Vec<&str>>()[1];

                    for sink in all_sinks.iter() {
                        if sink.key == s {
                            println!("{}", sink.name);
                            std::process::exit(0);
                        }
                    }

                    println!("no match");
                    std::process::exit(1);
                }
            }

            println!("invalid output from pactl");
            break;
        }

        if arg.eq("--set-active") {
            if args.len() <= 2 {
                println!("Missing first arg: dt/jabra");
                std::process::exit(1);
            }

            match args[2].as_str() {
                "dt" => {
                    Command::new("pactl").args(&["set-default-sink", all_sinks[0].key.as_str()]).output().expect("Failed to change to DT");
                },
                "jabra" => {
                    Command::new("pactl").args(&["set-default-sink", all_sinks[1].key.as_str()]).output().expect("Failed to change to JABRA");
                },
                _ => {
                    println!("Unknown arg value: {}", args[1].as_str());
                    std::process::exit(1);
                },
            }

            break;
        }
    }
}
