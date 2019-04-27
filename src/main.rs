use rand::Rng;
use clap::{Arg, App, ArgMatches};

const SUPERINTENDENT_QUOTES: [&str; 43] = [
    "TOLL ZONE. PLEASE, SLOW DOWN.",
    "FINAL NOTICE, BILL PAST DUE",
    "ATTENTION, TRAVELER! LOST ITEM CAN BE CLAIMED ON...LOWER LEVELS.",
    "ATTENTION PARTY OF ONE, COME IMMEDIATELY TO LOST AND FOUND! LOWER LEVELS.",
    "LOST-AND-FOUND ITEM! CALCULATING TRAJECTORY...",
    "OPENING WINDOW SHUTTER!",
    "KEEP IT CLEAN! RESPECT PUBLIC PROPERTY!",
    "BRIDGE TOLL ACCEPTED. HAVE A PLEASANT TRIP",
    "LOST-AND-FOUND ITEM! DISABLING SECURITY SHUTTER!",
    "WELCOME. ACCESS GRANTED.",
    "WARNING: HITCHHIKERS MAY BE ESCAPED CONVICTS.",
    " WELCOME. ACCESS GRANTED.",
    "CRIME DOESN'T PAY.",
    "CRIME SCENE. RESTRICTED ENTRY",
    "ICY CONDITIONS. CHAINS REQUIRED.",
    "CAUTION, TRAVELER! CAUTION!",
    "CRIME DOESN'T PAY. GOOD CITIZENS DO THEIR PART.",
    "WELCOME. ACCESS GRANTED.",
    "ELEVATOR UP. NEXT STOP:",
    "SPAY AND NEUTER YOUR PETS. ALL DOGS MUST BE KEPT ON LEASH!",
    "CONSTRUCTION AHEAD. EXPECT DELAYS.",
    "EMERGENCY SHUTDOWN INITIATED",
    "MOTOR-VEHICLE CRASH! SEARCHING N.M.P.D DATABASE...NO MATCH!",
    "WELCOME. ACCESS GRANTED",
    "KEEP IT CLEAN.",
    "RESPECT PUBLIC PROPERTY.",
    "PLEASE WALK.",
    "PLEASE REMAIN CALM.",
    "BEACH OPEN FOR WATER CONTACT",
    "YIELD TO PEDESTRIANS",
    "RECYCLING! DO YOUR PART!",
    "YOUR TAX DOLLARS AT WORK",
    "LOOK BOTH WAYS BEFORE CROSSING",
    "IN CASE OF FIRE, USE STAIRS",
    "PARDON OUR DUST",
    "PLEASE REMAIN CALM",
    "GOOD CITIZENS CONSERVE FUEL",
    "NO PARKING AT ANY TIME",
    "TURN LIGHTS OFF PRIOR TO DEPARTURE",
    "VEHICLES MAKE FREQUENT STOPS",
    "OBEY POSTED LIMITS!",
    "PLEASE REMAIN CALM!",
    "YIELD TO UPHILL TRAFFIC!",
];

const PREPEND_COMMAND: &str = "prepend";
const APPEND_COMMAND: &str = "append";

fn main() {
    let options: ArgMatches = get_options();

    if let Some(option_value) = options.value_of(PREPEND_COMMAND) {
        println!("{}", option_value);
    }

    println!("{}", SUPERINTENDENT_QUOTES[rand::thread_rng().gen_range(0, SUPERINTENDENT_QUOTES.len())]);

    if let Some(option_value) = options.value_of(APPEND_COMMAND) {
        println!("{}", option_value);
    }
}

fn get_options() -> ArgMatches<'static> {
    let prepend: Arg = Arg::with_name(PREPEND_COMMAND)
        .help("Set a string that will be printed before the quote.")
        .long(PREPEND_COMMAND)
        .short("p")
        .takes_value(true);

    let append: Arg = Arg::with_name(APPEND_COMMAND)
        .help("Set a string that will be printed after the quote.")
        .long(APPEND_COMMAND)
        .short("a")
        .takes_value(true);

    App::new("superintendent-quotes")
        .version("1.0")
        .about("Get a random Superintendent quote.")
        .arg(prepend)
        .arg(append)
        .get_matches()
}
