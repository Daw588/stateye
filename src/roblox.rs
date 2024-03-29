use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct PlaceInfo {
	pub name: String,
	pub url: String
}

pub struct RobloxAPI {
	pub client: reqwest::blocking::Client,
	pub token: String
}

#[derive(Debug)]
pub struct AuthInfo {
	pub id: i64
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PresenceType {
	Offline = 0,
	Online = 1,
	InGame = 2,
	InStudio = 3
}

#[derive(Clone)]
pub struct UserPresence {
	pub presence_type: PresenceType,
	pub place_id: Option<i64>,
	pub universe_id: Option<i64>
}

impl RobloxAPI {
	pub fn get_user_auth_info(&self) -> Result<AuthInfo, Error> {
		/*
			Get information about the user from the auth token
			(specifically the userid so we can use it to get their presence)
		*/
		let response = self.client.get("https://users.roblox.com/v1/users/authenticated")
			.header("Cookie", self.token.as_str())
			.header("Accept", "application/json")
			.send();

		if !response.is_ok() {
			return Err(Error::new(ErrorKind::ConnectionRefused, "Something unexpected happen!"));
		}
			
		let roblox_auth = response
			.unwrap()
			.json::<serde_json::Value>()
			.unwrap();

		/*
			.unwrap()
			.json::<HashMap<String, String>>()
			.await;
		*/

		let id = roblox_auth["id"].as_i64().expect("Failed to fetch user id");
	
		return Ok(AuthInfo {
			id
		});
	}

	pub fn get_place_icon_url(&self, universe_id: i64) -> Result<String, Error> {
		let response = self.client.get(
			format!(
				"https://thumbnails.roblox.com/v1/games/icons?universeIds={}&size=512x512&format=Png&isCircular=false",
				universe_id
			)
		).send();

		if !response.is_ok() {
			return Err(Error::new(ErrorKind::ConnectionRefused, "Something unexpected happen!"));
		}

		let place_icon = response
			.unwrap()
			.json::<serde_json::Value>()
			.unwrap();
		
		let place_icon_url = &place_icon["data"][0]["imageUrl"];
		let place_icon_url = place_icon_url.as_str().unwrap().to_string();
	
		return Ok(place_icon_url);
	}

	pub fn get_place_info(&self, place_id: i64) -> Result<PlaceInfo, Error> {
		let response = self.client.get(format!("https://games.roblox.com/v1/games/multiget-place-details?placeIds={}", place_id))
			.header("Cookie", self.token.as_str())
			.send();

		if !response.is_ok() {
			return Err(Error::new(ErrorKind::ConnectionRefused, "Something unexpected happen!"));
		}

		let place_info = response
			.unwrap()
			.json::<serde_json::Value>()
			.unwrap();
		
		let place_info = &place_info[0];
		
		return Ok(PlaceInfo {
			name: place_info["name"].as_str().unwrap().to_string(),
			url: place_info["url"].as_str().unwrap().to_string()
		});
	}

	pub fn get_user_presence(&self, user_id: i64) -> Result<UserPresence, Error> {
		let response = self.client.post("https://presence.roblox.com/v1/presence/users")
			.header("Cookie", self.token.as_str())
			.header("Content-Type", "application/json")
			.body(format!("{{\"userIds\":[{}]}}", user_id))
			.send();

		if !response.is_ok() {
			return Err(Error::new(ErrorKind::ConnectionRefused, "Something unexpected happen!"));
		}

		let roblox_presence = response
			.unwrap()
			.json::<serde_json::Value>()
			.unwrap();

		let roblox_presence = &roblox_presence["userPresences"][0];

		let presence_type_id = roblox_presence["userPresenceType"].as_i64().unwrap();

		// Convert id to enum for presence type
		let presence_type = match presence_type_id {
			0 => PresenceType::Offline,
			1 => PresenceType::Online,
			2 => PresenceType::InGame,
			3 => PresenceType::InStudio,
			_ => PresenceType::Offline
		};

		let place_id = roblox_presence["placeId"].as_i64();
		let universe_id = roblox_presence["universeId"].as_i64();

		return Ok(UserPresence {
			presence_type,
			place_id,
			universe_id
		});
	}
}
