use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::{time, fs};

use crate::config;

#[derive(Debug)]
pub struct Config {
	pub token: String,
	pub website: bool,
	pub player: bool,
	pub studio: bool
}

/// Converts string bool to actual bool.
/// 
/// `eg. "true" -> true`
/// 
/// `eg. "false" -> false`
fn str_to_bool(str: &str) -> bool {
	if str == "true" { true } else { false }
}

/// Generates program configuration from the config
/// file that is relative to where the program is located at.
/// 
/// eg. File `"./stateye.config"` -> `Config`
pub fn get_config() -> Config {
	// Find where our working directory is
	let working_dir_path = std::env::current_dir().unwrap();

	// Find the config file relative to our working directory
	let config_path = working_dir_path.join(config::CONFIG_FILE_NAME);
	let config_path = config_path.to_str().unwrap();

	// Default configuration in case file cannot be found
	let mut config = Config {
		token: String::new(),
		website: true,
		player: true,
		studio: true
	};

	// Read the config file
	let config_file = fs::read_to_string(config_path);

	/*
		If settings were read successfuly read keys and values,
		otherwise, use the default config.
	*/
	if config_file.is_ok() {
		println!("Loaded config from {}", config_path);

		// Since we checked with is_ok(), we can safely unwrap()
		let config_contents = config_file.unwrap();

		// Go through each line (line by line)
		for line in config_contents.lines() {
			/*
				Split key from value where the =
				character is located at.

				eg. "website=false"
				     ^^^^^^^ ^^^^^
					 0       1
			*/

			let split_line = line.split_once("=");
			if split_line.is_none() {
				/* 
					Skip over this line as it
					does not have a separator.

					This will prevent the program
					from crashing when blank lines
					are present in the config file.
				*/
				continue;
			}

			let key_value = split_line.unwrap();
			
			let key = key_value.0;
			let value = key_value.1;

			/*
				Set config key depending
				on the read key.

				eg. "token" will correspond to config.token
			*/

			match key {
				"token" => config.token = value.to_string(),
				"player" => config.player = str_to_bool(value),
				"studio" => config.studio = str_to_bool(value),
				"website" => config.website = str_to_bool(value),
				_ => {}
			}

			//println!("{}", line);
		}
	} else {
		println!("Config file not found, using defaults");
	}

	return config;
}

pub fn get_epoch_time() -> time::Duration {
	let start = time::SystemTime::now();
	
	let since_the_epoch = start
		.duration_since(time::UNIX_EPOCH)
		.expect("Time went backwards");

	return since_the_epoch;
}

pub struct ActivityInfo<'a> {
	pub discord_client: &'a mut DiscordIpcClient,
	pub details: &'a str,
	pub state: &'a str,
	pub big_icon_url: &'a str,
	pub small_icon_url: &'a str,
	pub buttons: Vec<activity::Button<'a>>,
	pub elapsed: i64
}

/// Sets Discord activity presence based on passed in arguments.
pub fn set_activity(activiy_info: ActivityInfo) {
	let mut activity = activity::Activity::new();
	let mut assets = activity::Assets::new();

	if !activiy_info.big_icon_url.is_empty() {
		assets = assets.large_image(activiy_info.big_icon_url);
	}

	if !activiy_info.small_icon_url.is_empty() {
		assets = assets.small_image(activiy_info.small_icon_url);
	}

	activity = activity.state(activiy_info.state);

	if !activiy_info.big_icon_url.is_empty() || !activiy_info.small_icon_url.is_empty() {
		activity = activity.assets(assets);
	} 

	if !activiy_info.buttons.is_empty() {
		activity = activity.buttons(activiy_info.buttons);
	}

	if !activiy_info.details.is_empty() {
		activity = activity.details(activiy_info.details);
	}

	let timestamps = activity::Timestamps::start(activity::Timestamps::new(), activiy_info.elapsed);

	activity = activity.timestamps(timestamps);

	activiy_info.discord_client.set_activity(activity).expect("Activity failed!");
}
