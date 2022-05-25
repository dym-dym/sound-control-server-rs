use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Serveur de controle de pulseaudio en réseau", long_about = None)]
pub struct Args {
    // IP pour accéder au serveur
    #[clap(short, long)]
    pub ip: String,

    // Port du serveur
    #[clap(short, long, default_value_t = String::from("9000"))]
    pub port: String,
}

/*
pub struct Network {
    pub ip: String,
    pub port: String,
}

impl Network {
    pub fn new(args: &[String]) -> Result<Network, &'static str> {
        if args.len() != 3 {
            return Err("Error: Usage: server <IP> <PORT>");
        }

        let ip = args[1].clone();
        let port = args[2].clone();

        Ok(Network { ip, port })
    }
}
*/
