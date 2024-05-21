use serde::{de::DeserializeOwned, Deserialize};

use crate::query::DatabaseValue;

///Represents an organisation tag
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Tag {
	///The internal ID for the tag
	pub id: DatabaseValue<u64>,
	///The display name for the tag
	pub name: DatabaseValue<String>,
	///Has a value if the tag is an alias, otherwise it is a standalone tag
	pub alias: DatabaseValue<u64>,
	///The user that created the tag
	pub created_by: DatabaseValue<u64>,
}

///Represents a piece of media, or any other resource, stored as a file in the database\
///This should be implemented for each form of file you wish to handle separately (image, video, etc. or jpg, png, mp4, avi etc.)
pub trait Media : Sized { //Todo: Work out how to make media generic so that any data can be represented and fetched from the database
	///The internal ID for the media
	fn id(&self) -> DatabaseValue<u64>;
	///The display name of the media
	fn name(&self) -> DatabaseValue<String>;
	///The user that created the media
	fn created_by(&self) -> DatabaseValue<u64>;
	///The tags applied to the media
	fn tags(&self) -> DatabaseValue<Vec<u64>>;
	fn from_properties(properties: &[MediaProperty]) -> Result<Self,MediaError>;
}

//Flow:
//Database adapter takes care of what data gets sent in
//Database adapter needs to package that into the library
//Library needs to accept the database adapter's custom properties
//Library needs to push the generic and custom properties to the frontend
//Frontend needs to be able to identify custom properties and utilise them in a type safe, memory safe way. 
//
//Library needs to be able to accept either compile time or runtime defined properties
//[ Compile-time is the job of traits and runtime is the job of.. hashmaps? ]
//These dynamically defined properties need to be serialised so they can be sent generically via an enum
//[ This is done by serde ]
//Identifying custom properties is handled by deserializing them
//What do we serialize them to?
//



///Errors returned when trying to construct a media object
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum MediaError<T: DeserializeOwned> {
	///The media had a recognised property with nonsensical data
	MalformedProperty(MediaProperty<T>),
	///The media was missing certain required properties
	MissingProperties(Vec<String>),
}
///A generalised media property stored as key and string-serialised value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MediaProperty<T> where T: DeserializeOwned { 
	pub key: String,
	pub value: T
}

///Represents generic media storage. 
pub struct GenericMedia {
	///The internal ID for the media
	pub id: DatabaseValue<u64>,
	///The display name of the media
	pub name: DatabaseValue<String>,
	///The user that created the media[]
	pub created_by: DatabaseValue<u64>,
	///The tags applied to the media
	pub tags: DatabaseValue<Vec<u64>>,
	///The form of media represented by this entry
	pub form: DatabaseValue<String>,
}
impl Media for GenericMedia {
	fn id(&self) -> DatabaseValue<u64> {
		self.id
	}

	fn name(&self) -> DatabaseValue<String> {
		self.name
	}

	fn created_by(&self) -> DatabaseValue<u64> {
		self.created_by
	}

	fn tags(&self) -> DatabaseValue<Vec<u64>> {
		self.tags
	}
	fn from_properties(properties: &[MediaProperty]) -> Result<Self,MediaError> {
		let unsure_id = None;
		let unsure_name = None;
		let unsure_created_by = None;
		let unsure_tags = None;
		let unsure_form = None;

		for property in properties {
			match property.key {
				_ => {},
				String::from("id") => { 
					
				}
			}
		}

		let id;
		let name;
		let created_by;
		let tags;
		let unsure_form;

		
	}
}

///Represents a group of tags
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Group {
	///The internal ID for the group
	pub id: DatabaseValue<u64>,
	///The display name of the group
	pub name: DatabaseValue<String>,
	///The tags organised under the group
	pub children: DatabaseValue<Vec<u64>>,
	///The user that created the group
	pub created_by: DatabaseValue<u64>
}

///Represents a group of media
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Collection {
	///The internal ID for the collection
	pub id: DatabaseValue<u64>,
	///The display name of the collection
	pub name: DatabaseValue<String>,
	///The media organised under the collection
	pub children: DatabaseValue<Vec<u64>>,
	///The user that created the collection
	pub created_by: DatabaseValue<u64>
}

///Represents a user's data
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct User {
	///The internal ID for the user
	pub id: DatabaseValue<u64>,
	///The display name of the user
	pub name: DatabaseValue<String>,
	///The hash for the user. This should not be provided to the client
	pub password_hash: DatabaseValue<Vec<u8>>,
	///The permissions for the user as a 64-bit integer
	pub permissions: DatabaseValue<u64>
}
