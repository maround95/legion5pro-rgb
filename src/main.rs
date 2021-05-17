use clap::{App, AppSettings, Arg, SubCommand};

use hidapi::{HidApi, HidDevice};

use std::error::Error;
use std::str::FromStr;

const DEVICE_INFO: (u16, u16, u16, u16) = (0x048d, 0xc965, 0xff89, 0x00cc);

fn parse_bytes_arg(arg: &str) -> Result<Vec<u8>, <u8 as FromStr>::Err> {
    arg.split(',').map(|b| b.parse::<u8>()).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("rgb")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("brightness")
                .help("Possible values: [1, 2], Default: 2")
                .takes_value(true)
                .short("b")
        )
        .arg(
            Arg::with_name("speed")
                .help("Possible values: [1, 2, 3, 4], Default: 2")
                .takes_value(true)
                .short("s")
        )
        .subcommand(
            SubCommand::with_name("static")
                .about("Choose same static color for all zones")
                .arg(
                    Arg::with_name("color")
                        .help("0-255 for each channel (R,G,B). Example: 255,0,0")
                        .index(1)
                        .required(true)
                ))
        .subcommand(
            SubCommand::with_name("zstatic")
                .about("Choose a static color for each of the 4 zones")
                .arg(
                    Arg::with_name("colors")
                        .help("List of 4 RGB triplets. Example: 255,0,0,255,255,0,0,0,255,255,128,0")
                        .index(1)
                        .required(true)
                ))
        .subcommand(
            SubCommand::with_name("breath")
                .about("Breath effect with same color choice for all zones")
                .arg(
                    Arg::with_name("color")
                        .help("0-255 for each channel (R,G,B). Example: 255,0,0")
                        .index(1)
                        .required(true)
                ))
        .subcommand(
            SubCommand::with_name("zbreath")
                .about("Breath effect with color choice for each of the 4 zones")
                .arg(
                    Arg::with_name("colors")
                        .help("List of 4 RGB triplets. Example: 255,0,0,255,255,0,0,0,255,255,128,0")
                        .index(1)
                        .required(true)
                ))
        .subcommand(SubCommand::with_name("smooth")
                .about("Smooth effect"))
        .subcommand(SubCommand::with_name("lwave")
                .about("Left Wave effect"))
        .subcommand(SubCommand::with_name("rwave")
                .about("Right Wave effect"))
        .get_matches();

    let mut payload: Vec<u8> = vec![0x0; 33];
    payload[0] = 0xcc;
    payload[1] = 0x16;

    payload[2] = match matches.subcommand_name() {
        Some(sub @ ("static" | "breath")) => {
            let matches = matches.subcommand_matches(sub).unwrap();
            let rgb: Vec<u8> = parse_bytes_arg(matches.value_of("color").unwrap())?;
            if rgb.len() != 3 { Err("Expected 1 RGB triplet")? }
            for i in (5 .. 17).step_by(3) {
                payload[i] = rgb[0];
                payload[i+1] = rgb[1];
                payload[i+2] = rgb[2];
            }
            if sub == "static" { 0x01 } else { 0x03 /*breath*/ }
        },
        Some(sub @ ("zstatic" | "zbreath")) => {
            let matches = matches.subcommand_matches(sub).unwrap();
            let rgb: Vec<u8> = parse_bytes_arg(matches.value_of("colors").unwrap())?;
            if rgb.len() != 12 { Err("Expected 4 RGB triplets")? }
            for i in 5 .. 17 {
                payload[i] = rgb[i-5];
            }
            if sub == "zstatic" { 0x01 } else { 0x03 /*breath*/ }
        },
        Some("smooth") => 0x06,
        Some("lwave") => { payload[19] = 0x1; 0x04 }
        Some("rwave") => { payload[18] = 0x1; 0x04 }
        _ => unreachable!(),
    };

    let bri: u8 = matches.value_of("brightness").map_or(Ok(2u8), |b| b.parse())?;
    if bri < 1 || bri > 2 { Err("Invalid Brightness. See help")? }
    let spd: u8 = matches.value_of("speed").map_or(Ok(2u8), |b| b.parse())?;
    if spd < 1 || spd > 4 { Err("Invalid Speed. See help")? }

    payload[3] = spd;
    payload[4] = bri;

    let api: HidApi = HidApi::new()?;

    let info = api.device_list()
        .find(|d| (d.vendor_id(), d.product_id(), d.usage_page(), d.usage()) == DEVICE_INFO)
        .ok_or("Error: Couldn't find device")?;

    let device: HidDevice = info.open_device(&api)?;

    device.send_feature_report(&payload[..])?;
    Ok(())
}
