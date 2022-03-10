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

        Ok(Network {
            ip,
            port,
        })
    }
}
