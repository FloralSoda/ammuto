use std::{collections::HashSet, rc::Rc};

use uuid::Uuid;

use crate::query::DatabaseValue;

///Represents an organisational tag
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Tag {
    id: u64,
    name: DatabaseValue<String>,
    created_by: DatabaseValue<u64>,
    aliases: DatabaseValue<Vec<u64>>,
}
impl Tag {
    ///The unique 64-bit identifier for the tag
    pub fn id(&self) -> u64 {
        self.id
    }
    ///The string name of the tag
    pub fn name(&self) -> &DatabaseValue<String> {
        &self.name
    }
    ///The list of identifiers for the tags this tag is linked to
    pub fn aliases(&self) -> &DatabaseValue<Vec<u64>> {
        &self.aliases
    }
    ///The id of the user that created this tag
    pub fn created_by(&self) -> &DatabaseValue<u64> {
        &self.created_by
    }
    ///Creates a new tag with a name and aliases
    pub fn new(name: String, created_by: u64, aliases: Vec<u64>) -> Self {
        Self {
            id: Uuid::new_v4().as_u64_pair().0, //The second 64 bits don't matter to us
            name: DatabaseValue::Some(name),
            created_by: DatabaseValue::Some(created_by),
            aliases: DatabaseValue::Some(aliases),
        }
    }
}
///Represents a group of media (albums, series, comics etc.)
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Collection {
    id: u64,
    name: DatabaseValue<String>,
    created_by: DatabaseValue<u64>,
    contained_media: DatabaseValue<Vec<u64>>,
}
impl Collection {
    ///The unique 64-bit identifier for the tag
    pub fn id(&self) -> u64 {
        self.id
    }
    ///The string name of the tag
    pub fn name(&self) -> &DatabaseValue<String> {
        &self.name
    }
    ///The ordered list of media that this collection represents
    pub fn contained_media(&self) -> &DatabaseValue<Vec<u64>> {
        &self.contained_media
    }
    ///The id of the user that created this tag
    pub fn created_by(&self) -> &DatabaseValue<u64> {
        &self.created_by
    }
    ///Creates a new collection with a name and contained files
    pub fn new(name: String, created_by: u64, contained_media: Vec<u64>) -> Self {
        Self {
            id: Uuid::new_v4().as_u64_pair().0, //The second 64 bits don't matter to us
            name: DatabaseValue::Some(name),
            created_by: DatabaseValue::Some(created_by),
            contained_media: DatabaseValue::Some(contained_media),
        }
    }
}
///Represents a group of tags
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Group {
    id: u64,
    name: DatabaseValue<String>,
    created_by: DatabaseValue<u64>,
    contained_tags: DatabaseValue<Vec<u64>>,
}
impl Group {
    ///The unique 64-bit identifier for the tag
    pub fn id(&self) -> u64 {
        self.id
    }
    ///The string name of the tag
    pub fn name(&self) -> &DatabaseValue<String> {
        &self.name
    }
    ///The list of tags that this group represents
    pub fn contained_tags(&self) -> &DatabaseValue<Vec<u64>> {
        &self.contained_tags
    }
    ///The id of the user that created this tag
    pub fn created_by(&self) -> &DatabaseValue<u64> {
        &self.created_by
    }
    ///Creates a new group with a name and tags
    pub fn new(name: String, created_by: u64, contained_tags: Vec<u64>) -> Self {
        Self {
            id: Uuid::new_v4().as_u64_pair().0, //The second 64 bits don't matter to us
            name: DatabaseValue::Some(name),
            created_by: DatabaseValue::Some(created_by),
            contained_tags: DatabaseValue::Some(contained_tags),
        }
    }
}

///Represents a collection of properties to attach to a media object
pub trait MediaProperties {
    fn as_hash_set(&self) -> HashSet<String, String>;
    fn load_hash_set(&mut self, set: HashSet<String, String>);
}

///Represents a media object
pub struct Media {
    id: u64,
    custom_properties: DatabaseValue<Rc<dyn MediaProperties>>,
    extension: DatabaseValue<String>,
    created_by: DatabaseValue<u64>,
}
impl Media {
    ///The unique 64-bit identifier for the tag
    pub fn id(&self) -> u64 {
        self.id
    }
    ///The string name of the tag
    pub fn extension(&self) -> &DatabaseValue<String> {
        &self.extension
    }
    ///The list of tags that this group represents
    pub fn custom_properties(&self) -> &DatabaseValue<Rc<dyn MediaProperties>> {
        &self.custom_properties
    }
    ///The id of the user that created this tag
    pub fn created_by(&self) -> &DatabaseValue<u64> {
        &self.created_by
    }
    ///Creates a new reference to a file with the extension and creator
    pub fn new(
        extension: String,
        created_by: u64,
        custom_properties: Option<Rc<dyn MediaProperties>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().as_u64_pair().0, //The second 64 bits don't matter to us
            extension: DatabaseValue::Some(extension),
            created_by: DatabaseValue::Some(created_by),
            custom_properties: (if let Some(properties) = custom_properties {
                DatabaseValue::Some(properties)
            } else {
                DatabaseValue::None
            }),
        }
    }
}

///Represents a user
pub struct User {
    id: u64,
    name: DatabaseValue<String>,
    passhash: DatabaseValue<String>,
}
impl User {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &DatabaseValue<String> {
        &self.name
    }
    pub fn passhash(&self) -> &DatabaseValue<String> {
        &self.passhash
    }
    ///Creates a new User out of their name and secure passhash
    pub fn new(name: String, passhash: String) -> Self {
        Self {
            id: Uuid::new_v4().as_u64_pair().0, //The second 64 bits don't matter to us
            name: DatabaseValue::Some(name),
            passhash: DatabaseValue::Some(passhash),
        }
    }
}
