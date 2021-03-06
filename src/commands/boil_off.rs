extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches, ArgGroup, ErrorKind};
use crate::AppSubCommand;
pub use crate::calculators::boil_off::BoilOff;

impl AppSubCommand for BoilOff {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>{
        app.subcommand(SubCommand::with_name("boil_off")
            .version("0.1")
            .about("Calculates how much you need to dilute or boil down your wort volume to hit a certain gravity")
            .arg(Arg::with_name("wort_volume")
                    .long("wort_volume")
                    .short("w")
                    .help("Wort Volume")
                    .required(true)
                    .takes_value(true))
            .arg(Arg::with_name("current_gravity")
                    .long("current_gravity")
                    .short("c")
                    .help("Current Gravity")
                    .required(true)
                    .takes_value(true))
            .arg(Arg::with_name("desired_gravity")
                    .long("desired_gravity")
                    .short("d")
                    .help("Desired Gravity")
                    .takes_value(true))
            .arg(Arg::with_name("target_volume")
                    .long("target_volume")
                    .short("t")
                    .help("Target Volume")
                    .takes_value(true))
            .group(ArgGroup::with_name("desired")
                    .args(&["target_volume", "desired_gravity"])
                    .required(true))
        )
    }

    fn do_matches<'c>(&self, matches: &ArgMatches<'c>){
        if let Some(ref sub_matches) = matches.subcommand_matches("boil_off") {
            let wort_volume = value_t!(sub_matches, "wort_volume", f32).unwrap_or_else(|e| e.exit());
            let current_gravity = value_t!(sub_matches, "current_gravity", f32).unwrap_or_else(|e| e.exit());

            println!("Wort Volume: {}", wort_volume);
            println!("Current Gravity: {}", current_gravity);

            match value_t!(sub_matches, "desired_gravity", f32) {
                Ok(desired_gravity) => {
                    let new_volume = self.calculate_boileoff_new_volume(wort_volume, current_gravity, desired_gravity);
                    println!("New Volume: {}", new_volume);
                    println!("Difference: {}", new_volume - wort_volume);
                },
                Err(ref e) if e.kind == clap::ErrorKind::ArgumentNotFound => {
                    let target_volume = value_t!(sub_matches, "target_volume", f32).unwrap_or_else(|er| er.exit());
                    let new_gravity = self.calculate_boileoff_new_gravity(wort_volume, current_gravity, target_volume);
                    println!("New Gravity: {}", new_gravity);
                    println!("Difference: {}", new_gravity - current_gravity);
                },
                Err(e) => e.exit()
            }
        }
    }
}

