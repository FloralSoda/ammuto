///Represents a fully formed query to send to the database.
///You may manually create one of this, or use one of the other Query structs to build one in a safer, more verbose way
pub struct DatabaseQuery {
	///The type or table the query references
	pub query_type: QueryType,
	///The conditions required for a match.
	pub conditions: Vec<QueryCondition>,
	///The format for a return
	pub return_type: ReturnType
} 

trait QueryBuilder {
	///Adds the condition to the query
	fn add_condition(&mut self, condition: QueryCondition);
	///Begins a query that returns the quantity of matches
	fn count() -> Self;
	///Begins a query that returns the objects that match the query
	fn all() -> Self;
	///Completes the query and converts it into a generic [`DatabaseQuery`]
	fn finalise(self) -> DatabaseQuery;

	///Requests the next condition fails
	fn not(&mut self) -> &mut Self {
		self.add_condition(QueryCondition::Not);
		self
	}
	///Requests objects with the specified ID
	fn with_id(&mut self, id: u64) -> &mut Self {
		self.add_condition(QueryCondition::ID(id));
		self
	}
	///Requests objects with the specified name
	fn matching_name(&mut self, name: String) -> &mut Self {
		self.add_condition(QueryCondition::NameIs(name));
		self
	}
	///Requests objects with a name that contains the specified text
	fn with_name_containing_text(&mut self, text: String) -> &mut Self {
		self.add_condition(QueryCondition::NameContains(text));
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
	///Requests objects that are related to the object with the specified ID (share a group/collection)
	fn related_to(&mut self, id: u64) -> &mut Self {
		self.add_condition(QueryCondition::IsRelatedTo(id));
		self
	}
}

///Describes a query about Tags
pub struct TagQuery {
	return_type: ReturnType,
	conditions: Vec<QueryCondition>
}
impl QueryBuilder for TagQuery {
	///Begins a query that returns the quantity of matches
	fn count() -> Self {
		Self {
			return_type: ReturnType::Quantity,
			conditions: Vec::new()
		}
	}

	///Begins a query that returns the objects that match the query
	fn all() -> Self {
		Self {
			return_type: ReturnType::Objects,
			conditions: Vec::new()
		}
	}

	///Completes the query and converts it into a generic [`DatabaseQuery`]
	fn finalise(self) -> DatabaseQuery {
		DatabaseQuery {
			query_type: QueryType::Tags,
			conditions: self.conditions,
			return_type: self.return_type
		}
	}

	fn add_condition(&mut self, condition: QueryCondition) {
		self.conditions.push(condition);
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
	conditions: Vec<QueryCondition>
}
impl QueryBuilder for MediaQuery {
	///Begins a query that returns the quantity of matches
	fn count() -> Self {
		Self {
			return_type: ReturnType::Quantity,
			conditions: Vec::new()
		}
	}

	///Begins a query that returns the objects that match the query
	fn all() -> Self {
		Self {
			return_type: ReturnType::Objects,
			conditions: Vec::new()
		}
	}

	///Completes the query and converts it into a generic [`DatabaseQuery`]
	fn finalise(self) -> DatabaseQuery {
		DatabaseQuery {
			query_type: QueryType::Media,
			conditions: self.conditions,
			return_type: self.return_type
		}
	}

	fn add_condition(&mut self, condition: QueryCondition) {
		self.conditions.push(condition);
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
	conditions: Vec<QueryCondition>
}
impl QueryBuilder for GroupQuery {
	///Begins a query that returns the quantity of matches
	fn count() -> Self {
		Self {
			return_type: ReturnType::Quantity,
			conditions: Vec::new()
		}
	}

	///Begins a query that returns the objects that match the query
	fn all() -> Self {
		Self {
			return_type: ReturnType::Objects,
			conditions: Vec::new()
		}
	}

	///Completes the query and converts it into a generic [`DatabaseQuery`]
	fn finalise(self) -> DatabaseQuery {
		DatabaseQuery {
			query_type: QueryType::Groups,
			conditions: self.conditions,
			return_type: self.return_type
		}
	}

