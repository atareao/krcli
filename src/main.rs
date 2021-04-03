use clap::{App, Arg, SubCommand};
use keyring::Keyring;
use exitcode;

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
        .subcommand(SubCommand::with_name("set")
            .help("set the value for a key")
            .arg(
                Arg::with_name("key")
                    .short("k")
                    .long("key")
                    .help("the key")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("value")
                    .short("v")
                    .long("value")
                    .help("the value")
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
        .subcommand(SubCommand::with_name("delete")
            .help("delete a key-value pair")
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
        if let Some(get_matches) = &matches.subcommand_matches("get"){
            let key = match get_matches.value_of("key"){
                Some(key) => key,
                None => panic!("key must be provided"),
            };
            let service = match get_matches.value_of("service"){
                Some(value) => value,
                None => "system",
            };
            let keyring = Keyring::new(&service, &key);
            match keyring.get_password(){
                Ok(value) => {
                    println!("{}", value);
                    std::process::exit(exitcode::OK);
                },
                Err(error) => {
                    eprintln!("Can't get value for key \"{}\". {}", &key, error);
                    std::process::exit(exitcode::NOINPUT);
                }
            };
        }else if let Some(set_matches) = &matches.subcommand_matches("set") {
            let key = match set_matches.value_of("key"){
                Some(key) => key,
                None => {
                    eprintln!("No key provided");
                    std::process::exit(exitcode::NOINPUT);
                },
            };
            let value = match set_matches.value_of("value"){
                Some(value) => value,
                None => {
                    eprintln!("No value provided");
                    std::process::exit(exitcode::NOINPUT);
                },
            };
            let service = match set_matches.value_of("service"){
                Some(service) => service,
                None => "system",
            };
            let keyring = Keyring::new(&service, &key);
            match keyring.set_password(&value){
                Ok(_resultado) => std::process::exit(exitcode::OK),
                Err(error) => {
                    eprintln!("Can't add the value. {}", error);
                    std::process::exit(exitcode::CANTCREAT);
                }
            }
        }else if let Some(delete_matches) = &matches.subcommand_matches("delete"){
            let key = match delete_matches.value_of("key"){
                Some(key) => key,
                None => {
                    eprintln!("No key provided");
                    std::process::exit(exitcode::NOINPUT);
                },
            };
            let service = match delete_matches.value_of("service"){
                Some(value) => value,
                None => "system",
            };
            let keyring = Keyring::new(&service, &key);
            match keyring.delete_password(){
                Ok(_resultado) => std::process::exit(exitcode::OK),
                Err(error) => {
                    eprintln!("Can't delete the key \"{}\". {}", &key, error);
                    std::process::exit(exitcode::UNAVAILABLE);
                }
            }
        } 
}
