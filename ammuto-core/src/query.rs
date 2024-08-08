use crate::data::{Collection, Group, Media, Tag, User};
use uuid::Uuid;

///Represents a fully formed query to send to the database.
///You may manually create one of this, or use one of the other Query structs to build one in a safer, more verbose way
#[derive(Clone, PartialEq)]
pub struct DatabaseQuery {
    ///The type or table the query references
    pub query_type: QueryType,
    ///The conditions required for a match.
    pub conditions: Vec<QueryCondition>,
    ///The format for a return
    pub return_type: ReturnType,
    ///The randomly generated ID of the query
    pub id: Uuid,
}

trait QueryBuilder {
    ///Adds the condition to the query
    fn add_condition(&mut self, condition: QueryCondition);
    ///Queues a modifier to the next condition(s) to the query
    fn queue_modifier(&mut self, modifier: QueryModifier);
    ///Dequeues the next query modifier. Returns [`Option::None`] if the queue is empty
    fn dequeue_modifier(&mut self) -> Option<QueryModifier>;
    ///Begins a query that returns the quantity of matches
    fn count() -> Self;
    ///Begins a query that returns the objects that match the query
    fn all() -> Self;
    ///Completes the query and converts it into a generic [`DatabaseQuery`]
    fn finalise(self) -> DatabaseQuery;

    ///Requires the next condition fails
    fn not(&mut self) -> &mut Self {
        self.queue_modifier(QueryModifier::Not);
        self
    }
    ///Allows a match from either of the next two conditions. Does stack
    fn or(&mut self) -> &mut Self {
        self.queue_modifier(QueryModifier::Or);
        self
    }
    ///Requests objects with the specified ID
    fn with_id(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::Id(id));
        self
    }
    ///Requests objects with the specified name
    fn with_name(&mut self, name: String) -> &mut Self {
        self.add_condition(QueryCondition::NameIs(name));
        self
    }
    ///Requests objects with a name that contains the specified text
    fn with_name_containing(&mut self, text: String) -> &mut Self {
        self.add_condition(QueryCondition::NameContains(text));
        self
    }
    ///Requests objects with a name that fuzzily matches the specified text
    fn with_name_fuzzy(&mut self, text: String) -> &mut Self {
        self.add_condition(QueryCondition::NameFuzzy(text));
        self
    }
    ///Defines a maximum amount of objects
    fn with_amount(&mut self, amount: u32) -> &mut Self {
        self.add_condition(QueryCondition::Amount(amount));
        self
    }
    ///Requests the specified amount of matches to be ignored before returning
    fn skip_entries(&mut self, count: u32) -> &mut Self {
        self.add_condition(QueryCondition::Skip(count));
        self
    }
    ///Requests objects that share a group/collection with the object with the specified ID\
    ///Only checks objects of the same type
    fn related_to(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::IsRelatedTo(id));
        self
    }
    ///Requests objects that have the specified custom property
    fn has_custom_property(&mut self, property: String) -> &mut Self {
        self.add_condition(QueryCondition::HasCustomProperty(property));
        self
    }
}

