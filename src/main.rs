use clap::{App, Arg, SubCommand};
use keyring::Keyring;

fn main(){
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("get")
            .help("get the value for a key")
            .arg(
                Arg::with_name("key")
                    .short("k")
                    .long("key")
                    .help("the key")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("service")
                    .short("s")
                    .long("service")
                    .help("the service")
                    .takes_value(true),
            )
        )
        .get_matches();
        if let Some(ref matches) = matches.subcommand_matches("get"){
            let key = match matches.value_of("key"){
                Some(key) => key,
                None => panic!("key must be provided"),
            };
            let service = match matches.value_of("service"){
                Some(value) => value,
                None => "system",
            };
            let keyring = Keyring::new(&service, &key);
            let password = keyring.get_password().unwrap();
            println!("{}", password);
        }
}
