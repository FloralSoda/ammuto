use crate::query::DatabaseValue;

pub enum MediaType {
	Image,
	Video,
	Audio,
	Custom(String)
}

///Represents an organisation tag
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

///Represents a piece of media, or any other resource, stored as a file in the database
pub struct Media {
	///The internal ID for the media
	pub id: DatabaseValue<u64>,
	///The display name of the media
	pub name: DatabaseValue<String>,
	///The user that created the media
	pub created_by: DatabaseValue<u64>,
	///The tags applied to the media
	pub tags: DatabaseValue<Vec<u64>>,
}

///Represents a group of tags
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