///Describes a query about Tags
pub struct TagQuery {
    return_type: ReturnType,
    conditions: Vec<QueryCondition>,
    accumulator: Option<QueryCondition>,
    modifiers: Vec<QueryModifier>,
}
impl QueryBuilder for TagQuery {
    ///Begins a query that returns the quantity of matches
    fn count() -> Self {
        Self {
            return_type: ReturnType::Quantity,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Begins a query that returns the objects that match the query
    fn all() -> Self {
        Self {
            return_type: ReturnType::Objects,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Completes the query and converts it into a generic [`DatabaseQuery`]
    fn finalise(self) -> DatabaseQuery {
        DatabaseQuery {
            query_type: QueryType::Tags,
            conditions: self.conditions,
            return_type: self.return_type,
            id: Uuid::new_v4(),
        }
    }

    fn add_condition(&mut self, condition: QueryCondition) {
        if let Some(modifier) = self.dequeue_modifier() {
            match modifier {
                QueryModifier::Not => self.add_condition(QueryCondition::Not(Box::new(condition))),
                QueryModifier::Or => {
                    if let Some(first) = self.accumulator.take() {
                        self.add_condition(QueryCondition::Or(Box::new(Or(first, condition))));
                    } else {
                        self.queue_modifier(QueryModifier::Or);
                        self.accumulator = Some(condition);
                    }
                }
            };
        } else {
            self.conditions.push(condition);
        }
    }

    fn queue_modifier(&mut self, modifier: QueryModifier) {
        self.modifiers.push(modifier)
    }

    fn dequeue_modifier(&mut self) -> Option<QueryModifier> {
        self.modifiers.pop()
    }
}
impl TagQuery {
    ///Requests tags that are an alias to the specified tag
    pub fn alias_of(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::IsAliasOf(id));
        self
    }
    ///Requests tags that are in the specified group
    pub fn in_group(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::IsInGroup(id));
        self
    }
    ///Requests objects that were created by the specified user
    pub fn created_by(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::CreatedBy(id));
        self
    }
}
///Describes a query about Media
pub struct MediaQuery {
    return_type: ReturnType,
    conditions: Vec<QueryCondition>,
    accumulator: Option<QueryCondition>,
    modifiers: Vec<QueryModifier>,
}
impl QueryBuilder for MediaQuery {
    ///Begins a query that returns the quantity of matches
    fn count() -> Self {
        Self {
            return_type: ReturnType::Quantity,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Begins a query that returns the objects that match the query
    fn all() -> Self {
        Self {
            return_type: ReturnType::Objects,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Completes the query and converts it into a generic [`DatabaseQuery`]
    fn finalise(self) -> DatabaseQuery {
        DatabaseQuery {
            query_type: QueryType::Media,
            conditions: self.conditions,
            return_type: self.return_type,
            id: Uuid::new_v4(),
        }
    }

    fn add_condition(&mut self, condition: QueryCondition) {
        if let Some(modifier) = self.dequeue_modifier() {
            match modifier {
                QueryModifier::Not => self.add_condition(QueryCondition::Not(Box::new(condition))),
                QueryModifier::Or => {
                    if let Some(first) = self.accumulator.take() {
                        self.add_condition(QueryCondition::Or(Box::new(Or(first, condition))));
                    } else {
                        self.queue_modifier(QueryModifier::Or);
                        self.accumulator = Some(condition);
                    }
                }
            };
        } else {
            self.conditions.push(condition);
        }
    }

    fn queue_modifier(&mut self, modifier: QueryModifier) {
        self.modifiers.push(modifier)
    }

    fn dequeue_modifier(&mut self) -> Option<QueryModifier> {
        self.modifiers.pop()
    }
}
impl MediaQuery {
    ///Requests objects that were created by the specified user
    pub fn created_by(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::CreatedBy(id));
        self
    }
    ///Requests media that is in the specified collection
    pub fn in_collection(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::IsInCollection(id));
        self
    }
}
///Describes a query about Groups
pub struct GroupQuery {
    return_type: ReturnType,
    conditions: Vec<QueryCondition>,
    accumulator: Option<QueryCondition>,
    modifiers: Vec<QueryModifier>,
}
impl QueryBuilder for GroupQuery {
    ///Begins a query that returns the quantity of matches
    fn count() -> Self {
        Self {
            return_type: ReturnType::Quantity,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Begins a query that returns the objects that match the query
    fn all() -> Self {
        Self {
            return_type: ReturnType::Objects,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Completes the query and converts it into a generic [`DatabaseQuery`]
    fn finalise(self) -> DatabaseQuery {
        DatabaseQuery {
            query_type: QueryType::Groups,
            conditions: self.conditions,
            return_type: self.return_type,
            id: Uuid::new_v4(),
        }
    }

    fn add_condition(&mut self, condition: QueryCondition) {
        if let Some(modifier) = self.dequeue_modifier() {
            match modifier {
                QueryModifier::Not => self.add_condition(QueryCondition::Not(Box::new(condition))),
                QueryModifier::Or => {
                    if let Some(first) = self.accumulator.take() {
                        self.add_condition(QueryCondition::Or(Box::new(Or(first, condition))));
                    } else {
                        self.queue_modifier(QueryModifier::Or);
                        self.accumulator = Some(condition);
                    }
                }
            };
        } else {
            self.conditions.push(condition);
        }
    }

    fn queue_modifier(&mut self, modifier: QueryModifier) {
        self.modifiers.push(modifier)
    }

    fn dequeue_modifier(&mut self) -> Option<QueryModifier> {
        self.modifiers.pop()
    }
}
impl GroupQuery {
    ///Requests objects that were created by the specified user
    pub fn created_by(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::CreatedBy(id));
        self
    }
    ///Requests groups that include the specified tag
    pub fn contains_media(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::Contains(id));
        self
    }
}
///Describes a query about Collections
pub struct CollectionQuery {
    return_type: ReturnType,
    conditions: Vec<QueryCondition>,
    accumulator: Option<QueryCondition>,
    modifiers: Vec<QueryModifier>,
}
impl QueryBuilder for CollectionQuery {
    ///Begins a query that returns the quantity of matches
    fn count() -> Self {
        Self {
            return_type: ReturnType::Quantity,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Begins a query that returns the objects that match the query
    fn all() -> Self {
        Self {
            return_type: ReturnType::Objects,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Completes the query and converts it into a generic [`DatabaseQuery`]
    fn finalise(self) -> DatabaseQuery {
        DatabaseQuery {
            query_type: QueryType::Collections,
            conditions: self.conditions,
            return_type: self.return_type,
            id: Uuid::new_v4(),
        }
    }

    fn add_condition(&mut self, condition: QueryCondition) {
        if let Some(modifier) = self.dequeue_modifier() {
            match modifier {
                QueryModifier::Not => self.add_condition(QueryCondition::Not(Box::new(condition))),
                QueryModifier::Or => {
                    if let Some(first) = self.accumulator.take() {
                        self.add_condition(QueryCondition::Or(Box::new(Or(first, condition))));
                    } else {
                        self.queue_modifier(QueryModifier::Or);
                        self.accumulator = Some(condition);
                    }
                }
            };
        } else {
            self.conditions.push(condition);
        }
    }

    fn queue_modifier(&mut self, modifier: QueryModifier) {
        self.modifiers.push(modifier)
    }

    fn dequeue_modifier(&mut self) -> Option<QueryModifier> {
        self.modifiers.pop()
    }
}
impl CollectionQuery {
    ///Requests objects that were created by the specified user
    pub fn created_by(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::CreatedBy(id));
        self
    }
    ///Requests collections that include the specified media
    pub fn contains_media(&mut self, id: u64) -> &mut Self {
        self.add_condition(QueryCondition::Contains(id));
        self
    }
}
///Describes a query about Users
pub struct UserQuery {
    return_type: ReturnType,
    conditions: Vec<QueryCondition>,
    accumulator: Option<QueryCondition>,
    modifiers: Vec<QueryModifier>,
}
impl QueryBuilder for UserQuery {
    ///Begins a query that returns the quantity of matches
    fn count() -> Self {
        Self {
            return_type: ReturnType::Quantity,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Begins a query that returns the objects that match the query
    fn all() -> Self {
        Self {
            return_type: ReturnType::Objects,
            conditions: Vec::new(),
            accumulator: None,
            modifiers: Vec::new(),
        }
    }

    ///Completes the query and converts it into a generic [`DatabaseQuery`]
    fn finalise(self) -> DatabaseQuery {
        DatabaseQuery {
            query_type: QueryType::Users,
            conditions: self.conditions,
            return_type: self.return_type,
            id: Uuid::new_v4(),
        }
    }

    fn add_condition(&mut self, condition: QueryCondition) {
        if let Some(modifier) = self.dequeue_modifier() {
            match modifier {
                QueryModifier::Not => self.add_condition(QueryCondition::Not(Box::new(condition))),
                QueryModifier::Or => {
                    if let Some(first) = self.accumulator.take() {
                        self.add_condition(QueryCondition::Or(Box::new(Or(first, condition))));
                    } else {
                        self.queue_modifier(QueryModifier::Or);
                        self.accumulator = Some(condition);
                    }
                }
            };
        } else {
            self.conditions.push(condition);
        }
    }

    fn queue_modifier(&mut self, modifier: QueryModifier) {
        self.modifiers.push(modifier)
    }

    fn dequeue_modifier(&mut self) -> Option<QueryModifier> {
        self.modifiers.pop()
    }
}

///Describes each table, or more generically data type, a query may operate on
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[non_exhaustive]
pub enum QueryType {
    Tags,
    Media,
    Groups,
    Collections,
    Users,
    Other(String),
}

///Describes queued modifiers for a query condition
pub enum QueryModifier {
    ///Requires the next loaded condition to fail
    Not,
    ///Allows a match from either of the next two loaded conditions
    Or,
}
///Two query conditions that returns positive if at least one condition matches.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Or(QueryCondition, QueryCondition);
///Describes each of the operators that can be found within a query. By default, all conditions are AND.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum QueryCondition {
    ///Requires the condition to fail
    Not(Box<QueryCondition>),
    ///Allows a match from either condition's success
    Or(Box<Or>),
    ///Requests objects with the specified ID
    Id(u64),
    ///Requests objects with a name that perfectly matches the provided string
    NameIs(String),
    ///Requests objects with a name that contains the provided string
    NameContains(String),
    ///Requests objects with a name that fuzzy matches the provided string
    NameFuzzy(String),
    ///Requests a maximum amount of objects
    Amount(u32),
    ///Requests the specified number of objects to be skipped before returning
    Skip(u32),
    ///Requests the objects related to an object with the specified ID (Shares a collection or group)
    IsRelatedTo(u64),
    ///Requests the tags that are an alias for the specified tag
    IsAliasOf(u64),
    ///Requests the tags that are within the specified group
    IsInGroup(u64),
    ///Requests the media that have the specified custom property
    HasCustomProperty(String),
    ///Requests the media that are within the specified collection
    IsInCollection(u64),
    ///Requests the objects that were created by the specified user
    CreatedBy(u64),
    ///Requests the users that have the specified permissions
    HasPermissions(u64),
    ///Requests the objects that contain the specified id
    Contains(u64),
}
///Describes each format a query may wish to receive the data back as
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum ReturnType {
    ///The query requests the amount of items that match
    Quantity,
    ///The query requests the items as memory objects
    Objects,
}

///An error related to the query\
/// **UnknownErrorType** - The type used to represent an error that is not covered by this enum.
#[derive(PartialEq, Clone, Debug)]
pub enum QueryError<UnknownErrorType> {
    ///The query was for a different table/type
    BadType(QueryType),
    ///The query used an operator that was not supported by the database
    Unsupported(QueryCondition),
    ///The user is not authorised to request this data
    Unauthorised,
    ///The query requests conditions that can never match
    ImpossibleQuery,
    ///The query requests a condition that does not apply to the operating type
    BadCondition(QueryCondition),
    ///The query was not complete
    Incomplete,
    ///The database terminated the connection
    ConnectionFault(u16),
    ///The database did not respond in time
    ConnectionTimeout,
    ///The adapter did not understand the response from the server
    BadResponse,
    ///An error occurred that is unique to this database form
    Other(UnknownErrorType),
    ///There is no database attached to this Ammuto instance
    NoDatabase,
}
///Describes the 3 states a property from a database can be in
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum DatabaseValue<T> {
    ///A valid value was assigned to the property
    Some(T),
    ///No value was assigned to the property
    None,
    ///The user is not authorised to know the value of this property
    Unauthorised,
}

///Describes the possible return types from a database
pub enum DatabaseResult {
    ///The database provided a list of tags
    Tags(Vec<Tag>),
    ///The database provided a list of media objects
    Media(Vec<Media>),
    ///The database provided a list of tag groups
    Groups(Vec<Group>),
    ///The database provided a list of media collections
    Collections(Vec<Collection>),
    ///The database provided a list of users
    Users(Vec<User>),
    ///The database provided an integer
    Integer(u32),
}
