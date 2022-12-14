#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Argument");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self { query, filename })
    }
}