	fn add_condition(&mut self, condition: QueryCondition) {
		self.conditions.push(condition);
	}
}
impl GroupQuery {
	///Requests objects that were created by the specified user
	pub fn created_by(&mut self, id: u64) -> &mut Self {
		self.add_condition(QueryCondition::CreatedBy(id));
		self
	}
}
///Describes a query about Collections
pub struct CollectionQuery {
	return_type: ReturnType,
	conditions: Vec<QueryCondition>
}
impl QueryBuilder for CollectionQuery {
	///Begins a query that returns the quantity of matches
	fn count() -> Self {
		Self {
			return_type: ReturnType::Quantity,
			conditions: Vec::new()
		}
	}

	///Begins a query that returns the objects that match the query
	fn all() -> Self {
		Self {
			return_type: ReturnType::Objects,
			conditions: Vec::new()
		}
	}

	///Completes the query and converts it into a generic [`DatabaseQuery`]
	fn finalise(self) -> DatabaseQuery {
		DatabaseQuery {
			query_type: QueryType::Collections,
			conditions: self.conditions,
			return_type: self.return_type
		}
	}

	fn add_condition(&mut self, condition: QueryCondition) {
		self.conditions.push(condition);
	}
}
impl CollectionQuery {
	///Requests objects that were created by the specified user
	pub fn created_by(&mut self, id: u64) -> &mut Self {
		self.add_condition(QueryCondition::CreatedBy(id));
		self
	}
}
///Describes a query about Users
pub struct UserQuery {
	return_type: ReturnType,
	conditions: Vec<QueryCondition>
}
impl QueryBuilder for UserQuery {
	///Begins a query that returns the quantity of matches
	fn count() -> Self {
		Self {
			return_type: ReturnType::Quantity,
			conditions: Vec::new()
		}
	}

	///Begins a query that returns the objects that match the query
	fn all() -> Self {
		Self {
			return_type: ReturnType::Objects,
			conditions: Vec::new()
		}
	}

	///Completes the query and converts it into a generic [`DatabaseQuery`]
	fn finalise(self) -> DatabaseQuery {
		DatabaseQuery {
			query_type: QueryType::Users,
			conditions: self.conditions,
			return_type: self.return_type
		}
	}

	fn add_condition(&mut self, condition: QueryCondition) {
		self.conditions.push(condition);
	}
}

///Describes each table, or more generically data type, a query may operate on
pub enum QueryType {
	Tags,
	Media,
	Groups,
	Collections,
	Users
}

///Describes each of the operators that can be found within a query
pub enum QueryCondition {
	///Requests the next condition to fail
	Not,
	///Requests objects with the specified ID
	ID(u64),
	///Requests objects with a name that perfectly matches
	NameIs(String),
	///Requests objects with a name that contains
	NameContains(String),
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
	///Requests the media that are within the specified collection
	IsInCollection(u64),
	///Requests the objects that were created by the specified user
	CreatedBy(u64),
	///Requests the users that have the specified permissions
	HasPermissions(u64),
}
///Describes each format a query may wish to receive the data back as
pub enum ReturnType {
	///The query requests the amount of items that match
	Quantity,
	///The query requests the items as memory objects
	Objects
}

///An error related to the query
pub enum QueryError {
	///The query used an operator that was not supported by the database
	Unsupported(QueryCondition),
	///The user is not authorised to request this data
	Unauthorised,
	///The query requests conditions that can never match
	ImpossibleQuery,
	///The query requests a condition that does not apply to the operating type
	BadCondition(QueryCondition),
	///The query used a modifier and did not specify an applicable condition afterwards
	BadModifier,
	///The query was not complete
	Incomplete,
	///The database terminated the connection
	ConnectionFault(u16),
	///The database did not respond in time
	ConnectionTimeout,
	///The adapter did not understand the response from the server
	BadResponse
}
///Describes the 3 states a property from a database can be in
pub enum DatabaseValue<T> {
	///A valid value was assigned to the property
	Some(T),
	///No value was assigned to the property
	None,
	///The user is not authorised to know the value of this property
	Unauthorised
}
