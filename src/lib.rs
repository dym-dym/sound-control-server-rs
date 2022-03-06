pub struct Network {
    ip: String,
    port: String,
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

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn network_parsing() {
        assert_eq!(Network::new(vec!["server", "127.0.0.1", "9000"]), Network {"127.0.0.1", "9000"});
    }

    #[test]
    fn network_error() {
        assert_eq!(Network::new(vec!["server"]), Err(format!("Error: Usage: {} <IP> <PORT>"), "server"));
    }
}
